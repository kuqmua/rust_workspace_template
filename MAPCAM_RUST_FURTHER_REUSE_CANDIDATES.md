# Дополнительные кандидаты для переноса из `mapcam_rust`

Дата анализа: 2026-07-14.

Источник: `/home/sergey/projects/mapcam_rust`.

Цель документа — перечислить полезные решения, которых ещё нет в
`rust_workspace_template` либо которые представлены здесь только частично. Это
не инструкция копировать файлы буквально: MapCam-специфичные имена, зависимости
и бизнес-правила следует удалить, а перенос выполнять небольшими независимыми
изменениями.

## Краткий приоритет

| Приоритет | Кандидат | Польза | Оценка трудоёмкости |
| --- | --- | --- | --- |
| P0 | Ограниченное чтение файлов и HTTP-ответов | Защита от неограниченного потребления памяти | Средняя |
| P0 | Полный набор проверок CI | Поиск неиспользуемых зависимостей, ошибок workflow и уязвимостей образа | Низкая–средняя |
| P1 | Compile-fail контракты маршрутов | Ошибки связи route/request/response обнаруживаются компилятором | Высокая |
| P1 | Runtime-сверка OpenAPI с ответами сервера | Документация не расходится с фактическим API | Средняя |
| P1 | Расширенный режим `nextest` в test runner | Быстрее и понятнее полный прогон большого workspace | Средняя |
| P1 | Безопасный инициализатор `.env` | Воспроизводимый запуск всех сервисов разработчиком | Средняя |
| P2 | Типизированные SQL-идентификаторы и policy-тесты | Меньше сырых имён таблиц и схем в SQL | Средняя–высокая |
| P2 | Контроль регрессий производительности | Замечает ухудшения алгоритмов до релиза | Средняя |

## 1. Ограниченное чтение файлов и HTTP-ответов

**Приоритет: P0.**

**Статус: выполнено.**

### Что взять за основу

- `shared/src/text_file.rs`:
  - `BoundedReadError`;
  - проверку размера по metadata до чтения;
  - повторную проверку фактического количества байтов;
  - ограниченное потоковое чтение HTTP body;
  - отдельное преобразование UTF-8 и JSON;
  - semaphore permit для ограничения одновременных больших чтений.

### Почему это полезно шаблону

В шаблоне есть ограничение размера входящего HTTP body, но нет общего API для
ограниченного чтения файлов и исходящих HTTP-ответов. Сейчас внутренние утилиты
используют `std::fs::read_to_string`, а будущий сервис легко может вызвать
`response.text()` или `response.bytes()` без верхней границы.

### Как переносить

Добавить небольшой модуль в подходящий shared crate. Начать только с `std`-части:
ограниченное синхронное и асинхронное чтение файла. HTTP-вариант добавлять, когда
появится реальный потребитель, чтобы не вводить зависимость без запроса.

### Критерии готовности

- размер проверяется до выделения основного буфера и после чтения;
- превышение лимита имеет отдельный тип ошибки;
- некорректный UTF-8 не преобразуется с потерями;
- тесты покрывают точную границу, превышение на один байт, изменение файла между
  metadata и чтением и ошибку UTF-8;
- прямое неограниченное чтение runtime-данных запрещено policy-тестом, но
  test fixtures и инструменты имеют явный allowlist.

## 2. Дополнить CI проверками из MapCam

**Приоритет: P0.**

**Статус: выполнено.**

### Чего сейчас не хватает

В текущем CI уже есть format, Clippy, docs, cargo-deny, cargo-audit,
cargo-hack, semver-checks, gitleaks, Taplo, Typos и scheduled `cargo-udeps`.
Из `.github/workflows/ci.yml` MapCam ещё полезны:

- `actionlint` для проверки GitHub Actions YAML;
- `cargo machete` на каждом pull request;
- `cargo llvm-cov --workspace --all-features --all-targets --summary-only`;
- Trivy filesystem/container scan;
- отдельная проверка `cargo +nightly udeps` на pull request либо более частый
  scheduled запуск;
- workflow-level `permissions: contents: read`;
- code-style тесты, требующие наличие этих job, timeout у каждого job и полные
  commit SHA у marketplace actions.

### Как переносить

