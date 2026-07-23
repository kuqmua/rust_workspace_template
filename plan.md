# Что нужно построить

Чтобы в production по одной ошибке понять:

* какой микросервис упал;
* из какого репозитория он собран;
* какой commit был развёрнут;
* в каком файле, строке и функции возникла проблема;
* при каком HTTP-запросе, сообщении очереди или фоновой задаче;
* через какие микросервисы прошёл запрос;
* в каком Tokio task и OS thread выполнялся код;
* какая цепочка ошибок привела к отказу,

недостаточно просто включить `RUST_BACKTRACE=1`.

Нужна совокупность из пяти слоёв:

1. **Структурированные логи** через `tracing`.
2. **Distributed tracing** через OpenTelemetry.
3. **Backtrace и SpanTrace** для конкретной ошибки.
4. **Метаданные сборки:** сервис, репозиторий, commit, image digest.
5. **Error tracker и debug symbols:** например Sentry.

Метрики обнаруживают проблему, логи описывают локальное событие, distributed trace показывает путь между сервисами, а error tracker и debug symbols превращают машинные адреса в файл и строку.

---

# 1. Какие поля должны присутствовать в каждой ошибке

Минимальная production-запись должна выглядеть примерно так:

```json
{
  "timestamp": "2026-07-23T11:15:42.124Z",
  "level": "ERROR",

  "service_name": "orders-service",
  "service_namespace": "backend",
  "service_instance_id": "orders-service-7d944ddf9b-x8p2k",

  "repository": "git.example.com/company/orders-service",
  "git_commit_sha": "aa91b3d26580f93f...",
  "image_digest": "sha256:a9b1...",
  "environment": "production",

  "message": "failed to create order",
  "error_type": "sqlx::Error",
  "error_code": "database_unavailable",
  "error_chain": [
    "failed to create order",
    "failed to insert order",
    "connection pool timed out"
  ],

  "code_file": "src/application/create_order.rs",
  "code_line": 137,
  "code_module": "orders::application::create_order",
  "code_function": "create_order",

  "trace_id": "a5f47dd7286714a7d9902e1ce0490e11",
  "span_id": "3b2874e91bfc9114",
  "request_id": "019c89c2-4dd8-7dc1-9bb1-4381e67dd32e",

  "http_method": "POST",
  "http_route": "/api/orders",
  "http_status": 503,

  "customer_id": "81263",
  "order_id": "128901",
  "retry_attempt": 3,

  "thread_id": "ThreadId(7)",
  "thread_name": "tokio-runtime-worker",
  "tokio_task_id": "42",

  "backtrace": "...",
  "span_trace": "..."
}
```

При этом `service.name`, `service.version`, `service.instance.id` и другие характеристики процесса удобно задавать как OpenTelemetry Resource Attributes. `service.name` необходимо задавать явно, иначе SDK может использовать `unknown_service`. Для репозитория и commit существуют VCS-атрибуты, включая URL репозитория и revision текущего ref. ([OpenTelemetry][1])

---

# 2. Структурированные JSON-логи через `tracing`

В Rust для production лучше использовать не обычные строки через `println!` и даже не только фасад `log`, а `tracing`.

`tracing-subscriber` умеет автоматически добавлять к событиям:

* файл;
* номер строки;
* target, обычно соответствующий module path;
* ID и имя OS thread;
* текущий span;
* полную цепочку активных span;
* поля всех родительских span.

В JSON-режиме span-контекст может прикладываться к каждому событию. ([Docs.rs][2])

Пример базовой настройки:

```rust
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(
            "info,\
             tower_http=info,\
             hyper=warn,\
             sqlx=warn"
        )
    });

    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(ErrorLayer::default())
        .with(json_layer)
        .init();
}
```

В production JSON обычно пишется в `stdout`, а уже Docker, Kubernetes, systemd, Fluent Bit, Vector или OpenTelemetry Collector отправляет его в Loki, Elasticsearch, OpenSearch или другой backend.

## Важное ограничение

Если написано:

```rust
let user = repository.find_user(id).await?;
```

а ошибка залогирована только уровнем выше:

```rust
tracing::error!(error = %error, "request failed");
```

то стандартные поля `file` и `line` укажут **строку с `error!`**, а не строку с `repository.find_user(...)`.

Поэтому для настоящего места появления ошибки требуются:

