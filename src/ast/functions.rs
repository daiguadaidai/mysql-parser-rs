// TimeUnitType is the type for time and timestamp units.
#[derive(Debug)]
pub enum TimeUnitType {
    // TimeUnitInvalid is a placeholder for an invalid time or timestamp unit
    TimeUnitInvalid,
    // TimeUnitMicrosecond is the time or timestamp unit MICROSECOND.
    TimeUnitMicrosecond,
    // TimeUnitSecond is the time or timestamp unit SECOND.
    TimeUnitSecond,
    // TimeUnitMinute is the time or timestamp unit MINUTE.
    TimeUnitMinute,
    // TimeUnitHour is the time or timestamp unit HOUR.
    TimeUnitHour,
    // TimeUnitDay is the time or timestamp unit DAY.
    TimeUnitDay,
    // TimeUnitWeek is the time or timestamp unit WEEK.
    TimeUnitWeek,
    // TimeUnitMonth is the time or timestamp unit MONTH.
    TimeUnitMonth,
    // TimeUnitQuarter is the time or timestamp unit QUARTER.
    TimeUnitQuarter,
    // TimeUnitYear is the time or timestamp unit YEAR.
    TimeUnitYear,
    // TimeUnitSecondMicrosecond is the time unit SECOND_MICROSECOND.
    TimeUnitSecondMicrosecond,
    // TimeUnitMinuteMicrosecond is the time unit MINUTE_MICROSECOND.
    TimeUnitMinuteMicrosecond,
    // TimeUnitMinuteSecond is the time unit MINUTE_SECOND.
    TimeUnitMinuteSecond,
    // TimeUnitHourMicrosecond is the time unit HOUR_MICROSECOND.
    TimeUnitHourMicrosecond,
    // TimeUnitHourSecond is the time unit HOUR_SECOND.
    TimeUnitHourSecond,
    // TimeUnitHourMinute is the time unit HOUR_MINUTE.
    TimeUnitHourMinute,
    // TimeUnitDayMicrosecond is the time unit DAY_MICROSECOND.
    TimeUnitDayMicrosecond,
    // TimeUnitDaySecond is the time unit DAY_SECOND.
    TimeUnitDaySecond,
    // TimeUnitDayMinute is the time unit DAY_MINUTE.
    TimeUnitDayMinute,
    // TimeUnitDayHour is the time unit DAY_HOUR.
    TimeUnitDayHour,
    // TimeUnitYearMonth is the time unit YEAR_MONTH.
    TimeUnitYearMonth,
}

