# Дополнительные кандидаты на перенос из `mapcam_rust`

## Область анализа

Документ дополняет `MAPCAM_RUST_REUSE_CANDIDATES.md`, а не повторяет его 24 уже рассмотренных направления. Сравнение выполнено 2026-07-14 между:

- `/home/sergey/projects/mapcam_rust`;
- `/home/sergey/projects/rust_workspace_template`.

Рекомендации основаны на текущем коде и конфигурации обоих репозиториев. Под «переносом» ниже понимается адаптация небольшого механизма под архитектуру шаблона, а не копирование Mapcam-доменов или всего `shared` crate. Новые зависимости в рамках этого анализа не добавляются.

## Краткий приоритет

| Приоритет | Кандидат | Куда переносить | Польза |
|---|---|---|---|
| P0 | Контроль владения `spawn`-задачами | `tests` и `server_runtime` | Не допускать молча отсоединённых фоновых задач |
| P0 | Инвентаризация прямого доступа к окружению и файловой системе | `tests`, `config_lib`, `server_config` | Сосредоточить ввод-вывод и конфигурацию на явных границах |
| P0 | Дополнительные запреты для аварийного завершения и обхода типов | `tests/src/code_style` | Не допускать `abort`, `transmute` и скрытого подавления ошибок |
| P1 | Доверенная обработка IP клиента за reverse proxy | `server_runtime` | Не доверять подделанным forwarding-заголовкам |
| P1 | Единый запуск внешних процессов | существующий shared crate для tooling | Централизовать завершение, логирование и очистку дочерних процессов |
| P1 | Артефакты и сводки `workspace_test_runner` | `workspace_test_runner` | Упростить диагностику параллельных и CI-запусков |
| P2 | Политика детерминированных тестов | `tests/src/code_style` | Отсеивать случайность, sleeps и зависимость от wall clock |
| P2 | Репозиторные конфиги `typos`, Taplo и Clippy MSRV | корень workspace и CI | Ловить опечатки и дрейф TOML отдельными дешёвыми проверками |
| P2 | Опциональный контейнерный эталон запуска | корень workspace | Дать шаблону воспроизводимый production-like пример |

## 1. Контроль владения `spawn`-задачами

Статус: **выполнено**.

Реализован AST-тест `spawned_tasks_must_retain_an_owner`: прямой отброшенный результат `tokio::spawn`, `tokio::task::spawn_blocking` или `std::thread::spawn` запрещён. Существующие runtime-вызовы сохраняют handles в `BackgroundTask` либо явно ожидают их.

### Источник

- `mapcam_rust/tests/src/tests/code_style.rs`, тест `enforces_spawn_ownership_inventory`.
- `mapcam_rust/shared/src/server.rs`, где долгоживущие задачи представлены владельцами и участвуют в shutdown.

### Что перенести

Добавить AST-проверку, которая инвентаризирует `tokio::spawn`, `tokio::task::spawn_blocking` и `std::thread::spawn`. Разрешать вызов только там, где результат:

1. сохраняется в типе-владельце;
2. явно ожидается;
3. передаётся существующему `server_runtime::BackgroundTask`;
4. находится в узком тестовом allowlist с объяснением.

В шаблоне уже есть supervised background task, поэтому новый runtime abstraction не нужен. Полезен именно автоматический контроль обходных мест.

### Критерии готовности

- потерянный `JoinHandle` приводит к понятной ошибке code-style теста;
- тест различает production, proc-macro и test-only код;
- каждое исключение содержит путь, ожидаемое число вхождений и причину;
- проверка не считает обычный вызов метода с именем `spawn` вызовом runtime API.

## 2. Инвентаризация прямого доступа к окружению и файловой системе

Статус: **выполнено**.

AST-тест `direct_environment_and_filesystem_access_stays_at_owned_boundaries` ограничивает прямой доступ configuration, tooling, persistence и test-fixture границами. Новое обращение из бизнес-логики приводит к ошибке code-style suite.

### Источник

- `mapcam_rust/tests/src/tests/code_style.rs`, тест `enforces_direct_environment_and_filesystem_access_inventory`.
- `mapcam_rust/shared/src/startup.rs`: единая загрузка `.env`, обязательных значений и положительных числовых параметров с типизированными ошибками.

### Что перенести