* backtrace;
* location, сохранённый при создании ошибки;
* SpanTrace;
* структурированная цепочка `source()`.

---

# 3. Функции и обстоятельства через span

Каждая логически важная операция должна иметь span.

```rust
use tracing::instrument;

#[instrument(
    name = "orders.create",
    skip(state, command),
    fields(
        request_id = %request_id,
        order_id = %command.order_id,
        customer_id = %command.customer_id,
        operation = "create_order"
    )
)]
pub async fn create_order(
    state: &AppState,
    request_id: RequestId,
    command: CreateOrder,
) -> anyhow::Result<Order> {
    let customer = state
        .customers
        .find(command.customer_id)
        .await
        .context("failed to load customer")?;

    let order = state
        .orders
        .insert(command)
        .await
        .context("failed to insert order")?;

    Ok(order)
}
```

В результате ошибка будет находиться внутри логической цепочки:

```text
http.request
└── orders.create
    ├── customers.find
    └── orders.insert
        └── database.query
```

Span должен содержать именно обстоятельства выполнения:

* `request_id`;
* route, а не сырой URL с идентификаторами;
* ID бизнес-объекта;
* тип операции;
* downstream service;
* имя очереди;
* номер попытки;
* timeout;
* shard или partition;
* feature flag;
* tenant;
* результат операции.

Не следует помещать туда:

* пароль;
* access token;
* cookie;
* полный HTTP body;
* SQL-параметры с персональными данными;
* банковские данные;
* произвольный `Debug` всего request.

---

# 4. Backtrace и SpanTrace — это разные вещи

## Backtrace

`std::backtrace::Backtrace` показывает стек OS thread в момент захвата:

```rust
use std::backtrace::Backtrace;

let backtrace = Backtrace::capture();
```

Захват через `Backtrace::capture()` управляется переменными окружения. Для `anyhow` можно использовать:

```bash
RUST_LIB_BACKTRACE=1
```

Для panic и обычных ошибок:

```bash
RUST_BACKTRACE=1
```

Документация `anyhow` отдельно предупреждает, что backtrace относительно дорогой, поэтому его не обязательно захватывать для каждой дешёвой ожидаемой ошибки. ([Docs.rs][3])

## SpanTrace

Для async-кода backtrace часто показывает внутренности Tokio executor вместо логического пути приложения. Future может приостанавливаться, продолжаться на другом worker thread, а обычный стек вызовов между `async fn` уже отсутствует.

`tracing_error::SpanTrace` сохраняет активный `tracing` span и его родителей. Он содержит имена span, поля и location каждого span. Поэтому для async Rust он часто полезнее обычного backtrace. ([Docs.rs][4])

Устанавливается поддержка через:

```rust
tracing_subscriber::registry()
    .with(tracing_error::ErrorLayer::default())
    // ...
    .init();
```

Захват:

```rust
use tracing_error::SpanTrace;

let span_trace = SpanTrace::capture();
```

Production-ошибка высокого уровня должна по возможности иметь **оба объекта**:

```text
Backtrace  → машинный стек выполнения
SpanTrace  → логический путь операции
```

---

# 5. Как сохранить точное место создания ошибки

Для собственных ошибок можно сохранять `Location`, `Backtrace` и `SpanTrace` при создании.

```rust
use std::{
    backtrace::Backtrace,
    panic::Location,
};

use tracing_error::SpanTrace;

#[derive(Debug)]
pub struct CapturedError<E> {
    pub source: E,
    pub location: &'static Location<'static>,
    pub backtrace: Backtrace,
    pub span_trace: SpanTrace,
}

impl<E> CapturedError<E> {
    #[track_caller]
    pub fn capture(source: E) -> Self {
        Self {
            source,
            location: Location::caller(),
            backtrace: Backtrace::capture(),
            span_trace: SpanTrace::capture(),
        }
    }
}
```

Использование:

```rust
let connection = pool
    .acquire()
    .await
    .map_err(CapturedError::capture)?;
```

Теперь у ошибки непосредственно сохранены:

```rust
error.location.file()
error.location.line()
error.location.column()
```

Но есть важное различие:

```rust
.map_err(CapturedError::capture)?
```

покажет строку, где ошибка была обёрнута. Она не может автоматически узнать внутреннюю строку в `sqlx`, `reqwest` или другом crate. Для этого нужен backtrace или location, который предоставила сама библиотека.