Сначала добавить `permissions`, `actionlint` и `cargo machete`: это дешёвые
проверки без изменения кода. Coverage и Trivy лучше вынести в отдельные job,
чтобы они не удлиняли основной Clippy/test critical path.

### Критерии готовности

- каждый новый job имеет timeout;
- сторонние actions закреплены полным commit SHA;
- команды воспроизводимы локально либо явно помечены как CI-only;
- policy-тест проверяет наличие job и ключевой команды, а не имя job;
- coverage публикует summary и не требует внешнего сервиса для unit-тестов.

## 3. Compile-fail контракты маршрутов

**Приоритет: P1.**

**Статус: выполнено.**

### Что взять за основу

- `shared/src/typed_json_api_contracts.rs`;
- `shared/src/typed_json_api_macros.rs`;
- `route_contract_macros/src/lib.rs`;
- `tests/trybuild/typed_json_api_*`.

MapCam связывает тип маршрута с единственными допустимыми типами request и
response через associated types. Негативные `trybuild`-тесты доказывают, что
нельзя отправить payload одного маршрута в другой или вернуть несовместимый
response.

### Как адаптировать

Не переносить большой MapCam DSL целиком. Сначала создать минимальное ядро:

- marker type маршрута;
- `Route::Request` и `Route::Response`;
- типизированный клиентский вызов;
- четыре compile-fail fixture: неверный request, response, route и публичный /
  аутентифицированный transport.

После этого генератор CRUD сможет генерировать реализации минимального
контракта, не раскрывая MapCam-сущности.

### Критерии готовности

- корректная пара request/response компилируется;
- каждая некорректная комбинация имеет стабильный compile-fail тест;
- server route, frontend client и OpenAPI metadata получают сведения из одного
  описания;
- публичный API ядра остаётся минимальным;
- перенос не создаёт второй параллельный источник CRUD route metadata.

## 4. Runtime-сверка OpenAPI с фактическими ответами

**Приоритет: P1.**

**Статус: выполнено.**

### Что взять за основу

- `mapcam/tests/integration/openapi_runtime_conformance.rs`;
- `mapcam/tests/integration/openapi_route_parity.rs`;
- `mapcam/tests/integration/openapi_negative_cases.rs`.

В шаблоне уже есть статические проверки схем сгенерированного CRUD. Следующий
полезный уровень — запустить Router/сервер, выполнить репрезентативные запросы и
сверить фактические status, content-type и JSON shape с OpenAPI document.

### Критерии готовности

- проверены health/version, один публичный read и одна защищённая mutation;
- фактический status присутствует в `responses` соответствующей операции;
- JSON response декодируется в документированный schema type;
- отрицательные случаи проверяют 400/401/403/404/409/422, когда они объявлены;
- тест работает локально без внешней сети и использует изолированную БД только
  для сценариев, которым она действительно нужна.

## 5. Расширить `workspace_test_runner` режимами `nextest`

**Приоритет: P1.**

**Статус: выполнено.**

### Что взять за основу

- `.config/nextest.toml` MapCam с отдельными профилями;
- `workspace_test_runner/src/main.rs` MapCam:
  - `static`, `database`, `heavy-load`, `release`, `all`;
  - `--no-fail-fast`;
  - отдельный запуск ignored tests;
  - отдельный doc-test этап;
  - агрегированный итог по фазам;
  - проверка доступности optional release tools.

Текущий runner уже умеет режимы и сохраняет артефакты, поэтому переносить нужно
не реализацию логирования, а только недостающую оркестрацию.

### Критерии готовности

- обычные, ignored и doc tests не теряются;
- file/macro generator tests остаются сериализованными через test group;
- `all` останавливает зависимые фазы после провала prerequisite, но summary
  сохраняется всегда;
- отсутствующий optional tool отражается в отчёте понятным статусом;
- cargo fallback остаётся доступен, если `cargo-nextest` не установлен локально.

## 6. Инициализатор environment-файлов workspace

**Приоритет: P1.**

**Статус: выполнено.**

### Что взять за основу

- `initialize_environment_files/src/main.rs`;
- policy-тесты MapCam для `.env.example`: уникальность портов и логинов,
  обязательные переменные, порядок и согласованность адресов.

