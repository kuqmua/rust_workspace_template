# План оптимизации аллокаций памяти

## 1. Цель и границы

Цель — уменьшить число heap-аллокаций, объём временной памяти и копирование данных без изменения публичного API, SQL, JSON/OpenAPI-контрактов и поведения ошибок. Оптимизация выполняется по измерениям, отдельно для runtime-кода и для compile-time/codegen-кода.

В план входят все production-crates workspace. Тестовые crates рассматриваются только там, где они запускают `cargo`, строят большие снимки или заметно влияют на длительность CI. Добавление зависимостей не требуется: начальный профиль строится средствами ОС, существующими тестами и Rust nightly. Любой новый профилировщик в зависимостях workspace требует отдельного согласования.

Не являются самостоятельной целью:

- уменьшение размера бинарника;
- замена понятного кода на небезопасный код;
- устранение аллокаций из холодных путей ошибок ценой усложнения API;
- перенос владения в `Arc`, если данные не разделяются между потоками;
- изменение доменных wrapper-типов или публичных сигнатур без отдельного решения.

## 2. Критерии готовности

Для каждой выполненной оптимизации должны быть зафиксированы:

1. сценарий и команда измерения;
2. медиана минимум из 10 прогонов после одного прогрева;
3. число аллокаций, суммарно выделенные байты и peak RSS до/после;
4. время выполнения до/после, чтобы не обменять память на существенное замедление;
5. тест, подтверждающий неизменность результата;
6. отсутствие регрессий в обязательных workspace-проверках.

Минимальная цель для принятия изменения: не менее 10% уменьшения аллокаций в выбранном сценарии или устранение хотя бы одной аллокации на каждый HTTP-запрос/строку bulk-операции/поле генератора. Для холодного кода изменение принимается только при заметном упрощении владения либо снижении peak memory.

## 3. Методика измерения

### 3.1. Сценарии

| ID | Контур | Сценарий | Основные метрики |
|---|---|---|---|
| R1 | `server` | 10 000 запросов к `/status` с готовым request ID | allocations/request, bytes/request, p50/p99 |
| R2 | `server` | 10 000 запросов без request ID | цена генерации UUID и заголовков |
| R3 | `cmn_routes` | 10 000 ответов 404 с коротким и длинным URI | allocations/response, временные `String` |
| R4 | `pg_tbl` | генерация каждого из 8 SQL CRUD-запросов, размеры 1/10/100/1000 | allocations/query, bytes/query |
| R5 | `wh_flts` | фильтры с 1/10/100/1000 значениями | рост памяти относительно числа значений |
| C1 | `gen_pg_types_src` | полный текущий fixture генератора | allocations/expansion, peak RSS, время |
| C2 | `gen_pg_tbl_src` | текущий полный table fixture | allocations/expansion, peak RSS, размер token stream |
| C3 | `gen_wh_flts_src` | полный fixture фильтров | allocations/expansion, peak RSS |
| T1 | `tests code_style` | полный текущий запуск | peak RSS и объём прочитанных/скопированных source strings |

### 3.2. Инструменты без изменения зависимостей

- `/usr/bin/time -v` — peak RSS и wall time процесса;
- `heaptrack` или Valgrind Massif, если они установлены локально, — источники и объёмы аллокаций;
- отдельные `#[test]`/`#[bench]` на nightly с `std::hint::black_box` для CPU-времени;
- временный counting global allocator только в локальном benchmark/test binary, не в library/runtime-коде;
- существующие golden/roundtrip-тесты генераторов — проверка эквивалентности результата.

Перед изменениями сохранить результаты в таблице в описании PR. Не сравнивать разные профили сборки, версии toolchain или наборы features.

## 4. Приоритеты

- **P0** — аллокация выполняется на каждом запросе, SQL-операции или элементе bulk-ввода.
- **P1** — аллокация масштабируется с числом полей/типов при macro expansion либо повышает peak memory CI.
- **P2** — редкий startup/error/test path; менять после подтверждения профилем.
- **Keep** — владение необходимо или оптимизация потребует нежелательного изменения API.

## 5. Runtime-модули