Сначала добавить не запрет, а инвентаризационный тест для `std::env::*`, `std::fs::*`, `tokio::fs::*` и прямого чтения текущей директории. После фиксации baseline постепенно оставить доступ только в:

- `config_lib` и `server_config` для конфигурации;
- entrypoint и startup-коде для выбора файлов;
- явно обозначенных persistence/file-storage адаптерах;
- тестовых fixtures.

Не следует копировать универсальный Mapcam trait ошибок дословно. В шаблоне уже есть собственные config crates; им полезнее перенять разделение `missing`, `invalid Unicode`, `parse` и `not positive` без расширения публичного API.

### Критерии готовности

- бизнес-логика получает готовые domain wrappers, а не имена env-переменных;
- новые прямые обращения ломают policy test;
- ошибки не содержат значения секретов;
- `.env` и `.env.example` продолжают проверяться существующим тестом на одинаковые ключи.

## 3. Запрет аварийного завершения и обхода системы типов

Статус: **выполнено**.

AST-тест запрещает `transmute` и фиксирует два оставшихся compile-time/generated-default исключения `abort` точными путями. Безусловный `abort` из создания idempotency key удалён; рост baseline запрещён.

### Источник

В `mapcam_rust/tests/src/tests/code_style.rs` реализованы отдельные проверки:

- `forbids_process_abort_call_in_all_non_policy_rust_sources`;
- `forbids_transmute_call_in_non_test_code`;
- `forbids_source_dropping_map_err_pattern`;
- `forbids_unwrap_and_error_masking_shortcuts_in_non_test_code`.

### Текущий разрыв

В шаблоне запрещён `unsafe`, а Clippy запрещает многие варианты `transmute`, но source-policy тест не фиксирует простое архитектурное правило целиком. Кроме того, в workspace есть несколько прямых `std::process::abort()` в macro/runtime-related коде. Поэтому немедленный абсолютный запрет сначала упадёт на существующем baseline.

### Что перенести

Добавлять правила поэтапно:

1. AST-инвентарь `abort` и `transmute` с точными известными местами.
2. Запрет новых вхождений.
3. Замена существующих `abort` на типизированное распространение ошибки там, где это возможно без изменения proc-macro контракта.
4. Проверка конструкций, которые вычисляют `Result`, а затем превращают ошибку в `Option` или значение по умолчанию без документированной политики.

### Критерии готовности

- тест работает по AST, а не по подстрокам и комментариям;
- proc-macro исключения отделены от runtime-кода;
- baseline не допускает увеличения числа исключений;
- после миграции allowlist сокращается, а не превращается в постоянный общий обход.

## 4. Доверенная обработка IP клиента за reverse proxy

Статус: **выполнено**.

В `server_runtime::client_ip` добавлены typed CIDR ranges и resolver. Заголовки принимаются только от trusted peer, `X-Forwarded-For` обходится справа налево, а multiple, malformed, oversized и смешанные IPv4/IPv6 случаи покрыты тестами.

### Источник

- `mapcam_rust/shared/src/client_ip.rs`, функции `classify_trusted_proxy_membership` и `resolve_client_ip_address_string_with_trusted_proxy_ranges`.
- Тесты в том же файле покрывают IPv4, IPv6, несовпадающие семейства адресов, цепочки `X-Forwarded-For`, malformed и oversized заголовки.

### Что перенести

Добавить в `server_runtime` небольшой тип конфигурации trusted proxy CIDR и resolver со следующим правилом: forwarding-заголовки учитываются только тогда, когда непосредственный peer входит в доверенную сеть. Цепочка должна обходиться справа налево до первого недоверенного адреса и иметь жёсткий предел числа элементов и размера заголовка.

Это стоит внедрять перед использованием IP для rate limiting, аудита или security decisions. Простое чтение первого `X-Forwarded-For` небезопасно.

### Критерии готовности

- без trusted proxy возвращается socket peer;
- заголовок от недоверенного peer игнорируется;
- IPv4 и IPv6 CIDR проверяются отдельно;
- malformed, multiple-header и oversized случаи имеют детерминированный fallback;
- публичная ошибка и логи не отражают произвольный заголовок целиком.

## 5. Единый запуск внешних процессов

Статус: **выполнено**.

Все прямые `std::process::Command::new` перенесены в `macros_helpers::tool_command`; отдельный AST-тест запрещает обход helper. Аргументы скрыты из `Debug`, а generator tooling, lint sync и runner используют один typed builder.