// List scalar function names.
pub const LOGIC_AND: &str = "and";
pub const CAST: &str = "cast";
pub const LEFT_SHIFT: &str = "leftshift";
pub const RIGHT_SHIFT: &str = "rightshift";
pub const LOGIC_OR: &str = "or";
pub const GE: &str = "ge";
pub const LE: &str = "le";
pub const EQ: &str = "eq";
pub const NE: &str = "ne";
pub const LT: &str = "lt";
pub const GT: &str = "gt";
pub const PLUS: &str = "plus";
pub const MINUS: &str = "minus";
pub const AND: &str = "bitand";
pub const OR: &str = "bitor";
pub const MOD: &str = "mod";
pub const XOR: &str = "bitxor";
pub const DIV: &str = "div";
pub const MUL: &str = "mul";
pub const UNARY_NOT: &str = "not"; // Avoid name conflict with Not in github/pingcap/check.
pub const BIT_NEG: &str = "bitneg";
pub const INT_DIV: &str = "intdiv";
pub const LOGIC_XOR: &str = "xor";
pub const NULL_EQ: &str = "nulleq";
pub const UNARY_PLUS: &str = "unaryplus";
pub const UNARY_MINUS: &str = "unaryminus";
pub const IN: &str = "in";
pub const LIKE: &str = "like";
pub const ILIKE: &str = "ilike";
pub const CASE: &str = "case";
pub const REGEXP: &str = "regexp";
pub const REGEXP_LIKE: &str = "regexp_like";
pub const REGEXP_SUBSTR: &str = "regexp_substr";
pub const REGEXP_IN_STR: &str = "regexp_instr";
pub const REGEXP_REPLACE: &str = "regexp_replace";
pub const IS_NULL: &str = "isnull";
pub const IS_TRUTH_WITHOUT_NULL: &str = "istrue"; // Avoid name conflict with IsTrue in github/pingcap/check.
pub const IS_TRUTH_WITH_NULL: &str = "istrue_with_null";
pub const IS_FALSITY: &str = "isfalse"; // Avoid name conflict with IsFalse in github/pingcap/check.
pub const ROW_FUNC: &str = "row";
pub const SET_VAR: &str = "setvar";
pub const GET_VAR: &str = "getvar";
pub const VALUES: &str = "values";
pub const BIT_COUNT: &str = "bit_count";
pub const GET_PARAM: &str = "getparam";
// common functions
pub const COALESCE: &str = "coalesce";
pub const GREATEST: &str = "greatest";
pub const LEAST: &str = "least";
pub const INTERVAL: &str = "interval";
// math functions
pub const ABS: &str = "abs";
pub const ACOS: &str = "acos";
pub const ASIN: &str = "asin";
pub const ATAN: &str = "atan";
pub const ATAN2: &str = "atan2";
pub const CEIL: &str = "ceil";
pub const CEILING: &str = "ceiling";
pub const CONV: &str = "conv";
pub const COS: &str = "cos";
pub const COT: &str = "cot";
pub const CRC32: &str = "crc32";
pub const DEGREES: &str = "degrees";
pub const EXP: &str = "exp";
pub const FLOOR: &str = "floor";
pub const LN: &str = "ln";
pub const LOG: &str = "log";
pub const LOG2: &str = "log2";
pub const LOG10: &str = "log10";
pub const PI: &str = "pi";
pub const POW: &str = "pow";
pub const POWER: &str = "power";
pub const RADIANS: &str = "radians";
pub const RAND: &str = "rand";
pub const ROUND: &str = "round";
pub const SIGN: &str = "sign";
pub const SIN: &str = "sin";
pub const SQRT: &str = "sqrt";
pub const TAN: &str = "tan";
pub const TRUNCATE: &str = "truncate";
// time functions
pub const ADD_DATE: &str = "adddate";
pub const ADD_TIME: &str = "addtime";
pub const CONVERT_TZ: &str = "convert_tz";
pub const CURDATE: &str = "curdate";
pub const CURRENT_DATE: &str = "current_date";
pub const CURRENT_TIME: &str = "current_time";
pub const CURRENT_TIMESTAMP: &str = "current_timestamp";
pub const CURTIME: &str = "curtime";
pub const DATE: &str = "date";
pub const DATE_LITERAL: &str = "'tidb`.(dateliteral";
pub const DATE_ADD: &str = "date_add";
pub const DATE_FORMAT: &str = "date_format";
pub const DATE_SUB: &str = "date_sub";
pub const DATE_DIFF: &str = "datediff";
pub const DAY: &str = "day";
pub const DAY_NAME: &str = "dayname";
pub const DAY_OF_MONTH: &str = "dayofmonth";
pub const DAY_OF_WEEK: &str = "dayofweek";
pub const DAY_OF_YEAR: &str = "dayofyear";
pub const EXTRACT: &str = "extract";
pub const FROM_DAYS: &str = "from_days";
pub const FROM_UNIX_TIME: &str = "from_unixtime";
pub const GET_FORMAT: &str = "get_format";
pub const HOUR: &str = "hour";
pub const LOCAL_TIME: &str = "localtime";
pub const LOCAL_TIMESTAMP: &str = "localtimestamp";
pub const MAKE_DATE: &str = "makedate";
pub const MAKE_TIME: &str = "maketime";
pub const MICRO_SECOND: &str = "microsecond";
pub const MINUTE: &str = "minute";
pub const MONTH: &str = "month";
pub const MONTH_NAME: &str = "monthname";
pub const NOW: &str = "now";
pub const PERIOD_ADD: &str = "period_add";
pub const PERIOD_DIFF: &str = "period_diff";
pub const QUARTER: &str = "quarter";
pub const SEC_TO_TIME: &str = "sec_to_time";
pub const SECOND: &str = "second";
pub const STR_TO_DATE: &str = "str_to_date";
pub const SUB_DATE: &str = "subdate";
pub const SUB_TIME: &str = "subtime";
pub const SYSDATE: &str = "sysdate";
pub const TIME: &str = "time";
pub const TIME_LITERAL: &str = "'tidb`.(timeliteral";
pub const TIME_FORMAT: &str = "time_format";
pub const TIME_TO_SEC: &str = "time_to_sec";
pub const TIME_DIFF: &str = "timediff";
pub const TIMESTAMP: &str = "timestamp";
pub const TIMESTAMP_LITERAL: &str = "'tidb`.(timestampliteral";
pub const TIMESTAMP_ADD: &str = "timestampadd";
pub const TIMESTAMP_DIFF: &str = "timestampdiff";
pub const TO_DAYS: &str = "to_days";
pub const TO_SECONDS: &str = "to_seconds";
pub const UNIX_TIMESTAMP: &str = "unix_timestamp";
pub const UTC_DATE: &str = "utc_date";
pub const UTC_TIME: &str = "utc_time";
pub const UTC_TIMESTAMP: &str = "utc_timestamp";
pub const WEEK: &str = "week";
pub const WEEKDAY: &str = "weekday";
pub const WEEK_OF_YEAR: &str = "weekofyear";
pub const YEAR: &str = "year";
pub const YEAR_WEEK: &str = "yearweek";
pub const LAST_DAY: &str = "last_day";
// TSO functions
// TiDBBoundedStaleness is used to determine the TS for a read only request with the given bounded staleness.
// It will be used in the Stale Read feature.
// For more info, please see AsOfClause.
pub const TI_DB_BOUNDED_STALENESS: &str = "tidb_bounded_staleness";
pub const TI_DB_PARSE_TSO: &str = "tidb_parse_tso";
pub const TI_DB_PARSE_TSO_LOGICAL: &str = "tidb_parse_tso_logical";
pub const TI_DB_CURRENT_TSO: &str = "tidb_current_tso";