---

# 6. Где логировать `Result::Err`

Не надо логировать одну ошибку на каждом уровне.

Плохой вариант:

```text
repository: ERROR database failed
service: ERROR repository failed
handler: ERROR service failed
middleware: ERROR request failed
```

В итоге одна проблема создаёт четыре почти одинаковых сообщения и четыре алерта.

Лучше:

1. На внутренних уровнях добавлять контекст через `source`.
2. На границе операции логировать один полный error event.
3. Там же отправлять ошибку в error tracker.

Границы операции:

* HTTP handler или middleware;
* gRPC method;
* consumer сообщения;
* background job;
* Tokio task supervisor;
* CLI command;
* application startup.

Пример с `anyhow`:

```rust
use anyhow::Context;

pub async fn process_order(id: OrderId) -> anyhow::Result<()> {
    load_order(id)
        .await
        .context("failed to load order")?;

    reserve_stock(id)
        .await
        .context("failed to reserve stock")?;

    Ok(())
}
```

На границе:

```rust
pub async fn handle_request() -> Result<Response, ApiError> {
    let result = process_order(order_id).await;

    if let Err(ref error) = result {
        let error_chain = error
            .chain()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        tracing::error!(
            error = %error,
            error_debug = ?error,
            error_chain = ?error_chain,
            backtrace = %error.backtrace(),
            tokio_task_id = ?tokio::task::try_id(),
            "order processing failed"
        );
    }

    result.map_err(ApiError::from)
}
```

Практическое разделение обычно такое:

* `thiserror` — типизированные domain/infrastructure errors;
* `anyhow` — добавление контекста на application boundary;
* error code — стабильный машинный идентификатор;
* message — описание конкретного случая.

Например:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CreateOrderError {
    #[error("customer does not exist")]
    CustomerNotFound,

    #[error("database is unavailable")]
    DatabaseUnavailable {
        #[source]
        source: sqlx::Error,
    },
}
```

В метрики помещается:

```text
error_code="database_unavailable"
```

а не полный `error.message`.

---

# 7. Panic

Стандартный panic hook получает:

* payload panic;
* location исходного `panic!`;
* возможность вывести backtrace.

Rust позволяет заменить hook через `std::panic::set_hook`; стандартный hook выводит сообщение и, если включено, backtrace. ([doc.rust-lang.org][5])

Пример:

```rust
use std::{
    backtrace::Backtrace,
    panic,
};

pub fn install_panic_hook() {
    let previous_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| {
                info.payload()
                    .downcast_ref::<String>()
                    .map(String::as_str)
            })
            .unwrap_or("non-string panic payload");

        let location = info.location();

        tracing::error!(
            panic_message = message,
            panic_file = location.map(|v| v.file()),
            panic_line = location.map(|v| v.line()),
            panic_column = location.map(|v| v.column()),
            backtrace = %Backtrace::force_capture(),
            tokio_task_id = ?tokio::task::try_id(),
            "process panicked"
        );

        previous_hook(info);
    }));
}
```

Если используется Sentry, его panic integration обычно уже устанавливает соответствующую обработку. Не следует независимо ставить несколько hook без корректного chaining, иначе panic может отправляться дважды.

Для production чаще имеет смысл:

```toml
[profile.release]
panic = "unwind"
```

При `panic = "abort"` процесс немедленно завершается без unwinding. Cargo по умолчанию использует `unwind` и в release-профиле. ([doc.rust-lang.org][6])

`catch_unwind` не должен превращать panic в обычный способ обработки ошибок. Его оправданно использовать на изолированных границах, например вокруг plugin/worker execution, где нужно записать аварийное завершение и остановить повреждённую задачу.

---

# 8. Tokio task важнее, чем thread

В многопоточном Tokio runtime async task может начать выполнение на одном worker thread, после `.await` продолжиться на другом. Поэтому поле:

```text
thread_id=ThreadId(7)
```

не отвечает на вопрос «какая логическая задача выполнялась».

Tokio предоставляет `task::try_id()`, `JoinHandle::id()` и ID внутри `JoinError`. Сам `tokio::spawn` допускает исполнение задачи на текущем или другом thread. ([Docs.rs][7])

Лучше создавать отдельный span для каждой фоновой задачи:

```rust
use std::future::Future;