### 5.1. `server/src/main.rs`

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `parse_separated_values` | собирает новый `Vec`, ёмкость заранее неизвестна | оставить как контрольный вариант; передавать iterator напрямую невозможно, потому что возвращается владеющий wrapper | Keep; тесты parsing |
| `parse_separated_values_with_capacity` | уже считает разделители и вызывает `Vec::with_capacity` | измерить, окупает ли первый проход второй; для короткого CORS списка один проход может быть дешевле | P2; 1/4/32/256 элементов |
| `split_count` | второй полный проход по строке | объединять подсчёт и parse только если профиль показывает выигрыш; альтернативно использовать консервативную малую capacity | P2; CPU и reallocations вместе |
| `parse_cors_allow_origin_value` | header value создаётся из trimmed slice | подтвердить, что `HeaderValue::from_str` является единственным необходимым копированием | Keep; allocations/origin |
| `parse_cors_allow_origin` | владеющий список нужен слою CORS | исключить дополнительные промежуточные коллекции; сохранить пропуск невалидных значений | P2; текущие unit tests |
| `mk_api_routes` | `Router` клонирует внутренние shared nodes при композиции | проверить ненужные `clone` router/state; не заменять внутреннее sharing собственным `Arc` | P2; route parity tests |
| `mk_app_state` | создаёт общий state один раз | убедиться, что крупные config values перемещаются, а не клонируются | P2; startup profile |
| `mk_pg_pool`, `run_server` | startup-only | не оптимизировать до request paths; перемещать config/handles по возможности | Keep/P2 |

### 5.2. `server_runtime/src/lib.rs`

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `RequestId::try_from(String)` | корректно принимает владение без нового буфера | сохранить consuming conversion; добавить проверку, что hot path не делает повторный `String` | Keep |
| `RequestId::try_from(&HeaderValue)` | выполняет `to_owned` для extension | владение необходимо после завершения borrow запроса; оценить хранение одного `HeaderValue`-backed wrapper только если API допускает | P0; R1 |
| `RequestIdLayer::call` | клонирует request ID, method и path; затем клонирует header в response | определить реально нужные значения в async future; method можно форматировать до future или хранить дешёвым enum clone, path — писать в span без owned `String`, если lifetime future это позволяет; создать header один раз и перемещать, где возможно | P0; R1/R2 и сохранение обоих response headers |
| UUID fallback в `RequestIdLayer::call` | `Uuid::to_string` выделяет `String` | проверить `Uuid::as_hyphenated` + запись непосредственно в заранее выделенный/stack buffer, затем одно создание header/request ID; не добавлять crate | P0; R2 |
| `RequestTimeoutService::call` | `as_secs().to_string()` только на timeout | оставить до профиля: холодный error path | Keep |
| `SecurityHeadersService::call` | статические header values не должны аллоцировать на запрос | подтвердить профилем; использовать `HeaderValue::from_static` и константные имена везде | P0 verification |
| `AsyncRunHistory::clone` | clone `Arc` семантически нужен для cross-task sharing | сохранить | Keep |
| `AsyncRunHistory::push` | queue владеет report; eviction не должен копировать | проверить `VecDeque` capacity и стабильность maximum length; создать с capacity в `new` | P1; 10× limit pushes |
| `AsyncRunHistory::snapshot` | клонирует все reports по публичному контракту | измерить размер report и частоту status polling; рассмотреть iterator/borrow невозможно через async lock, поэтому менять API только отдельно; допустим snapshot `Arc<[T]>` лишь при доказанном multi-consumer sharing | P2; не менять API сейчас |
| `spawn_interval_task` | closure и task создаются один раз | не оптимизировать | Keep |
| `serve_with_graceful_shutdown` | startup/shutdown-only | не оптимизировать | Keep |

### 5.3. `cmn_routes/src/lib.rs`

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `mk_no_route_msg` | строит owned сообщение для JSON | оставить одну итоговую `String`; не создавать отдельно suffix string | P0; R3 |
| `mk_no_route_msg_for_suffix` | уже использует точную capacity | проверить формулу для UTF-8 и `write!`/`push_str`; исключить `format!` вокруг частей | P0; exact-capacity test и R3 |
| `get_uri_suffix` | возвращает borrowed `&str` | сохранить borrow и не вводить `String` | Keep |
| `mk_not_found_payload` | payload должен владеть данными после request borrow | перемещать готовое сообщение; commit/swagger URL брать borrowed/static или `Cow` из state | P0; response shape |
| `mk_commit_json_res` | общая упаковка payload | проверить отсутствие clone commit link при каждой выдаче health/git-info; использовать существующий `Cow` по назначению | P1; health endpoint profile |
| `cmn_routes` | state обёрнут в `Arc<dyn ...>` | `Arc` оправдан разделением между handlers; не добавлять вложенный `Arc` | Keep |