// string functions
pub const ASCII: &str = "ascii";
pub const BIN: &str = "bin";
pub const CONCAT: &str = "concat";
pub const CONCAT_WS: &str = "concat_ws";
pub const CONVERT: &str = "convert";
pub const ELT: &str = "elt";
pub const EXPORT_SET: &str = "export_set";
pub const FIELD: &str = "field";
pub const FORMAT: &str = "format";
pub const FROM_BASE64: &str = "from_base64";
pub const INSERT_FUNC: &str = "insert_func";
pub const INSTR: &str = "instr";
pub const LCASE: &str = "lcase";
pub const LEFT: &str = "left";
pub const LENGTH: &str = "length";
pub const LOAD_FILE: &str = "load_file";
pub const LOCATE: &str = "locate";
pub const LOWER: &str = "lower";
pub const LPAD: &str = "lpad";
pub const L_TRIM: &str = "ltrim";
pub const MAKE_SET: &str = "make_set";
pub const MID: &str = "mid";
pub const OCT: &str = "oct";
pub const OCTET_LENGTH: &str = "octet_length";
pub const ORD: &str = "ord";
pub const POSITION: &str = "position";
pub const QUOTE: &str = "quote";
pub const REPEAT: &str = "repeat";
pub const REPLACE: &str = "replace";
pub const REVERSE: &str = "reverse";
pub const RIGHT: &str = "right";
pub const R_TRIM: &str = "rtrim";
pub const SPACE: &str = "space";
pub const STRCMP: &str = "strcmp";
pub const SUBSTRING: &str = "substring";
pub const SUBSTR: &str = "substr";
pub const SUBSTRING_INDEX: &str = "substring_index";
pub const TO_BASE64: &str = "to_base64";
pub const TRIM: &str = "trim";
pub const TRANSLATE: &str = "translate";
pub const UPPER: &str = "upper";
pub const UCASE: &str = "ucase";
pub const HEX: &str = "hex";
pub const UNHEX: &str = "unhex";
pub const RPAD: &str = "rpad";
pub const BIT_LENGTH: &str = "bit_length";
pub const CHAR_FUNC: &str = "char_func";
pub const CHAR_LENGTH: &str = "char_length";
pub const CHARACTER_LENGTH: &str = "character_length";
pub const FIND_IN_SET: &str = "find_in_set";
pub const WEIGHT_STRING: &str = "weight_string";
pub const SOUNDEX: &str = "soundex";