use tokio::task::JoinHandle;
use tracing::{Instrument, info_span};

pub fn spawn_observed<F, T>(
    task_name: &'static str,
    future: F,
) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let span = info_span!(
        "background_task",
        task_name,
    );

    tokio::spawn(future.instrument(span))
}
```

## Нельзя терять `JoinHandle`

Плохой вариант:

```rust
tokio::spawn(process_queue());
```

Если `process_queue()` вернёт `Err`, ошибка может просто исчезнуть. Если task запаникует, panic hook сработает, но бизнес-результат задачи никто не обработает.

Лучше:

```rust
let handle = spawn_observed("orders_consumer", async move {
    run_consumer().await
});

match handle.await {
    Ok(Ok(())) => {}

    Ok(Err(error)) => {
        tracing::error!(
            error = %error,
            "orders consumer terminated with error"
        );
    }

    Err(join_error) if join_error.is_panic() => {
        tracing::error!(
            task_id = %join_error.id(),
            error = %join_error,
            "orders consumer panicked"
        );
    }

    Err(join_error) => {
        tracing::error!(
            task_id = %join_error.id(),
            error = %join_error,
            "orders consumer was cancelled"
        );
    }
}
```

`JoinError` позволяет отличить panic от cancellation и получить ID завершившейся task. ([Docs.rs][8])

Для постоянных воркеров стоит иметь supervisor, который:

* хранит `JoinHandle`;
* записывает завершение;
* увеличивает `background_task_failures_total`;
* решает, перезапустить worker или завершить весь процесс;
* не позволяет критическому consumer тихо исчезнуть.

---

# 9. Distributed tracing между микросервисами

Одного `request_id` недостаточно, потому что его приходится вручную передавать и логировать в каждом сервисе.

OpenTelemetry использует:

```text
trace_id
span_id
parent_span_id
```

Контекст передаётся между процессами через HTTP/gRPC headers, обычно с использованием W3C Trace Context. Благодаря context propagation spans из разных сервисов объединяются в единый trace. ([OpenTelemetry][9])

Пример:

```text
API Gateway
└── POST /orders
    trace_id=abc123

    Orders Service
    └── create_order
        trace_id=abc123

        Customers Service
        └── GET /customers/81263
            trace_id=abc123

        Inventory Service
        └── reserve_stock
            trace_id=abc123

            PostgreSQL
            └── INSERT reservation
                trace_id=abc123
                ERROR: lock timeout
```

В UI Tempo, Jaeger, Sentry Performance или другого backend будет видно:

* какой сервис вызвал следующий;
* длительность каждого вызова;
* где появился первый error status;
* какие retries выполнялись;
* какой downstream вызвал задержку;
* полный критический путь запроса.

Для Rust обычно используется связка:

```text
tracing
    ↓
tracing-opentelemetry
    ↓
opentelemetry-otlp
    ↓
OpenTelemetry Collector
    ↓
Tempo / Jaeger / vendor backend
```

OpenTelemetry рекомендует в production отправлять telemetry через Collector; OTLP сохраняет модель OpenTelemetry и поддерживается разными backend. ([OpenTelemetry][10])

## Что обязательно делать

На входящем запросе:

1. Извлечь родительский trace context из headers.
2. Создать server span.
3. Сделать его текущим span.

На исходящем запросе:

1. Создать client span.
2. Вставить текущий trace context в headers.
3. Записать peer service и результат вызова.

Это должно работать для:

* HTTP;
* gRPC;
* Kafka;
* RabbitMQ;
* NATS;
* фоновых job;
* cron-задач.

Для сообщений очереди trace context нужно сохранять в message headers.

---

# 10. Commit, repository и container image

Commit не следует вычислять во время запуска контейнера: внутри production image обычно нет `.git`.

Commit должен передаваться во время CI build:

```dockerfile
ARG GIT_SHA
ARG REPOSITORY_URL
ARG BUILD_VERSION

ENV GIT_SHA="${GIT_SHA}"
ENV REPOSITORY_URL="${REPOSITORY_URL}"
ENV BUILD_VERSION="${BUILD_VERSION}"

LABEL org.opencontainers.image.revision="${GIT_SHA}"
LABEL org.opencontainers.image.source="${REPOSITORY_URL}"
LABEL org.opencontainers.image.version="${BUILD_VERSION}"