### Источник

- `mapcam_rust/tests/src/tests/code_style.rs`, тест `forbids_direct_command_new_usage`.
- `mapcam_rust/workspace_test_runner/src/main.rs`: `spawn_test_command`, `terminate_running_test_commands`, ожидание процессов и сбор stdout/stderr.

### Что перенести

В шаблоне `std::process::Command::new` вызывается в generator tooling, lint helpers и test runner. После появления хотя бы двух одинаковых требований стоит выделить внутренний helper в существующий shared tooling crate. Он должен отвечать за:

- явный program/args contract;
- статус и типизированную ошибку запуска;
- ограниченный диагностический вывод;
- kill-and-wait при раннем завершении;
- отсутствие shell interpolation по умолчанию;
- redaction секретных аргументов.

После этого source-policy тест должен запрещать прямой `Command::new` вне helper и узкого platform-specific allowlist.

### Критерии готовности

- неуспешный spawn, signal termination и non-zero exit различаются;
- при fail-fast дочерние процессы завершаются и reap-ятся;
- секретные аргументы не попадают в `Debug`, summary или CI log;
- generator tooling сохраняет текущую семантику вызова `rustfmt`.

## 6. Артефакты и сводки `workspace_test_runner`

Статус: **выполнено**.

Static/database команды создают уникальный run directory, отдельный raw log и ANSI-free `summary.txt` со статусом, длительностью и именами упавших тестов. Парсер Cargo/nextest и partial logs покрыт unit tests.

### Источник

`mapcam_rust/workspace_test_runner/src/main.rs` содержит:

- `TestCommandRunReport` и `RunningTestCommand`;
- уникальный каталог каждого запуска;
- отдельный лог каждой команды;
- `write_summary_file` и aggregate summary;
- `parse_failed_test_names`;
- завершение остальных процессов после критического сбоя.

### Что перенести

Текущий runner шаблона хорошо покрывает memory-allocation workloads, но для общего `all/static/database` запуска полезно адаптировать файловый отчёт Mapcam:

```text
test_results/workspace_test_runner/<run-id>/
  summary.txt
  cargo-clippy.log
  code-style.log
  database.log
```

Не нужно копировать Mapcam-список сервисов. Следует перенести только runner model, bounded log naming, exit aggregation и извлечение имён упавших тестов.

### Критерии готовности

- итоговый exit code остаётся ненулевым при любом обязательном сбое;
- summary показывает команду, длительность, статус и путь к логу;
- параллельные запуски не перезаписывают артефакты;
- ANSI удаляется из машинно читаемой сводки, но при необходимости сохраняется в raw log;
- parser покрыт тестами для стандартного Cargo output и повреждённого/неполного лога.

## 7. Политика детерминированных тестов

Статус: **выполнено**.

AST-тест инвентаризирует test-only wall clock, sleeps и entropy-based UUID. Два существующих детерминированных применения зафиксированы точными путями: paused-time timeout test и уникальный UUID fixture, не участвующий в assertion.

### Источник

- `mapcam_rust/tests/src/tests/code_style.rs`, тест `enforces_deterministic_test_patterns`.

### Что перенести

Сделать AST-инвентарь опасных test-only паттернов:

- `thread_rng`, entropy-based UUID и случайный порядок без фиксированного seed;
- `SystemTime::now` и `Utc::now` в assertions;
- `sleep` как средство синхронизации;
- временные пути с предсказуемым общим именем;
- зависимость от порядка итерации `HashMap`/`HashSet` в golden output.

Проверка должна поддерживать узкие исключения: уникальное имя временного ресурса может быть случайным, если значение не участвует в assertion и cleanup гарантирован владельцем.

### Критерии готовности

- Tokio time tests используют paused time или явные timeout boundaries;
- property-like тесты имеют фиксированный seed либо перебирают ограниченное пространство;
- сообщения policy test объясняют безопасную замену;
- инвентарь не запрещает криптографическую случайность в production-коде.

## 8. Конфиги `typos`, Taplo и Clippy MSRV

Статус: **выполнено**.

Добавлены `.typos.toml`, `taplo.toml` и закреплённые CI jobs для `typos`, `taplo fmt --check` и `taplo lint`. `clippy.toml` намеренно не создан: workspace поддерживает latest nightly и не объявляет MSRV.