### 5.4. `route_validators`

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `check_body_size` | агрегирует body в `Bytes`, что необходимо для дальнейшего parse | проверить, нет ли конвертации `Bytes -> Vec -> String` у вызывающих; передавать `Bytes`/slice дальше | P0; body 0/limit/limit+1/large chunks |
| `BodySizeEr::to_err_string` | pretty `Debug` через `format!` на error path | заменить на прямой `Display`/структурированную запись только если формат ошибки можно сохранить буквально | P2; snapshot текста ошибки |
| `check_commit` functions | сравнение заголовка должно идти по borrowed bytes/str | аудит на `to_str().to_owned()` и сохранение borrowed comparison | P1; валидный/невалидный header |
| `hdr_val` functions | header parsing | не создавать нормализованную `String`, если достаточно `HeaderValue`/`&str` | P1; allocation count/header |

### 5.5. `pg_crud/pg_tbl/src/lib.rs` — построение SQL

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `gen_insert_query_string` | `format!` создаёт итоговый query buffer | вычислять приблизительную/точную capacity и писать через `std::fmt::Write`; одна итоговая аллокация | P0; R4, byte-for-byte SQL |
| `gen_select_query_string` | две ветки `format!`, все части borrowed | единый pre-sized buffer + `push_str`; не создавать пустой `where` fragment | P0; R4 |
| `gen_update_query_string` | форматирование selector и fragments | передать fragments как refs и собрать один раз; избежать промежуточного wrapped selector | P0; R4 |
| `gen_delete_query_string` | аналогично select/update | одна pre-sized `String`; сохранить пробелы буквально | P0; R4 |
| `gen_cm_query_string`, `gen_co_query_string`, `gen_rm_query_string`, `gen_ro_query_string`, `gen_um_query_string`, `gen_uo_query_string`, `gen_dm_query_string`, `gen_dlo_query_string` | делегируют общим builders | после оптимизации четырёх builders убедиться, что wrappers перемещают итоговый `String` без копии | P0; восемь golden tests |
| `gen_col_queals_v_comma_uo_qp` | capacity уже задана, но приблизительно | рассчитать длину по именам колонок и числу placeholders либо использовать верхнюю оценку без второго прохода | P0; 1/10/100/1000 columns |
| `gen_when_col_id_then_v_um_qp` | строка растёт с bulk rows | capacity зависит от длины идентификаторов и числа строк; исключить `format!` внутри iteration | P0; bulk sizes |
| `gen_col_eqs_case_acc_else_col_end_comma_um_qp` | горячая вложенная генерация bulk update | использовать один accumulator и `write!`; не создавать fragment на колонку/строку | P0; largest expected bulk |