RUN cargo build --release --locked
```

Build:

```bash
docker build \
  --build-arg GIT_SHA="$CI_COMMIT_SHA" \
  --build-arg REPOSITORY_URL="$CI_REPOSITORY_URL" \
  --build-arg BUILD_VERSION="$CI_COMMIT_TAG" \
  -t registry.example.com/orders-service:"$CI_COMMIT_SHA" \
  .
```

В Rust:

```rust
pub const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const GIT_SHA: &str = match option_env!("GIT_SHA") {
    Some(value) => value,
    None => "unknown",
};

pub const REPOSITORY_URL: &str = match option_env!("REPOSITORY_URL") {
    Some(value) => value,
    None => "unknown",
};
```

На старте сервис должен записать отдельное событие:

```rust
tracing::info!(
    service_name = SERVICE_NAME,
    service_version = SERVICE_VERSION,
    git_sha = GIT_SHA,
    repository_url = REPOSITORY_URL,
    "service started"
);
```

Для OpenTelemetry:

```bash
OTEL_SERVICE_NAME=orders-service

OTEL_RESOURCE_ATTRIBUTES="
service.namespace=backend,
service.version=${GIT_SHA},
service.instance.id=${HOSTNAME},
deployment.environment.name=production,
vcs.ref.head.revision=${GIT_SHA},
vcs.repository.url.full=${REPOSITORY_URL}
"
```

Можно использовать собственные короткие имена атрибутов, но важно, чтобы формат был одинаковым во всех сервисах.

## Самая надёжная идентификация deployment

Храните одновременно:

```text
service.name
git.commit.sha
repository.url
container.image.name
container.image.tag
container.image.digest
service.instance.id
deployment.environment
```

Image tag можно перезаписать. Digest нельзя, поэтому именно digest подтверждает, какой бинарник реально выполнялся.

---

# 11. Debug symbols и строки в release-сборке

Обычный release-профиль Cargo по умолчанию имеет:

```toml
debug = false
```

Без debug information backtrace может содержать только имена функций или адреса, но не точные файлы и строки.

Для минимального размера можно включить только line tables:

```toml
[profile.release]
debug = "line-tables-only"
strip = "none"
panic = "unwind"
```

Cargo прямо указывает, что `line-tables-only` создаёт минимальную debug information, необходимую для filename и line number в backtrace. Полная debug information задаётся через `debug = "full"`. Оптимизация может переставлять и inline-ить код, поэтому debugging оптимизированного бинарника сложнее. ([doc.rust-lang.org][6])

## Два production-варианта

### Простой

Оставить line tables непосредственно в бинарнике:

```toml
debug = "line-tables-only"
strip = "none"
```

Плюсы:

* backtrace сразу содержит строки;
* проще эксплуатация.

Минусы:

* бинарник становится больше.

### Продвинутый

1. Собрать бинарник с полной debug information.
2. Отделить debug symbols.
3. Runtime image содержит stripped binary.
4. Debug artifact загружается в Sentry или artifact storage.
5. Artifact хранится по commit SHA и build ID.

Важно хранить для каждого release:

```text
точный production binary
отдельные debug symbols
Cargo.lock
rustc version
target triple
Git commit
repository URL
container image digest
```

Нельзя взять debug symbols от «почти такого же» commit: адреса уже могут не совпасть.

---

# 12. Sentry как error tracker

OpenTelemetry хорошо показывает путь запроса, но специализированный error tracker удобнее для:

* panic;
* stacktrace;
* группировки одинаковых ошибок;
* уведомлений;
* source context;
* связи ошибки с release;
* breadcrumbs перед ошибкой.

Для каждого сервиса задаётся отдельный release:

```text
orders-service@aa91b3d26580f93f
inventory-service@33af0d272d5ffab7
```

Пример конфигурации:

```rust
let _guard = sentry::init((
    dsn,
    sentry::ClientOptions {
        release: Some(
            format!("{SERVICE_NAME}@{GIT_SHA}").into()
        ),
        environment: Some("production".into()),
        server_name: Some(hostname.into()),
        attach_stacktrace: true,
        send_default_pii: false,
        ..Default::default()
    },
));
```

Sentry Rust SDK может прикладывать stacktrace к сообщениям, задавать server name и не отправляет default PII при выключенном `send_default_pii`. Для отображения исходного кода рядом со stack frames необходимо загрузить debug information/source context. ([Docs.rs][11])

В CI после сборки:

```text
build binary
→ extract/store debug symbols
→ upload symbols to error tracker
→ associate symbols with release
→ deploy exactly that image
```

Тогда в интерфейсе будет не просто:

```text
0x000055ad8371aa91
```

а:

```text
orders_service::application::create_order
src/application/create_order.rs:137
commit aa91b3d26580f93f
```

---

# 13. Sampling: не потерять ошибочный trace

При большом трафике нельзя бесконечно хранить 100% traces.

Но простой head sampling, например 1%, может выбросить запрос до того, как станет известно, что внутри него произошла ошибка.

Для этого используется tail sampling в OpenTelemetry Collector:

* traces с ошибками — сохранять всегда;
* очень медленные traces — сохранять всегда;
* успешные быстрые — сохранять с вероятностью;
* traces нового release — временно сохранять чаще.

Tail sampling принимает решение после получения большей части trace и позволяет всегда сохранять traces, содержащие error. При этом sampler становится stateful-компонентом и требует отдельного мониторинга. ([OpenTelemetry][12])

Пример политики:

```yaml
processors:
  tail_sampling:
    decision_wait: 10s
    policies:
      - name: errors
        type: status_code
        status_code:
          status_codes:
            - ERROR

      - name: slow
        type: latency
        latency:
          threshold_ms: 2000

      - name: normal-traffic
        type: probabilistic
        probabilistic:
          sampling_percentage: 5