// information functions
pub const BENCHMARK: &str = "benchmark";
pub const CHARSET: &str = "charset";
pub const COERCIBILITY: &str = "coercibility";
pub const COLLATION: &str = "collation";
pub const CONNECTION_ID: &str = "connection_id";
pub const CURRENT_USER: &str = "current_user";
pub const CURRENT_ROLE: &str = "current_role";
pub const DATABASE: &str = "database";
pub const FOUND_ROWS: &str = "found_rows";
pub const LAST_INSERT_ID: &str = "last_insert_id";
pub const ROW_COUNT: &str = "row_count";
pub const SCHEMA: &str = "schema";
pub const SESSION_USER: &str = "session_user";
pub const SYSTEM_USER: &str = "system_user";
pub const USER: &str = "user";
pub const VERSION: &str = "version";
pub const TI_DB_VERSION: &str = "tidb_version";
pub const TI_DB_IS_DDL_OWNER: &str = "tidb_is_ddl_owner";
pub const TI_DB_DECODE_PLAN: &str = "tidb_decode_plan";
pub const TI_DB_DECODE_BINARY_PLAN: &str = "tidb_decode_binary_plan";
pub const TI_DB_DECODE_SQL_DIGESTS: &str = "tidb_decode_sql_digests";
pub const TI_DB_ENCODE_SQL_DIGEST: &str = "tidb_encode_sql_digest";
pub const FORMAT_BYTES: &str = "format_bytes";
pub const FORMAT_NANO_TIME: &str = "format_nano_time";
pub const CURRENT_RESOURCE_GROUP: &str = "current_resource_group";

// control functions
pub const IF: &str = "if";
pub const IFNULL: &str = "ifnull";
pub const NULLIF: &str = "nullif";

// miscellaneous functions
pub const ANY_VALUE: &str = "any_value";
pub const DEFAULT_FUNC: &str = "default_func";
pub const INET_ATON: &str = "inet_aton";
pub const INET_NTOA: &str = "inet_ntoa";
pub const INET6_ATON: &str = "inet6_aton";
pub const INET6_NTOA: &str = "inet6_ntoa";
pub const IS_FREE_LOCK: &str = "is_free_lock";
pub const IS_I_PV4: &str = "is_ipv4";
pub const IS_I_PV4_COMPAT: &str = "is_ipv4_compat";
pub const IS_I_PV4_MAPPED: &str = "is_ipv4_mapped";
pub const IS_I_PV6: &str = "is_ipv6";
pub const IS_USED_LOCK: &str = "is_used_lock";
pub const IS_UUID: &str = "is_uuid";
pub const MASTER_POS_WAIT: &str = "master_pos_wait";
pub const NAME_CONST: &str = "name_const";
pub const RELEASE_ALL_LOCKS: &str = "release_all_locks";
pub const SLEEP: &str = "sleep";
pub const UUID: &str = "uuid";
pub const UUID_SHORT: &str = "uuid_short";
pub const UUID_TO_BIN: &str = "uuid_to_bin";
pub const BIN_TO_UUID: &str = "bin_to_uuid";
pub const VITESS_HASH: &str = "vitess_hash";
pub const TI_DB_SHARD: &str = "tidb_shard";
pub const TI_DB_ROW_CHECKSUM: &str = "tidb_row_checksum";
pub const GET_LOCK: &str = "get_lock";
pub const RELEASE_LOCK: &str = "release_lock";
pub const GROUPING: &str = "grouping";

