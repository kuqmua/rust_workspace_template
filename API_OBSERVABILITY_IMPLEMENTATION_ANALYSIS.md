# Анализ внедрения observability в API согласно `plan.md`

`plan.md` описывает не новые бизнес-маршруты API, а production-наблюдаемость: как связать HTTP-запрос, ошибку, микросервис, deployment и конкретный commit.

## Что уже реализовано

- Структурированные логи через `tracing`, включая JSON-режим.
- Общий инфраструктурный crate `server_runtime`.
- HTTP span `http.request`.
- Генерация и возврат `x-request-id`.
- Логирование метода, пути, статуса и длительности запроса.
- Prometheus-метрики HTTP:
  - количество запросов;
  - длительность;
  - количество ответов 5xx.
- Нормализация путей для metric labels.
- `/metrics`, health/readiness и информация о Git commit.
- Типы для `traceparent` и `tracestate`.
- Передача trace-заголовков в исходящий `reqwest`-запрос.
- Release workflow получает immutable image digest после публикации.

Основная HTTP-инфраструктура находится в `server_runtime/src/lib.rs`, метрики — в `server_runtime/src/metrics_layer.rs`.

## Что обязательно внедрить первым

### 1. Полноценный OpenTelemetry вместо простой передачи заголовков

Сейчас `traceparent` валидируется и может быть передан дальше, но это ещё не distributed tracing:

- входящий контекст не становится родителем `tracing` span;
- новые `trace_id` и `span_id` не создаются через OpenTelemetry;
- spans не экспортируются по OTLP;
- нет связи между server span и downstream client span.

Нужно добавить в общий runtime:

- извлечение W3C trace context на входе;
- создание server span;
- создание client span для исходящих запросов;
- автоматическую инъекцию контекста;
- OTLP exporter;
- корректный graceful shutdown tracer provider.

Это наиболее важный пробел.

### 2. Исправить HTTP span

Сейчас в span попадает сырой `req.uri().path()`:

```rust
path = %req.uri().path()
```

Для запроса `/api/v1/users/123` это приводит к высокой cardinality и потенциально раскрывает идентификаторы.

Нужно записывать:

```text
http.route=/api/v1/users/{user_id}
url.path=/api/v1/users/123
```

`url.path` следует добавлять только тогда, когда это действительно требуется и безопасно. Основным полем должен быть `MatchedPath`, а не сырой URL.

Также следует добавить:

- `http.request.method`;
- `http.response.status_code`;
- `error.type`;
- `error_code`;
- `server.address`;
- `client.address` с учётом доверия к proxy;
- `trace_id` и `span_id`;
- `service.name`.

### 3. Единое логирование ошибок на HTTP boundary

Сейчас middleware всегда пишет информационное событие о завершении запроса. Детальная ошибка логируется только в отдельных местах, например в notification service.

Нужна единая граница, которая при `5xx` пишет ровно одно структурированное событие:

```text
request_id
trace_id
service_name
http_route
http_method
http_status
error_code
error_type
error_chain
backtrace
span_trace
```

Middleware не сможет восстановить цепочку ошибки только из готового HTTP response. Поэтому тип API-ошибки должен сохранять диагностический объект до момента преобразования в response.

Ожидаемые ответы `4xx` обычно не нужно логировать как `ERROR`.

### 4. Общий тип диагностической ошибки

Стоит разместить в shared crate обёртку примерно следующего назначения:

```text
ObservedError<E>
├── source
├── error_code
├── Location
├── Backtrace
└── SpanTrace
```

Она должна захватываться при переходе инфраструктурной ошибки в application error, а не для каждой обычной ошибки валидации.

Важно сохранить существующие типизированные `thiserror` enums. Не стоит превращать весь workspace в `anyhow`; он уместнее на границах запуска, фоновой задачи или запроса.

### 5. Расширить JSON tracing

Существующая инициализация включает `.json()`, но явно не настраивает требуемые планом поля.

Нужно централизовать обе реализации tracing initialization и включить:

- target/module;
- file;
- line;
- thread ID;
- thread name;
- current span;
- span list;
- `tracing_error::ErrorLayer`.

Сейчас инициализация дублируется в:

- `server/src/main.rs`;
- `server_runtime/src/service_bootstrap.rs`.

Её следует оставить только в общем crate.