```

---

# 14. Метрики и alerting

Логи отвечают на вопрос «что произошло», но обнаруживать проблему лучше через метрики.

Для каждого сервиса нужны как минимум:

```text
http_requests_total
http_request_duration_seconds
application_errors_total
panics_total
background_task_failures_total
dependency_requests_total
dependency_request_duration_seconds
timeouts_total
retries_total
queue_messages_failed_total
dead_letter_messages_total
process_restarts_total
```

Пример alert на долю HTTP 5xx:

```promql
(
  sum by (service) (
    rate(http_requests_total{status_code=~"5.."}[5m])
  )
  /
  sum by (service) (
    rate(http_requests_total[5m])
  )
) > 0.01
```

Alert должен содержать ссылки:

```text
service
environment
dashboard
filtered logs
filtered traces
Sentry issue
current deployment
commit
runbook
```

В labels метрик нельзя помещать:

```text
request_id
user_id
order_id
полный URL
error message
stacktrace
```

Это создаст огромную cardinality.

Используйте стабильные labels:

```text
service
route
method
status_class
error_code
dependency
operation
environment
```

---

# 15. Ошибки, которые Rust-приложение само не успеет записать

## SIGSEGV, SIGABRT, native crash

Для bare-metal/systemd необходимо включить core dumps.

`systemd-coredump` сохраняет core dump и summary события, а `coredumpctl` позволяет получать и анализировать сохранённые dumps. ([FreeDesktop][13])

Типичный процесс:

```bash
coredumpctl list
coredumpctl info <PID>
coredumpctl debug <PID>
```

Для нормальной символизации требуется:

* exact binary;
* exact debug symbols;
* matching build ID;
* exact commit.

## OOMKilled

При OOM процесс обычно не может записать Rust error или panic: его завершает kernel/cgroup.

В Kubernetes нужно смотреть:

```bash
kubectl describe pod <pod>
kubectl get pod <pod> -o yaml
```

Поля:

```text
lastState.terminated.reason
lastState.terminated.exitCode
restartCount
```

Причина `OOMKilled` и restart count показывают, что container превысил memory limit и был перезапущен. ([Kubernetes][14])

Нужны alerts на:

```text
container restart count
OOMKilled
working set memory
memory limit utilization
node memory pressure
```

## SIGKILL

На `SIGKILL` невозможно выполнить panic hook или graceful shutdown. Источник ищется в:

* Kubernetes events;
* systemd journal;
* kernel log;
* orchestrator audit;
* deployment activity.

## Deadlock или зависание

Тут ошибки может вообще не быть.

Нужны:

* readiness probe;
* liveness probe;
* watchdog;
* timeout на операции;
* метрика активных запросов;
* метрика event-loop/task latency;
* thread dump или profiler по требованию;
* alert на отсутствие прогресса consumer.

---

# 16. Рекомендуемая архитектура

Для десятков Rust-микросервисов стоит сделать общий crate:

```text
company-observability
```

Он должен предоставлять:

```rust
observability::init(...)
observability::install_panic_reporting(...)
observability::request_span(...)
observability::spawn_observed(...)
observability::report_error(...)
observability::shutdown(...)
```

Архитектура:

```text
Rust microservice
├── tracing JSON → stdout
├── tracing spans → OpenTelemetry OTLP
├── errors/panics → Sentry
└── metrics → Prometheus endpoint