### 5.6. `pg_crud/pg_crud_cmn/src/lib.rs`

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `LogicalOperator::to_qp` | маленькие строки создаются через `format!` | писать статические варианты через `String::from`/predefined `&'static str` в owning wrapper; отрицательные варианты собрать с точной capacity | P1; operator golden tests |
| `QpFragment` conversions и `fmt::Write::write_str` | центральный owning SQL buffer | сохранить consuming `into_inner`; проверить, что callers не вызывают `to_string` после получения wrapper | P0; call-site audit |
| `QpIncr::checked_add_one` и `incr_checked_add_one_returning_incr` | числовая логика без heap | не менять | Keep |
| `NotEmptyUnqVec::try_new` | duplicate search без дополнительной коллекции, но квадратичный CPU | память уже минимальна; `HashSet` увеличит память и требует bounds, поэтому оставить для малых Vec | Keep; задокументировать crossover |
| `NotEmptyUnqVec::try_new_by_hash` | HashSet-путь предназначен для больших Vec | задать capacity по `values.len()` внутри реализации и хранить refs/indices, не clone элементов | P1; 10/1000/10000 unique и duplicate |
| `first_duplicate_idx`, `take_fst_dup` | borrowed scan/одно remove | сохранить без временного Vec | Keep |
| `first_duplicate_idx_by_hash`, `take_fst_dup_by_hash` | возможна аллокация HashSet | `HashSet::with_capacity(values.len())`; проверить, что set живёт только до определения индекса | P1 |
| `NotEmptyUnqVec::from_t1_impl_from_t2` | новый Vec неизбежен при смене типа | использовать `into_iter().map().collect()`; это уже даёт size hint, отдельно проверить отсутствие realloc | Keep/P1 verification |
| `PgnStartsWithZero::{start,end}` и pagination query methods | числовые wrappers | не менять | Keep |
| `Order::{to_sc_str,to_ucc_str}`, `EqOprtr::to_query_str` | если возвращают owning short strings, возможна аллокация на query | рассмотреть wrapper над `&'static str` только как отдельное публичное API-решение; пока оптимизировать caller, записывая Display прямо в query | P1; не ломать API |
| SQL `qb`/`qp` implementations около строк 635, 932, 1133, 1273, 1609 | repeated fragment building и error conversion | передавать единый mutable builder; исключить создание fragment с последующим append там, где trait позволяет | P0; R4/R5 и bind order tests |
| `string_test_cases_vec`, `uuid_uuid_test_cases_vec` | test-data allocation | вне runtime scope | Keep |

### 5.7. `pg_crud/wh_flts/src/lib.rs`

| Функция/метод | Наблюдение | План | Приоритет и проверка |
|---|---|---|---|
| `From<RegexRgx> for String` | consuming conversion сейчас копирует через `as_str().to_owned()` | если wrapper владеет `String`, добавить/использовать consuming `into_inner`, чтобы переместить буфер; сохранить публичную конверсию | P0; pointer/capacity unit test допустим только локально |
| `RgxCase::postgreql_syntax` | короткий SQL token | возвращать/писать static fragment без нового `String`, если текущий wrapper это допускает без API change | P1; display/query tests |
| `PgTypeNotEmptyUnqVec::{try_from,try_from_by_hash}` | может повторять uniqueness allocation | делегировать consuming Vec в один validation path; HashSet pre-size | P1; R5 |
| `PgTypeNotEmptyUnqVec::into_vec`, `WhIn::into_inn` | consuming методы должны перемещать Vec | сохранить; проверить callers на clone перед вызовом | P0 call-site audit |
| `WhIn::{pg_type_qp,pg_type_qp_minus_one,qb,qp}` | строят placeholders пропорционально длине списка | единый pre-sized query buffer; `write!` числа placeholder прямо в него; bind values borrowed | P0; R5 и bind-order tests |
| regex/sqlx error conversion | `er.to_string()` нужен owned domain error | холодный путь; не оптимизировать до профиля | Keep |

### 5.8. `pg_types_*`, `config_lib`, `loc_lib`, `naming`

| Модуль и функция | План | Приоритет |
|---|---|---|
| `pg_types_cmn`, `pg_types_numeric`, `pg_types_chrono_net`, `pg_types_text_misc` conversions | найти `From<&T>` на owning wrappers в runtime paths; предпочесть consuming `From<T>` у callers, borrowing оставить для boundary; не менять generated public API | P1 |
| `config_lib/src/lib.rs` и `types.rs`: env parsing | env API уже возвращает owned `String`; перемещать её в wrapper, не делать `to_owned`; format error только при ошибке | P2/startup |
| `loc_lib/src/loc.rs`: constructors | `file.as_ref().to_owned()` и commit clone допустимы для owned diagnostic; проверить возможность consuming constructor при массовой генерации ошибок | P2 |
| `loc_lib::DatetimeFmt::fmt` | писать chrono formatting прямо в formatter; не вызывать `to_string` в production caller | P2 |
| `naming/src/lib.rs`: swagger path generation | заменить вложенные `format!` + quote helpers одним write в итоговый buffer; статические части хранить borrowed | P1 compile/startup |
| `to_err_string` implementations | owned error string является контрактом; оптимизировать только repeated intermediate formatting, не lifetime | Keep/P2 |

## 6. Proc-macro и генераторы

