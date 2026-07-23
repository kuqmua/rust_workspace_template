# Покрытие чтения полей БД через API

## Итог

- Для существующего административного `GET /tables/{table}` теперь есть один
  исчерпывающий интеграционный тест, который проходит по всем 12 таблицам и отдельно
  сопоставляет каждое публичное поле со значением, прочитанным из PostgreSQL.
- В workspace используются две отдельные PostgreSQL БД: основная БД сервера с
  административной схемой и БД `notification_service`.
- В административном API маршрут `GET /tables/{table}` поддерживает 12 таблиц.
  Тест `postgresql_data_table_api_reads_every_public_field_from_every_table`
  проверяет все таблицы, названия и порядок колонок, количество строк, наличие
  каждого значения и его равенство значению PostgreSQL.
- Для `users`, `roles`, `permissions`, `user_roles`, `role_permissions` и
  `system_settings` также сгенерированы типизированные read-маршруты
  `POST /admin_<table>/rm` и `POST /admin_<table>/ro`. Для них есть unit-тесты
  сериализации запросов и соответствия маршрутов OpenAPI, но нет интеграционного
  теста, который читает известную запись из PostgreSQL через эти HTTP-маршруты и
  отдельно проверяет все возвращённые поля.
- HTML-тест открывает страницы всех 12 таблиц, но проверяет только HTTP 200 и наличие
  общей CSR-оболочки. Это подтверждает регистрацию маршрутов, но не чтение отдельных
  полей.
- У `notification_service` есть только `POST /notifications`. API чтения уведомлений
  отсутствует, поэтому API-тестов чтения полей `notifications` быть не может.

## Критерий проверки

Поле отмечается как покрытое полностью только если интеграционный тест:

1. записывает или однозначно находит строку с известным значением поля;
2. выполняет запрос к публичному HTTP API чтения;
3. декодирует ответ;
4. отдельно сравнивает полученное значение этого поля с ожидаемым.

Проверка только статуса ответа, общей HTML-оболочки, количества строк, SQL-запроса
напрямую к БД или наличия любого значения в строке не считается полной проверкой
чтения конкретного поля через API.

## Основная БД сервера: административная схема

Общий read-only маршрут таблиц: `GET /tables/{table}`. У шести сгенерированных
таблиц дополнительно есть типизированные read-many/read-one маршруты
`POST /admin_<table>/rm` и `POST /admin_<table>/ro`.

### `users`

- Доступные через API поля:
  - [x] `id`
  - [x] `login`
  - [x] `display_name`
  - [x] `is_banned`
  - [x] `created_at`
  - [x] `updated_at`
- Не выдаётся этим API намеренно: `password_hash`.
- Итог: **все публичные поля `GET /tables/users` покрыты**.

### `roles`

- Доступные через API поля:
  - [x] `id`
  - [x] `name`
  - [x] `is_system`
  - [x] `created_at`
  - [x] `updated_at`
- Итог: **все публичные поля `GET /tables/roles` покрыты**.

### `permissions`

- Доступные через API поля:
  - [x] `id`
  - [x] `name`
  - [x] `created_at`
- Итог: **все публичные поля `GET /tables/permissions` покрыты**.

### `user_roles`

- Доступные через API поля:
  - [x] `id`
  - [x] `user_id`
  - [x] `role_id`
  - [x] `created_at`
- Итог: **все публичные поля `GET /tables/user_roles` покрыты**.

### `role_permissions`

- Доступные через API поля:
  - [x] `id`
  - [x] `role_id`
  - [x] `permission_id`
  - [x] `created_at`
- Итог: **все публичные поля `GET /tables/role_permissions` покрыты**.

### `refresh_tokens`

- Доступные через API поля:
  - [x] `id`
  - [x] `user_id`
  - [x] `expires_at`
  - [x] `created_at`
  - [x] `revoked_at`
- Не выдаётся этим API намеренно: `token_hash`.
- Итог: **все публичные поля `GET /tables/refresh_tokens` покрыты**.

### `access_sessions`

- Доступные через API поля:
  - [x] `id`
  - [x] `user_id`
  - [x] `expires_at`
  - [x] `created_at`
  - [x] `revoked_at`
- Не выдаются этим API намеренно: `token_identifier_hash`, `csrf_token_hash`,
  `token_context_hash`.
- Итог: **все публичные поля `GET /tables/access_sessions` покрыты**.
- Общий одинаковый набор публичных колонок `access_sessions` и `refresh_tokens`
  теперь явно выражен общей константой `SERVER_ADMIN_DATA_SESSION_COLUMNS`.

### `login_attempts`

- Доступные через API поля:
  - [x] `id`
  - [x] `login`
  - [x] `ip_address`
  - [x] `succeeded`
  - [x] `attempted_at`
- Итог: **все публичные поля `GET /tables/login_attempts` покрыты**.

### `audit_log`

- Доступные через API поля:
  - [x] `id`
  - [x] `user_id`
  - [x] `user_login`
  - [x] `action`
  - [x] `resource`
  - [x] `resource_id`
  - [x] `request_id`
  - [x] `succeeded`
  - [x] `details`
  - [x] `created_at`
- Отдельный маршрут журнала аудита проверяется на статус, пагинацию и наличие
  успешных/неуспешных записей в БД, но значения всех полей API-ответа отдельно не
  сравниваются.