stdout
└── Fluent Bit / Vector / OTel Collector
    └── Loki / OpenSearch

OTLP
└── OpenTelemetry Collector
    └── Tempo / Jaeger

metrics
└── Prometheus
    └── Alertmanager
        └── Telegram / email / Slack

errors
└── Sentry
    └── stacktrace + source context + release
```

---

# 17. Итоговый порядок расследования

Когда приходит alert, процесс должен выглядеть так:

1. Alert сообщает: `orders-service`, production, error rate 8%.
2. Открывается dashboard и определяется время начала.
3. По deployment metadata видно:

   ```text
   commit=aa91b3d
   image_digest=sha256:a9b1...
   ```
4. Открывается Sentry issue.
5. В issue видно:

   ```text
   src/application/create_order.rs:137
   orders::application::create_order
   ```
6. Из issue берётся `trace_id`.
7. В Tempo открывается весь distributed trace.
8. Видно:

   ```text
   API → Orders → Inventory → PostgreSQL
   ```
9. По span fields видно:

   ```text
   order_id=128901
   retry_attempt=3
   db_operation=insert_reservation
   ```
10. По SpanTrace виден логический путь async-функций.
11. По backtrace виден машинный стек.
12. По commit открывается точная версия файла в репозитории.

## Минимум, который я бы внедрил первым

1. Общий crate с `tracing` JSON.
2. `file`, `line`, `target`, thread ID и span list.
3. `#[instrument]` на application/service/repository boundaries.
4. `trace_id` propagation через OpenTelemetry.
5. `service.name`, repo URL, Git SHA и image digest.
6. `anyhow::Context` и единичное логирование на boundary.
7. `Backtrace + SpanTrace` для неожиданных ошибок.
8. Sentry с release и загруженными debug symbols.
9. Supervisor для всех критических Tokio tasks.
10. Alerts на 5xx, panics, task failures, restarts и OOMKilled.

Именно эта комбинация даёт ответ не только «где был вызван `error!`», а **где ошибка появилась, через какие функции и сервисы прошла, в каком deployment и commit выполнялся код и при каких входных данных это произошло**.

[1]: https://opentelemetry.io/docs/languages/sdk-configuration/general/?utm_source=chatgpt.com "General SDK Configuration"
[2]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Layer.html?utm_source=chatgpt.com "Layer in tracing_subscriber::fmt - Rust"
[3]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html?utm_source=chatgpt.com "Error in anyhow - Rust"
[4]: https://docs.rs/tracing-error/latest/tracing_error/struct.SpanTrace.html?utm_source=chatgpt.com "SpanTrace in tracing_error - Rust"
[5]: https://doc.rust-lang.org/std/panic/fn.set_hook.html?utm_source=chatgpt.com "set_hook in std::panic"
[6]: https://doc.rust-lang.org/cargo/reference/profiles.html "Profiles - The Cargo Book"
[7]: https://docs.rs/tokio/latest/tokio/task/struct.Id.html?utm_source=chatgpt.com "Id in tokio::task - Rust"
[8]: https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html?utm_source=chatgpt.com "JoinError in tokio::task - Rust"
[9]: https://opentelemetry.io/docs/concepts/context-propagation/?utm_source=chatgpt.com "Context propagation"
[10]: https://opentelemetry.io/docs/languages/rust/exporters/ "Exporters | OpenTelemetry"
[11]: https://docs.rs/sentry/latest/sentry/struct.ClientOptions.html?utm_source=chatgpt.com "ClientOptions in sentry - Rust"
[12]: https://opentelemetry.io/docs/concepts/sampling/?utm_source=chatgpt.com "Sampling"
[13]: https://www.freedesktop.org/software/systemd/man/systemd-coredump.html?utm_source=chatgpt.com "systemd-coredump"
[14]: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/?utm_source=chatgpt.com "Resource Management for Pods and Containers"