Главный принцип этой группы: не превращать `TokenStream` в `String` и обратно, если можно построить токены через `quote!`, `format_ident!`, `syn::parse2` или реализовать `ToTokens`. Самые дорогие места — повторная материализация больших `Vec<TokenStream>`, клонирование `syn`-деревьев и вложенные `quote!` внутри итераций.

### 6.1. `pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs`

| Функция/stage | План | Приоритет и проверка |
|---|---|---|
| `gen_pg_tbl` | снять C2 baseline по каждому stage; не оптимизировать монолит вслепую | P1 |
| `parse_gen_pg_tbl_input_stage` | сериализованную config-часть парсить один раз; не повторять `TokenStream::to_string`; перемещать parsed model дальше | P1; malformed/valid fixtures |
| `gen_pg_tbl_field_model_stage` | хранить ссылки на `syn::Field`/attrs, где model не переживает input; если lifetime усложняет публичный API, оставить owned AST и оптимизировать только clones | P1 |
| `gen_pg_tbl_variant_field_model_stage` | общие ident/type tokens вычислять один раз на поле; передавать refs в variant emission | P1; token equality |
| `gen_pg_tbl_syn_field_loc_attr_stage` | не клонировать полный `syn::Field` ради location attr; извлекать только необходимые borrowed части | P1 |
| `gen_pg_tbl_variant_model_stage` | заранее задать capacity коллекциям по числу fields × variants; не хранить одновременно промежуточные strings и tokens | P1; peak RSS C2 |
| `build_gen_pg_tbl_fields_model_stage` | использовать iterator/`Vec::with_capacity(fields.len())`; перемещать field models | P1 |
| `build_gen_pg_tbl_input_model_stage` | один owning root model; shared immutable metadata передавать по ссылке, не `Arc` | P1 |
| `validate_gen_pg_tbl_fields_model_stage` | собирать только первую ошибку либо писать все ошибки в один pre-sized buffer согласно текущей семантике | P2; exact diagnostics |
| `emit_gen_pg_tbl_tests_stage` | генерировать token iterators непосредственно внутри `quote!`; не собирать `Vec<TokenStream>` без повторного использования | P1 |
| `emit_gen_pg_tbl_final_stage` | append в один `TokenStream` через `extend`; освобождать stage models до форматирования/записи файла | P1; byte/token-equivalent expansion |
| generated SQL/route/client methods | применить runtime-рекомендации из разделов 5.2–5.7 к коду внутри `quote!`; это важнее compile-time micro-allocations | P0; generated tests + R4/R5 |

### 6.2. `pg_crud/pg_types/gen_pg_types_src/src/lib.rs`

| Функция/область внутри `gen_pg_types` | Наблюдение и план | Приоритет |
|---|---|---|
| input parse около строки 534 | один `input_ts.as_ref().to_string()` неизбежен для JSON config; не повторять и освободить строку сразу после parse | P1 |
| record collection около строк 637/685 | capacity уже используется; проверить точность и заменить fold на `map/collect` только если allocator count не ухудшается | P1 verification |
| clone emission около строки 764 | `dot_clone_ts` означает generated runtime clone; классифицировать по типам: Copy — `*v`/copy, owned input consumed — move, borrowed boundary — clone только при необходимости | P0 generated runtime |
| identifier creation около строк 785–822 | убрать `format!("{value}")` и `ident.to_string()` перед `format_ident!`, передавать Display/ident fragments напрямую | P1 C1 |
| OpenAPI custom formats около строк 955/962 | строки создаются при schema build; проверить API utoipa на `Cow`/static input, не менять версию/dependency | P2 |
| sqlx bind errors около строк 1118/1124 | error-only allocation оставить | Keep |
| serialization generation около строки 1295 | UUID/date numeric serializers должны писать в serializer без промежуточной `String`, если serde API поддерживает primitive/string view; подтвердить формат | P0 runtime |
| chrono/date error payloads около строк 2608–2631 | owned strings нужны только на ошибке; не оптимизировать раньше success path | Keep/P2 |
| emptiness check около строки 2826 | `self.0.to_string().is_empty()` аллоцирует; генерировать прямую проверку для string-like типов либо formatter-free predicate | P0 |
| parse errors около строк 3058–3144 | холодный путь; объединить только повторное форматирование одного error | P2 |
| финальный `ToErrString` generation | избегать `self.to_string()` только если можно писать напрямую в единственный итоговый `String`; контракт остаётся owned | P2 |

