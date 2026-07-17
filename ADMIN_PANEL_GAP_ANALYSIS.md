# Аудит готовности административной панели

Дата аудита: 2026-07-17

## Цель и границы

Документ отвечает на практический вопрос: чего не хватает текущей реализации, чтобы её можно
было считать полноценной production-админкой для управления самим сервисом. Выводы сделаны по
текущим контрактам, маршрутам, PostgreSQL schema, backend handlers, Leptos UI и тестам. Функции,
которые зависят от предметной области будущего приложения (управление заказами, публикациями и
тому подобное), намеренно не считаются обязательными.

## Что уже реализовано

- вход, refresh, выход, CSRF-защита и серверно отзываемые access/refresh sessions;
- RBAC с отдельными permissions для чтения и изменения пользователей, ролей, настроек, аудита,
  метрик и OpenAPI;
- создание, изменение, блокировка, смена пароля и удаление администраторов;
- создание, изменение и удаление ролей, назначение ролей и permissions;
- защита от удаления или лишения прав последнего активного администратора;
- append-only audit log, фиксация неудачных входов, rate limiting и фоновая очистка;
- типизированные route contracts, OpenAPI, body limits и проверяемые DTO;
- страницы пользователей, ролей, permissions, аудита, настроек, метрик, версии и OpenAPI;
- базовые поиск, сортировка и клиентская пагинация таблиц;
- обработка истечения access session, конкурентного refresh, browser history и устаревших
  ответов навигации;
- backend integration tests и небольшой Playwright-набор для критической навигации и refresh.

Это уже рабочая основа. Основной разрыв сейчас находится между возможностями backend и качеством
операторского UI.

## Сводка пробелов

| Приоритет | Область | Что отсутствует или недостаточно |
|---|---|---|
| P0 | Роли и permissions | UI не знает текущие назначения и принимает списки числовых ID через `prompt` |
| P0 | Сессии | API управления сессиями есть, но страницы сессий в SPA нет |
| P0 | Большие наборы данных | Нет серверной пагинации, поиска и сортировки; часть данных молча обрезается |
| P0 | Формы и мутации | Нет нормальных диалогов, field errors, pending/success state и защиты от повторной отправки |
| P0 | Audit log | Backend filters и подробности события почти не доступны из UI |
| P1 | Безопасность входа | Нет MFA/WebAuthn/TOTP, recovery codes и step-up authentication |
| P1 | Профиль администратора | Нет отдельного экрана своего профиля и безопасной смены собственного пароля |
| P1 | Настройки | Сохранённые branding/settings почти не применяются самой админкой |
| P1 | Обзор системы | Нет dashboard; metrics и version показываются как необработанный текст |
| P1 | Тестирование UI | Не покрыты основные CRUD flows, permissions matrix, формы и управление сессиями |
| P2 | Операционные функции | Нет export, maintenance/job status и понятной диагностики состояния зависимостей |
| P2 | Доступность и UX | Нет подтверждённого keyboard/screen-reader/a11y контракта и устойчивых notifications |

## P0. Сделать назначения ролей и permissions безопасными

### Текущее состояние

`AdminUserSummary` не содержит назначенные роли, а `AdminRoleSummary` не содержит назначенные
permissions. В `server_admin_frontend/src/app/tables.rs` оператору предлагается вручную ввести
числовые ID через browser `prompt`. Парсеры в `app/forms.rs` молча отбрасывают неверные элементы.
После этого endpoint полностью заменяет набор назначений.

Оператор не видит исходное состояние и может случайно отправить пустой или неполный список.
Особенно опасен ввод вроде `1, invalid, 2`: UI не сообщает об ошибке и применяет только `1, 2`.

### Что нужно

- добавить read model или отдельные endpoints для ролей пользователя и permissions роли;
- заменить ввод ID на modal/drawer с checkbox, названиями, поиском и текущим состоянием;
- показывать diff перед сохранением для системных или критичных ролей;
- считать любой неизвестный ID/невалидный ввод ошибкой, а не пропускать его;
- после сохранения показывать подтверждение и обновлённые назначения;
- добавить конкурентную защиту (`version`/ETag или эквивалент), чтобы два оператора не затирали
  изменения друг друга.

### Критерии готовности

- назначение выполняется без знания database ID;
- UI до отправки показывает полный текущий и будущий набор;
- неверный элемент блокирует всю операцию с понятным сообщением;
- Playwright проверяет добавление, удаление, конфликт и защиту последнего администратора.

## P0. Добавить страницу управления сессиями

### Текущее состояние

В `server_admin_contract` и backend уже существуют `AdminSessionsRoute`,
`AdminRevokeSessionRoute` и `AdminRevokeAllSessionsRoute`. В `AdminPage`, `Page` и SPA navigation
соответствующей страницы нет. Таким образом, security-функция доступна через API, но не через
админку.