- Итог: **все публичные поля `GET /tables/audit_log` покрыты**.

### `system_settings`

- Доступные через API поля:
  - [x] `id`
  - [x] `site_name`
  - [x] `tab_title`
  - [x] `main_logo`
  - [x] `primary_color`
  - [x] `default_admin_route`
  - [x] `organization_name`
  - [x] `organization_contacts`
  - [x] `support_url`
  - [x] `updated_at`
- HTML-тест последовательно меняет восемь редактируемых полей и после каждого
  изменения выполняет `GET /admin/settings`, но проверяет только статус и общую
  CSR-оболочку. Конечные значения сравниваются прямым SQL-запросом, а не по
  API-ответу. `id` и `updated_at` отдельно не проверяются.
- Итог: **все публичные поля `GET /tables/system_settings` покрыты**; HTML-сценарий
  остаётся дополнительной проверкой редактирования.

### `rate_limits`

- Доступные через API поля:
  - [x] `scope`
  - [x] `subject`
  - [x] `window_started_at`
  - [x] `request_count`
- Итог: **все публичные поля `GET /tables/rate_limits` покрыты**.

### `cleanup_status`

- Доступные через API поля:
  - [x] `singleton`
  - [x] `last_success_at`
  - [x] `last_deleted_rows`
- Тест фоновой очистки проверяет состояние таблицы напрямую через SQL, а не через
  `GET /tables/cleanup_status`.
- Итог: **все публичные поля `GET /tables/cleanup_status` покрыты**.

## БД `notification_service`

### `notifications`

- Поля БД:
  - [ ] `id`
  - [ ] `message`
  - [ ] `created_at`
- `POST /notifications` возвращает только созданный `id`.
- Интеграционный тест проверяет HTTP 201 и то, что возвращённый `id` не равен nil UUID.
  Он не читает запись обратно через API и не проверяет `message` или `created_at`.
- GET-маршрут для чтения одной записи или списка уведомлений отсутствует.
- Итог: **чтение полей через API не реализовано и не протестировано**.

## Какие тесты существуют сейчас

- `postgresql_data_table_api_reads_every_public_field_from_every_table`
  - проходит по `AdminDataTable::PG_ORDER`, поэтому охватывает все 12 таблиц;
  - для таблиц, которые могут быть пустыми после подготовки fixture, создаёт
    детерминированные строки;
  - вызывает `GET /tables/{table}` с аутентификацией;
  - отдельно проверяет имя и позицию каждой публичной колонки;
  - отдельно сравнивает каждое значение каждой строки API с результатом PostgreSQL.
- `postgresql_auth_rbac_csrf_session_and_audit_flow`
  - вызывает `GET /tables/users`;
  - проверяет пагинацию;
  - проверяет фильтр по `login`;
  - остаётся дополнительной проверкой фильтрации.
- Сгенерированные `*_rm_payload_json_round_trip`, `*_ro_payload_json_round_trip` и
  `*_route_open_api_parity`
  - проверяют форму запросов, сериализацию и документацию типизированных маршрутов
    для шести таблиц;
  - не запускают HTTP-запрос к PostgreSQL и не подтверждают фактическое чтение
    значений полей.
- `postgresql_html_router_registers_every_owned_page_and_action`
  - открывает HTML-страницу каждой из 12 административных таблиц;
  - проверяет HTTP 200 и CSR-оболочку;
  - не проверяет значения полей.
- `postgresql_html_settings_updates_and_reads_every_field_separately`
  - меняет восемь редактируемых настроек по одной;
  - выполняет HTML GET после изменения;
  - проверяет значения прямым SQL, но не содержимым API/HTML-ответа.
- `postgresql_html_users_crud_covers_every_frontend_field_separately` и
  `postgresql_html_roles_crud_covers_every_frontend_field_separately`
  - проверяют изменения через HTML actions;
  - итоговые значения преимущественно подтверждаются SQL-запросами;
  - не являются полными тестами чтения всех полей `GET /tables/{table}`.
- `create_notification_persists_through_http_route`
  - проверяет только создание уведомления и возвращённый `id`;
  - чтение записи через API не проверяет.

## Источники

- Реестр 12 административных таблиц:
  `server_admin_contract/src/lib.rs`, `AdminDataTable::PG_ORDER`.
- Состав полей генерируемых таблиц и исключение `password_hash`:
  `server_admin/src/generated_tables.rs`.
- Реализация и генерируемые contract-тесты типизированных CRUD-маршрутов:
  `pg_crud/pg_table/generate_pg_table_src/src/source.rs`.
- Фактически возвращаемые колонки и SQL чтения:
  `str_constants/src/lib.rs`, константы `SERVER_ADMIN_DATA_*_COLUMNS`,
  `SERVER_ADMIN_DATA_SESSION_COLUMNS` и
  `SERVER_ADMIN_DATA_*_SQL`.
- Диспетчеризация `GET /tables/{table}`:
  `server_admin/src/repository/data_tables.rs`.
- Интеграционные API/HTML-тесты:
  `server_admin/tests/admin_api.rs`.
- Схема и API уведомлений:
  `notification_service/migrations/0001_notifications.sql` и
  `notification_service/src/main.rs`.