## Критическая проблема release-сборки

Текущий профиль содержит:

```toml
panic = "abort"
strip = "symbols"
```

Это конфликтует с целью `plan.md`:

- при `panic = "abort"` нет нормального unwind;
- `strip = "symbols"` ухудшает символизацию;
- точные строки release-backtrace получить сложнее.

Для предложенного в плане простого варианта нужны примерно такие настройки:

```toml
debug = "line-tables-only"
strip = "none"
panic = "unwind"
```

В качестве альтернативы symbols нужно отделять и сохранять как CI artifact, а затем загружать в error tracker.

## Panic и фоновые Tokio tasks

Существующий `panic_location` предназначен главным образом для proc-macro crates и выводит location через `eprintln!`. Он не является production panic reporting для серверов.

Нужно реализовать:

- общий panic hook с chaining предыдущего hook;
- structured event с panic location, task ID и backtrace;
- счётчик `panics_total`;
- отправку panic в Sentry, если Sentry будет выбран.

Для фоновых задач нужен `spawn_observed` или supervisor:

- span с `task_name`;
- обязательное хранение `JoinHandle`;
- различение `Err`, panic и cancellation;
- `background_task_failures_total`;
- политика restart/fail-fast для каждой критической задачи.

Особенно это касается фоновой очистки административных таблиц.

## Build metadata

Git commit уже встраивается через `git_version`, но этого недостаточно.

На старте каждого сервиса следует логировать и добавлять в OpenTelemetry Resource:

```text
service.name
service.version
service.instance.id
repository.url
git.commit.sha
container.image.name
container.image.tag
container.image.digest
deployment.environment
```

CI уже вычисляет digest, но приложение его не получает.

В workflow нужно:

1. Передавать commit, repository и version во время build.
2. Добавлять OCI labels.
3. Передавать фактический digest deployment-средой.
4. Писать startup event со всеми метаданными.

Отдельный публичный endpoint для этих данных необязателен. Существующий Git endpoint можно расширить только безопасными данными; image digest и instance ID лучше держать в telemetry и защищённом admin endpoint.

## Метрики, которых не хватает

HTTP-метрики уже есть, но согласно плану нужны также:

```text
application_errors_total{error_code}
panics_total
background_task_failures_total{task_name}
dependency_requests_total{dependency,operation,result}
dependency_request_duration_seconds
timeouts_total{operation}
retries_total{operation}
```

Для notification service следует подключить тот же `HttpMetricsLayer`: сейчас там есть `/metrics`, но HTTP middleware метрик не применяется.

Нельзя добавлять в labels:

- `request_id`;
- user ID;
- raw URL;
- сообщение ошибки;
- stacktrace.

## Sentry и инфраструктура

Это следующий этап после базового tracing:

- Sentry release вида `server@<git_sha>`;
- `send_default_pii=false`;
- debug symbols/source context из CI;
- связь Sentry event с `trace_id`;
- OpenTelemetry Collector;
- Tempo или Jaeger;
- tail sampling с сохранением всех ошибочных и медленных traces;
- alerts на 5xx, panic, failed tasks, restarts и `OOMKilled`.

Это потребует новых зависимостей. По правилам репозитория их можно добавлять только после явного запроса на реализацию.

## Рекомендуемый порядок

1. Перенести всю инициализацию observability в `server_runtime`.
2. Улучшить HTTP span и заменить raw path на route template.
3. Сделать единый error boundary и стабильные `error_code`.
4. Добавить `ErrorLayer`, `SpanTrace`, location и выборочный backtrace.
5. Внедрить OpenTelemetry/OTLP и реальную propagation.
6. Добавить resource/build/deployment metadata.
7. Реализовать supervisor фоновых задач.
8. Исправить release symbols и panic strategy.
9. Добавить недостающие метрики.
10. Подключить Sentry, Collector, sampling и alerts.

## Итог

Фундамент observability уже частично реализован. Основные отсутствующие элементы:

- настоящий OpenTelemetry;
- сохранение диагностического контекста ошибки;
- централизованный error boundary;
- production panic/task reporting;
- символизация release-бинарников.

Новые пользовательские API endpoints для выполнения `plan.md` практически не требуются. Основная работа должна выполняться в общем runtime, HTTP middleware, типах ошибок, CI/CD и инфраструктуре наблюдаемости.