### 6.3. `pg_crud/wh_flts/gen_wh_flts_src/src/lib.rs`

| Область внутри `gen_wh_flts` | План | Приоритет |
|---|---|---|
| config parse | материализовать input string один раз | P1 |
| generated `try_bind` | не делать `value.to_string()` для типов, которые sqlx принимает напрямую; bind borrow/value согласно sqlx lifetime | P0; DB-independent compile tests + bind model tests |
| placeholder builders с capacity 32 | capacity вычислять из количества values и текущего placeholder index; убрать realloc для больших IN | P0; C3/R5 |
| list builder около строки 475 | `len * 8` заменить оценкой, учитывающей число цифр placeholder; один buffer | P0 |
| operator branches около строк 709–722 | `IsNull` и другие static tokens писать в accumulator без `to_string` | P0 |
| repeated `format!` to `dq_ts` | добавить общий helper, который принимает `fmt::Arguments` или формирует token literal один раз; shared logic разместить в существующем shared crate, не дублировать | P1 |
| generated error conversions | оставить cold-path `to_string` | Keep |

### 6.4. `workspace_macro_helpers`, `macros_helpers`, `naming` и macro wrappers

| Функция/метод | План | Приоритет |
|---|---|---|
| `workspace_macro_helpers::split_top_level_commas` | `Vec::new` заменить capacity только после дешёвого top-level estimate; не делать предварительный полный clone token stream | P2 |
| `first_ident`, `first_ident_at` | `Ident::to_string` неизбежен только для owned domain wrapper; где достаточно сравнения, сравнивать ident напрямую | P1 |
| `compile_error_ts` | owned message нужен token literal; сохранить одну копию | Keep |
| `macros_helpers::write_ts_into_file::try_write_ts_into_file` | `TokenStream::to_string()` создаёт один большой buffer, необходимый для файла; передать его consuming в write path и не вычислять повторно | P1; peak RSS large fixture |
| `write_string_into_file::{should_write_string_into_file,should_write_with_same_len_diff,should_write_with_diff_len}` | сейчас чтение старого файла может удвоить peak memory; сравнивать потоково по chunks/metadata, сохраняя точную семантику, затем писать новый buffer | P1; equal/same-length-diff/diff-length tests |
| `naming_cmn::{display_case_str,tokenized_case_str,str_case}` | избежать цепочки `TokenStream -> String -> case String -> TokenStream`; разделить borrowed string conversion и token conversion | P1 |
| `naming_macros::gen_ucc_and_sc_str_and_ts` | folds начинать с рассчитанной capacity; повторно использовать вычисленные case strings; формировать identifiers через `format_ident!` | P1 |
| `naming_macros::gen_self_ucc_and_sc_str_and_ts` | устранить повторные конкатенации одних частей и `v.to_string()` в generated runtime methods | P1/P0 generated runtime |
| `gen_impl_trait_for_ident_ts` и enum conversion generators | передавать token iterators в `quote!`, не собирать временные Vec, если один проход | P1 |
| `macro_clippy_check_cmn::clippy_check` | строит несколько полных копий Cargo.toml; заменить цепочки `collect<String>`/`replace` одним pre-sized output writer; освободить buffers до запуска cargo | P2/T1 |
| `pg_crud_cmn_macros`, `config_lib_macros`, `server_app_state_macros`, `newtype`, `token_patterns_macros`, `loc_macros` entrypoints | убрать только подтверждённые `input.to_string()` roundtrips; если parser принимает token stream, использовать `syn::parse`/`parse2` | P1 |

## 7. Остальные crates: аудит и ожидаемое решение