// encryption and compression functions
pub const AES_DECRYPT: &str = "aes_decrypt";
pub const AES_ENCRYPT: &str = "aes_encrypt";
pub const COMPRESS: &str = "compress";
pub const DECODE: &str = "decode";
pub const DES_DECRYPT: &str = "des_decrypt";
pub const DES_ENCRYPT: &str = "des_encrypt";
pub const ENCODE: &str = "encode";
pub const ENCRYPT: &str = "encrypt";
pub const MD5: &str = "md5";
pub const OLD_PASSWORD: &str = "old_password";
pub const PASSWORD_FUNC: &str = "password_func";
pub const RANDOM_BYTES: &str = "random_bytes";
pub const SHA1: &str = "sha1";
pub const SHA: &str = "sha";
pub const SHA2: &str = "sha2";
pub const SM3: &str = "sm3";
pub const UNCOMPRESS: &str = "uncompress";
pub const UNCOMPRESSED_LENGTH: &str = "uncompressed_length";
pub const VALIDATE_PASSWORD_STRENGTH: &str = "validate_password_strength";

// json functions
pub const JSON_TYPE: &str = "json_type";
pub const JSON_EXTRACT: &str = "json_extract";
pub const JSON_UNQUOTE: &str = "json_unquote";
pub const JSON_ARRAY: &str = "json_array";
pub const JSON_OBJECT: &str = "json_object";
pub const JSON_MERGE: &str = "json_merge";
pub const JSON_SET: &str = "json_set";
pub const JSON_INSERT: &str = "json_insert";
pub const JSON_REPLACE: &str = "json_replace";
pub const JSON_REMOVE: &str = "json_remove";
pub const JSON_OVERLAPS: &str = "json_overlaps";
pub const JSON_CONTAINS: &str = "json_contains";
pub const JSON_MEMBER_OF: &str = "json_memberof";
pub const JSON_CONTAINS_PATH: &str = "json_contains_path";
pub const JSON_VALID: &str = "json_valid";
pub const JSON_ARRAY_APPEND: &str = "json_array_append";
pub const JSON_ARRAY_INSERT: &str = "json_array_insert";
pub const JSON_MERGE_PATCH: &str = "json_merge_patch";
pub const JSON_MERGE_PRESERVE: &str = "json_merge_preserve";
pub const JSON_PRETTY: &str = "json_pretty";
pub const JSON_QUOTE: &str = "json_quote";
pub const JSON_SCHEMA_VALID: &str = "json_schema_valid";
pub const JSON_SEARCH: &str = "json_search";
pub const JSON_STORAGE_FREE: &str = "json_storage_free";
pub const JSON_STORAGE_SIZE: &str = "json_storage_size";
pub const JSON_DEPTH: &str = "json_depth";
pub const JSON_KEYS: &str = "json_keys";
pub const JSON_LENGTH: &str = "json_length";

// vector functions (tidb extension)
pub const VEC_DIMS: &str = "vec_dims";
pub const VEC_L1_DISTANCE: &str = "vec_l1_distance";
pub const VEC_L2_DISTANCE: &str = "vec_l2_distance";
pub const VEC_NEGATIVE_INNER_PRODUCT: &str = "vec_negative_inner_product";
pub const VEC_COSINE_DISTANCE: &str = "vec_cosine_distance";
pub const VEC_L2_NORM: &str = "vec_l2_norm";
pub const VEC_FROM_TEXT: &str = "vec_from_text";
pub const VEC_AS_TEXT: &str = "vec_as_text";

// TiDB internal function.
pub const TI_DB_DECODE_KEY: &str = "tidb_decode_key";
pub const TI_DBMVCC_INFO: &str = "tidb_mvcc_info";
pub const TI_DB_ENCODE_RECORD_KEY: &str = "tidb_encode_record_key";
pub const TI_DB_ENCODE_INDEX_KEY: &str = "tidb_encode_index_key";
pub const TI_DB_DECODE_BASE64_KEY: &str = "tidb_decode_base64_key";

// MVCC information fetching function.
pub const GET_MVCC_INFO: &str = "get_mvcc_info";

// Sequence function.
pub const NEXT_VAL: &str = "nextval";
pub const LAST_VAL: &str = "lastval";
pub const SET_VAL: &str = "setval";