### Источник

- `mapcam_rust/.typos.toml` задаёт исключения generated/vendor файлов и словарь доменных слов;
- `mapcam_rust/taplo.toml` исключает `target/**` из форматирования TOML;
- `mapcam_rust/clippy.toml` фиксирует MSRV.

### Что перенести

Создать локальные конфиги с минимальными исключениями шаблона и добавить отдельные CI-команды:

- `typos` для исходников, Markdown и конфигурации;
- `taplo fmt --check` для TOML;
- `taplo lint` для структуры TOML;
- `clippy.toml` только если проект действительно поддерживает конкретный MSRV.

Не копировать словарь Mapcam: слова `buscheb`, `wheres` и другие доменные исключения здесь будут скрывать реальные опечатки. Не объявлять MSRV равным Mapcam автоматически, поскольку шаблон ориентирован на latest nightly.

### Критерии готовности

- generated, vendor и build-output каталоги исключены явно;
- allowlist опечаток содержит только слова, реально встречающиеся в шаблоне;
- локальная команда совпадает с CI-командой;
- версии CI tools и actions закреплены в соответствии с существующей release policy.

## 9. Опциональный контейнерный эталон запуска

Статус: **выполнено**.

Добавлены multi-stage `Dockerfile`, `.dockerignore` и минимальный `docker-compose.yml` с PostgreSQL healthcheck, readiness server, non-root user, read-only root filesystem, tmpfs и loopback-only published ports. Пароль обязателен через environment и не имеет встроенного default.

### Источник

- `mapcam_rust/Dockerfile`: multi-stage release build, `--locked`, отдельный непривилегированный пользователь и минимальный runtime image.
- `mapcam_rust/docker-compose.yml`: healthchecks, `depends_on: condition: service_healthy`, loopback-only published ports, named volumes и общая сеть.

### Что перенести

После определения целевого бинарника добавить один минимальный example profile, а не весь Mapcam compose:

1. PostgreSQL с healthcheck и loopback binding.
2. Один server binary из шаблона.
3. Multi-stage build с `cargo build --locked --release`.
4. Non-root runtime user.
5. Read-only root filesystem и writable volume только там, где он нужен.
6. Явный health/readiness endpoint.

Mapcam Dockerfile нельзя копировать буквально: он содержит FFmpeg, статические каталоги и migrations конкретных сервисов. Также не следует переносить example passwords как production defaults.

### Критерии готовности

- образ запускается без root;
- в final image нет Cargo registry, compiler и исходников;
- секреты не встроены в image layers;
- Compose ждёт readiness базы до запуска миграций/server;
- shutdown посылает сигнал, который обрабатывает существующий `server_runtime`.

## Что пока не переносить

1. Весь `shared` crate Mapcam: он создаст скрытую связь с GIS/video/auth доменами.
2. Полный список из десятков новых style-политик одним изменением: сначала нужен baseline и миграция существующих нарушений.
3. Mapcam `str_constants`: большие доменные каталоги не относятся к workspace template.
4. Конкретный Docker Compose со всеми Mapcam сервисами, MediaMTX, Prometheus и FFmpeg.
5. `proptest`, `trybuild`, `typos` или Taplo как новые зависимости/инструменты без отдельного решения о поддержке и CI ownership.
6. Запрет всех публичных полей: у шаблона уже есть generated/API contracts, где публичные поля могут быть сознательной частью wire API.

## Рекомендуемый порядок внедрения

1. Добавить read-only inventories для `spawn`, env/filesystem, `abort`, `transmute` и `Command::new`.
2. Зафиксировать baseline с точными путями, количеством и объяснением каждого исключения.
3. Подключить spawn ownership к существующему `BackgroundTask` и централизовать command execution.
4. Перенести trusted-proxy resolver до использования IP в security-sensitive логике.
5. Расширить `workspace_test_runner` каталогами запусков и сводками.
6. Уменьшать inventories до строгих запретов отдельными небольшими изменениями.
7. Подключить репозиторные форматтеры/линтеры и контейнерный example только после определения их CI ownership.

Каждый пункт следует внедрять отдельным изменением. Первым acceptance gate для policy-тестов должен быть запрет роста baseline, а конечным — удаление исключений там, где архитектура уже предоставляет безопасный общий путь.