| Crate | Действие |
|---|---|
| `app_state`, `server_app_state` | проверить, что pool/config перемещаются в state, а `Arc` создаётся ровно на уровне cross-thread router state; обычно Keep |
| `server_config`, `config_lib/try_from_env` | startup-only: consuming conversions из env strings, без дополнительного clone |
| `contract_constants`, `token_patterns`, `gen_quotes` | константы и token literals должны быть borrowed/static; `String` создавать только при требовании proc-macro API |
| `git_info` | commit/link по возможности `Cow<'static, str>`; не клонировать link на каждый health response |
| `newtype` | generated `From<String>` должен перемещать String; `From<&str>` аллоцирует осознанно; generated `AsRef` без аллокаций |
| `optml` | проверить folds/collect и capacity по известному числу элементов; низкий приоритет до профиля |
| `panic_loc`, `loc_lib` | диагностические строки — cold path; сохранить точность сообщений |
| `pg_crud_macros_cmn`, `pg_crud_cmn_macros` | общий emit-код оптимизировать раньше отдельных генераторов, если helper вызывается многократно |
| `gen_getter_traits_for_struct_fields`, `gen_derive_ts_builder` | identifiers строить без `format!().parse()`; коллекциям дать capacity `fields.len()` |
| `macro_clippy_check_cmn`, `workspace_test_runner` | снижать peak CI памяти после runtime и основных generators |
| `tests`, `*_test`, `loc_test` | не оптимизировать обычные fixture allocations; оптимизировать только полное чтение workspace/снимков и повторные cargo process buffers |

## 8. Последовательность реализации

### Этап 0. Baseline

1. Зафиксировать toolchain, profile и hardware.
2. Реализовать локальный allocation counter только в benchmark/test binary либо использовать установленный heap profiler.
3. Выполнить R1–R5, C1–C3 и T1.
4. Сохранить таблицу baseline и top-20 allocation stacks.

### Этап 1. P0 runtime

1. `server_runtime::RequestIdLayer::call`.
2. SQL builders в `pg_tbl`.
3. generated query/bind paths в `gen_pg_tbl_src`, `gen_pg_types_src`, `gen_wh_flts_src`.
4. `wh_flts` IN/list placeholder construction.
5. 404/status payload construction в `cmn_routes`.

После каждого модуля повторить только релевантные R-сценарии и обязательные тесты. Не объединять несвязанные оптимизации в один diff.

### Этап 2. P1 macro expansion

1. Убрать TokenStream/String roundtrips в shared macro helpers.
2. Сократить owned промежуточные models в `gen_pg_tbl_src`.
3. Сократить повторную генерацию identifier/case strings в `gen_pg_types_src` и `naming_macros`.
4. Предварительно задавать capacity коллекциям с известным размером.
5. Освобождать крупные AST/models до записи и запуска rustfmt/clippy.

Повторить C1–C3 и сравнить peak RSS всего дочернего cargo process tree, а не только proc-macro процесса.

### Этап 3. P1/P2 infrastructure

1. Потоковое сравнение generated files в `macros_helpers`.
2. Сборка временного Cargo.toml в `macro_clippy_check_cmn` одним buffer.
3. Startup config/location/error paths — только если остаются в top allocation stacks.
4. Удалить экспериментальные benchmark hooks, которые не являются постоянными тестами.

## 9. Проверка каждого изменения