Шаблон уже проверяет равенство ключей `.env` и `.env.example`, но отдельная
утилита может создать отсутствующие `.env`, сохранить существующие секреты и
сформировать отчёт `created / updated / skipped`.

### Ограничения переноса

- нельзя перезаписывать существующие секреты;
- пути workspace member необходимо нормализовать и запрещать выход через `..`;
- MapCam-specific переменные следует заменить декларативным набором общих
  значений либо оставить только копирование `.env.example` в `.env`;
- вывод не должен содержать значения секретов.

### Критерии готовности

- dry-run показывает план без записи;
- повторный запуск идемпотентен;
- существующий `.env` не теряет пользовательские значения;
- тесты используют временный workspace и не меняют реальные environment-файлы;
- отчёт содержит только пути, ключи и статусы, но не значения.

## 7. Типизированные SQL-идентификаторы и запрет сырых schema/table names

**Приоритет: P2.**

**Статус: выполнено.**

### Что взять за основу

- `shared/src/sql_identifier.rs`;
- `shared/src/sql_column.rs`;
- `shared/src/query_builder.rs`;
- policy-тесты MapCam:
  - `forbids_raw_schema_identifiers_in_scoped_sql_query_literals`;
  - `forbids_raw_table_names_after_sql_table_clauses_in_rust_sources`;
  - `forbids_raw_postgres_schema_fragments_in_rust_sql_sources`.

Это особенно полезно генераторам `pg_crud`: динамический SQL должен принимать
только проверенный identifier wrapper, а повторяющиеся имена таблиц должны идти
из одного контрактного источника.

### Критерии готовности

- identifier разрешает только согласованный ASCII grammar;
- динамический query builder не принимает `&str` для schema/table/column;
- bind values по-прежнему передаются параметрами, а не интерполируются;
- policy-тест начинает с точного baseline и запрещает новые сырые идентификаторы;
- proc-macro generated SQL покрыт positive и negative tests.

## 8. Контроль регрессий производительности

**Приоритет: P2.**

**Статус: выполнено.**

### Что взять за основу

- CI job `cargo bench + criterion baseline compare` из MapCam;
- `shared/src/algorithm_benchmarks.rs` как пример выделения чистых алгоритмов;
- отдельный nightly job, чтобы шум benchmark не блокировал быстрый feedback.

Для шаблона разумные первые цели — генерация CRUD token streams, разбор больших
filter payload и построение SQL для bulk operations.

### Критерии готовности

- benchmark не использует БД, сеть и wall-clock sleeps;
- фиксированы размеры входов и seed;
- CI сохраняет baseline и сравнивает его в той же среде;
- порог учитывает шум runner и не превращает benchmark в flaky test;
- сначала собирается история измерений, и только затем регрессия делает job
  обязательным.

## Что повторно переносить не нужно

Следующие решения MapCam уже представлены в шаблоне и не должны становиться
отдельными задачами без конкретного пробела:

- request ID и correlation ID;
- request timeout и `Retry-After`;
- security headers и graceful shutdown;
- bounded semaphore permits и resource budgets;
- observable background tasks и async run history;
- trusted-proxy client IP resolution;
- CORS parsing;
- cookie CSRF token, проверка Origin и rate limiting в admin API;
- idempotency для generated mutations;
- cargo-deny, cargo-audit, cargo-hack, semver-checks и gitleaks;
- Taplo, Typos, Docker example и test-run artifacts;
- hardened release profile и aliases ежедневной проверки.

## Рекомендуемый порядок внедрения

1. Добавить workflow permissions, actionlint и cargo-machete.
2. Реализовать bounded file reads и policy для runtime-кода.
3. Добавить llvm-cov и Trivy как независимые CI jobs.
4. Расширить test runner без изменения существующего формата артефактов.
5. Добавить минимальное ядро typed route contracts и compile-fail fixtures.
6. Построить runtime OpenAPI conformance поверх этих контрактов.
7. Добавить безопасный `.env` initializer.
8. Вводить SQL identifier policy и benchmarks после фиксации baseline.

Такой порядок сначала закрывает дешёвые риски безопасности и сопровождения, а
затем переходит к изменениям архитектуры API и генераторов.