`AdminSessionView` содержит только identifier, creation time и expiry. Для осмысленного выбора
сессии оператору обычно также нужны признак текущей сессии и безопасные метаданные устройства:
последняя активность, browser/user-agent summary и приблизительный IP (с учётом политики
персональных данных).

### Что нужно

- добавить страницу «Sessions / Security»;
- помечать текущую сессию и не позволять случайно отозвать её без отдельного подтверждения;
- поддержать отзыв одной и всех остальных сессий;
- определить и документировать, какие device/IP metadata допустимо хранить;
- добавить empty/loading/error/success состояния и browser tests.

## P0. Перенести таблицы на серверную пагинацию

### Текущее состояние

Users endpoint возвращает не более 500 записей (`SERVER_ADMIN_LIST_USERS_SQL`), но не возвращает
total или cursor. UI затем ищет, сортирует и делит на страницы только полученный массив. При 501-м
пользователе интерфейс молча создаёт впечатление, что записей больше нет. Roles и permissions
загружаются целиком без limit. Audit endpoint возвращает только последние 200 записей, также без
курсора; UI дополнительно пагинирует этот фрагмент локально.

### Что нужно

- ввести общие typed query/page contracts: cursor или offset, limit, sort, direction, filters;
- возвращать `items` вместе с `next_cursor` или надёжным `total`;
- выполнять поиск и сортировку в PostgreSQL;
- для audit использовать стабильный keyset cursor, например `(created_at, id)`;
- сохранять фильтры и страницу в URL, чтобы ссылку можно было открыть повторно;
- ограничить максимальный page size и стоимость поисковых запросов.

### Критерии готовности

- запись за пределами первых 500 пользователей достижима из UI;
- audit можно просматривать глубже последних 200 событий;
- сортировка и поиск применяются ко всему набору, а не к загруженному фрагменту;
- contract tests проверяют стабильность страниц при параллельных insert/delete.

## P0. Заменить browser prompts полноценными формами

### Текущее состояние

Создание и изменение пользователя/роли, смена пароля и назначения выполняются через
`window.prompt`. Только sign-in имеет `pending` и локальное сообщение об ошибке. Settings form
использует большой `if let`: при невалидном поле submit просто ничего не делает. Общий
`run_action` при ошибке заменяет всю страницу error-screen, а при успехе только перезагружает
данные. Для большинства кнопок нет pending state, блокировки повторного клика или success
notification.

### Что нужно

- отдельные typed form components для create/edit/password/ban/roles/permissions/settings;
- field-level validation с теми же правилами, что в contract types;
- общий mutation state: idle, pending, success, validation error, conflict, server error;
- запрет повторной отправки, сохранение введённых значений после ошибки и retry;
- toast/inline success feedback без потери текущей страницы и table state;
- понятное отображение `409`, `422`, `429` и `Retry-After`;
- destructive confirmations с названием ресурса и объяснением последствий.

## P0. Довести audit log до операторского инструмента

### Текущее состояние

Backend `AdminAuditQuery` уже принимает user, action, resource и временной диапазон. Frontend
всегда вызывает audit route без query parameters. Таблица отображает только время, user ID,
action, resource и success. Хотя `AdminAuditView` содержит login, resource ID, event ID и JSON
details, UI их не показывает.

### Что нужно

- фильтры по времени, пользователю/login, action, resource, resource ID и результату;
- раскрываемая карточка события с request ID (если он входит в публичный DTO), details и
  нормальным форматированием JSON;
- ссылки из события на соответствующего пользователя/ресурс;
- server pagination и export с отдельным permission и жёсткими limits;
- явная политика retention, timezone и маскирования секретов/PII.

## P1. Усилить безопасность учётных записей

Текущий код не содержит TOTP, WebAuthn/passkeys, recovery codes или MFA challenge. Для панели с
правами управления пользователями одного password-фактора обычно недостаточно.

Минимальный production-вариант:

- enrollment и удаление TOTP/WebAuthn только после повторной проверки пароля;
- recovery codes, отображаемые один раз и хранимые только как hashes;
- политика обязательного MFA для привилегированных ролей;
- step-up authentication для смены ролей, удаления администратора и security settings;
- аудит enrollment, failed challenge, recovery и отключения MFA;
- защита от lockout и документированный recovery flow.

Точный набор следует выбирать по threat model. Если админка доступна только через внешний SSO,
локальную MFA лучше не дублировать, а реализовать OIDC/SAML и проверку нужного assurance level.

## P1. Добавить профиль и self-service security

`AuthenticatedAdmin` содержит текущего пользователя, но SPA использует его в основном для header
и проверки permissions. Нет отдельного экрана, где администратор может:

- увидеть login, display name и роли;
- сменить собственный пароль с подтверждением текущего;
- посмотреть и завершить свои сессии;
- настроить MFA;
- увидеть недавние security events.

Смена собственного пароля не должна требовать permission на изменение всех пользователей. После
смены пароля нужна явно выбранная политика отзыва остальных сессий.

## P1. Реально применять system settings

Settings сохраняются и читаются через API, но поиск usages показывает, что `site_name`,
`tab_title`, `main_logo`, `primary_color`, organization data, support URL и default route почти не
используются вне settings repository/form. Header и sign-in по-прежнему содержат статические
`Admin Console`, `Admin`, букву `A` и фиксированный цвет из CSS.

Нужно:

- загружать публично допустимую часть branding до/на sign-in;
- применять tab title, logo, site name и primary color через безопасные CSS variables;
- использовать default route после входа и при открытии `/admin`;
- валидировать URL и цвет семантически, а не только по длине;
- определить `null`/очистку optional settings: текущая form всегда отправляет `Some`, даже для
  пустой строки;
- добавить preview и rollback/default action.

## P1. Добавить dashboard вместо raw runtime text

Metrics и version сейчас отображаются одним `<pre>`, OpenAPI — JSON text. Для повседневной
работы нужен небольшой dashboard, а не попытка заменить полноценную observability-систему.

Полезный минимум:

- версия/commit, uptime и состояние database;
- число активных сессий и недавних failed sign-ins;
- последние административные изменения;
- health зависимостей без выдачи секретных деталей;
- ссылки на внешние logs/metrics/traces dashboards;
- отдельные permissions для чувствительных operational данных.

Prometheus/Grafana не следует полностью встраивать в SPA: админка должна показывать summary и
ссылки, а не становиться второй системой мониторинга.

## P1. Расширить автоматизированные проверки UI

Текущий Playwright-набор проверяет sign-in rendering, refresh, временную ошибку session check,
навигацию, базовую таблицу, history, stale response и конкурентный refresh. Он не доказывает
работоспособность большинства административных операций.

Нужно покрыть:

- create/edit/ban/password/delete user;
- create/edit/delete role и назначения;
- permissions matrix: скрытая/disabled операция и прямой `403`;
- settings validation, сохранение и применение branding;
- session revoke и revoke-all;
- audit filters/details/pagination;
- `409`, `422`, `429`, network retry и повторный submit;
- keyboard navigation, focus restoration после modal и базовый automated a11y scan;
- mobile widths и длинные локализованные значения;
- production Trunk build и smoke test собранного WASM, а не только fixture behavior.

## P2. Операционные возможности

После P0/P1 имеет смысл добавить только те функции, которые нужны реальному deployment:

- read-only status фоновых cleanup/job процессов и время последнего успешного запуска;
- безопасный CSV/JSON export пользователей и audit log с отдельными permissions;
- maintenance banner/read-only mode;
- управление feature flags, если они появятся в основном приложении;
- диагностику email/object storage/очередей, если эти зависимости появятся;
- документированный bootstrap и emergency access procedure.

Backup/restore базы, произвольный SQL console, просмотр секретов и редактирование environment
variables через браузер не следует добавлять в обычную админку: риск значительно выше пользы.

## Рекомендуемый порядок реализации

### Этап 1 — безопасные ежедневные операции

1. Read models назначений и selector UI ролей/permissions.
2. Полноценные формы и единый mutation/error state.
3. Страница сессий на уже существующем API.
4. Audit filters/details.

### Этап 2 — масштабирование

1. Общие server page/query contracts.
2. Users, roles, permissions и audit на server-side pagination.
3. URL state и concurrency/version protection.

### Этап 3 — security и персонализация

1. Self-service profile/password/sessions.
2. MFA или интеграция с внешним SSO согласно threat model.
3. Фактическое применение system settings и branding.

### Этап 4 — эксплуатационная зрелость

1. Dashboard summary и dependency health.
2. Полный CRUD/permissions/a11y Playwright-набор.
3. Export, job status и runbook для bootstrap/recovery.

## Определение «полноценной админки» для этого проекта

Админку можно считать полноценной после выполнения P0 и основных пунктов P1, когда:

- все существующие backend admin capabilities доступны через безопасный UI;
- оператор нигде не вводит внутренние ID вслепую;
- списки не теряют записи из-за скрытых limits;
- каждая мутация имеет validation, pending, success, conflict и retry UX;
- администратор управляет собственными security settings и sessions;
- критичные действия защищены MFA/SSO assurance согласно threat model;
- настройки действительно меняют интерфейс;
- audit позволяет найти и исследовать событие, а не только увидеть последние 200 строк;
- end-to-end тесты покрывают права и основные destructive flows;
- operational dashboard показывает состояние, но не раскрывает секреты и не заменяет внешнюю
  observability.