Обязательный локальный набор:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests code_style
```

Дополнительно по затронутой области:

```bash
cargo test -p server_runtime
cargo test -p server_tbl_example
cargo test -p gen_pg_tbl_test
cargo test -p gen_pg_types_test
cargo test -p gen_wh_flts_test
```

Для генераторов сравнить:

- публичные типы и сигнатуры;
- получившийся `TokenStream` после нормализации форматирования;
- SQL строка byte-for-byte;
- порядок sqlx bind-параметров;
- JSON roundtrip и тексты отрицательных ошибок;
- OpenAPI/route parity;
- compile-fail diagnostics, включая существующие 8-символьные ID.

## 10. Риски и стоп-условия

- Не заменять `String` на borrowed field в публичном domain type без отдельного API решения: это протечёт lifetime/generics пользователям.
- Не применять `Cow` повсеместно: он увеличивает сложность типов и полезен только при высокой доле borrowed/static значений.
- Не вводить `Arc<str>` для устранения clone, пока нет cross-thread sharing нескольких владельцев.
- Не кешировать результаты в `static`/глобальных maps: это создаёт неограниченное удержание памяти.
- Не использовать `unsafe`, custom arenas и interning без отдельной задачи и измеренного обоснования.
- Не заменять линейный проход предварительным проходом только ради точной capacity, если CPU-регрессия превышает выигрыш памяти.
- Остановить конкретную оптимизацию, если выигрыш ниже порога, публичный API меняется либо диагностика/SQL/JSON перестают совпадать.

## 11. Формат отчёта по модулю

Для каждого реализованного пункта дополнять PR следующей таблицей:

| Поле | Значение |
|---|---|
| crate/module/function | точный путь и имя |
| scenario | R1–R5/C1–C3/T1 |
| before | allocations, allocated bytes, peak RSS, time |
| after | те же метрики |
| delta | абсолютное и процентное изменение |
| semantic evidence | названия тестов/golden snapshots |
| trade-off | CPU, читаемость, capacity over-allocation |
| decision | принять, отклонить или собрать больше данных |

## 12. Журнал выполнения

Измерения выполняются на `rustc 1.99.0-nightly (77cf889bc 2026-07-12)`, Linux x86_64. Для сравнений используется один toolchain, одинаковый `release` profile и один и тот же временный executable; profiling hooks после измерения удаляются.

### R4 — базовые CRUD SQL builders

Сценарий: 100 000 последовательных вызовов каждого из восьми public CRUD builders, всего 800 000 запросов. Baseline снят с `HEAD` в отдельном detached worktree, after — с текущей реализацией. Время — медиана 10 прогонов после сборки и прогрева; аллокации — `heaptrack`; peak heap дополнительно проверен Valgrind Massif.

| Метрика | До | После | Изменение |
|---|---:|---:|---:|
| вызовы allocator | 2 100 010 | 800 010 | −61,90% |
| выделено суммарно | 263 676 348 B | 88 376 348 B | −66,48% |
| временные аллокации | 1 400 002 | 100 002 | −92,86% |
| peak heap (`heaptrack`) | 75,83 KiB | 75,46 KiB | −0,49% |
| peak heap (`massif`, без profiler overhead) | 2 098 B | 1 736 B | −17,25% |
| медиана wall time | 76,5 ms | 10,2 ms | −86,67% |

Решение: принять. `gen_insert_query_string`, `gen_select_query_string`, `gen_update_query_string` и `gen_delete_query_string` теперь выделяют один заранее рассчитанный итоговый буфер. Семантика подтверждается 15 byte-for-byte SQL unit-тестами.

### Реализованные P0/P1/P2 пункты

- `server_runtime::RequestIdLayer::call`: удалены owned-копии method/path и повторная `String` request ID; один `HeaderValue` сохраняется до формирования response. Добавлен тест обеих ветвей request ID, все 5 тестов crate проходят.
- `gen_pg_tbl_src`: metric handles создаются один раз при построении routes, а не на каждый запрос; сортировка удаления и primary-key tie-break пишутся без промежуточных строк; имена операций создаются через `format_ident!` без `String -> TokenStream` parse.
- `gen_pg_types_src`: UUID сериализуется напрямую, UUID/MAC emptiness не материализует строку, устранены промежуточные identifier strings.
- `gen_wh_flts_src` и `pg_crud_cmn`: equality/IN placeholders, скобки и logical operators пишутся в один accumulator без промежуточных SQL fragments.
- `macros_helpers::write_string_into_file`: unchanged generated files сравниваются chunks по 8192 bytes без второй полной копии файла; добавлены тесты large equal и late difference.
- `macro_clippy_check_cmn`: временный Cargo.toml собирается одним buffer, borrowed строки представлены `Cow`, path rewrite выполняется in-place.
- `naming_macros`, `gen_derive_ts_builder`, `gen_getter_traits_for_struct_fields`: regex создаётся один раз, case conversion выполняется одним проходом, identifiers создаются через `format_ident!`; удалена ставшая неиспользуемой зависимость `proc-macro2`.

### Проверки

- `cargo fmt` — успешно.
- `cargo clippy --all-targets --all-features -- -D warnings` — успешно.
- generated-code clippy tests `gen_pg_tbl_test`, `gen_pg_types_test`, `gen_wh_flts_test` — успешно.
- unit tests `server_runtime`, `pg_tbl`, `pg_crud_cmn`, `macros_helpers`, `gen_derive_ts_builder` — успешно.
- `cargo test -p tests code_style`: 40 из 42 независимых проверок проходят после устранения runtime `expect` и синхронизации стабильных Clippy lints. Два оставшихся ограничения не относятся к оптимизированному коду: rust lint-sync требует четыре unstable lint, которые сам rustc отклоняет без `strict_provenance_lints`, а English-only policy отклоняет этот предоставленный русскоязычный план.
