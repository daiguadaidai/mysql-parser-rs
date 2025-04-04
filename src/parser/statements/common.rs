use crate::ast::ci_str::CIStr;
use crate::ast::column_name::ColumnName;
use crate::ast::common::{
    FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE, FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE,
    FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION,
};
use crate::ast::expr_node::{ColumnNameExpr, ExprNode, ValueExpr, ValueExprKind};
use crate::ast::functions::{GetFormatSelectorType, TimeUnitType, TrimDirectionType};
use crate::ast::table_name::TableName;
use crate::charset::charset;
use crate::common::misc::is_in_correct_identifier_name;
use crate::mysql::consts::PriorityEnum;
use crate::parser::common::*;
use crate::parser::error::ErrorKind;
use crate::parser::input::Input;
use crate::parser::token_kind::TokenKind::*;
use nom::branch::alt;
use nom::combinator::map;
use nom_rule::rule;

pub fn priority(i: Input) -> IResult<PriorityEnum> {
    alt((
        map(rule!("LOW_PRIORITY"), |_| PriorityEnum::LowPriority),
        map(rule!("HIGH_PRIORITY"), |_| PriorityEnum::HighPriority),
        map(rule!("DELAYED"), |_| PriorityEnum::DelayedPriority),
    ))(i)
}

pub fn log_or(i: Input) -> IResult<()> {
    map(rule!(OR | PipesAsOr), |_| {})(i)
}

pub fn log_and(i: Input) -> IResult<()> {
    map(rule!(AND | "&&"), |_| {})(i)
}

pub fn column_name(i: Input) -> IResult<ColumnName> {
    alt((
        map(rule!(Ident), |(col_name)| {
            let name = col_name.get_trim_start_end_text('`');

            let mut cn = ColumnName::default();
            cn.name = CIStr::new(name);
            cn
        }),
        map(rule!(Ident ~ "." ~ Ident), |(tbl_name, _, col_name)| {
            let table = tbl_name.get_trim_start_end_text('`');
            let name = col_name.get_trim_start_end_text('`');

            let mut cn = ColumnName::default();
            cn.table = CIStr::new(table);
            cn.name = CIStr::new(name);
            cn
        }),
        map(
            rule!(Ident ~ "." ~ Ident ~ "." ~ Ident),
            |(schema_name, _, tbl_name, _, col_name)| {
                let schema = schema_name.get_trim_start_end_text('`');
                let table = tbl_name.get_trim_start_end_text('`');
                let name = col_name.get_trim_start_end_text('`');

                let mut cn = ColumnName::default();
                cn.schema = CIStr::new(schema);
                cn.table = CIStr::new(table);
                cn.name = CIStr::new(name);
                cn
            },
        ),
    ))(i)
}

pub fn column_name_list(i: Input) -> IResult<Vec<ColumnName>> {
    separated_list1(map(rule!(","), |_| ()), column_name)(i)
}

pub fn simple_ident(i: Input) -> IResult<ColumnNameExpr> {
    map(rule!(#column_name), |name| ColumnNameExpr { name: name })(i)
}

pub fn time_unit(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(#timestamp_unit), |(t)| t),
        map(rule!(#timestamp_unit_sql_tsi), |(t)| t),
        map(rule!(#time_unit_1), |(t)| t),
    ))(i)
}

pub fn timestamp_unit(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(MICROSECOND), |_| TimeUnitType::TimeUnitMicrosecond),
        map(rule!(SECOND), |_| TimeUnitType::TimeUnitSecond),
        map(rule!(MINUTE), |_| TimeUnitType::TimeUnitMinute),
        map(rule!(HOUR), |_| TimeUnitType::TimeUnitHour),
        map(rule!(DAY), |_| TimeUnitType::TimeUnitDay),
        map(rule!(WEEK), |_| TimeUnitType::TimeUnitWeek),
        map(rule!(MONTH), |_| TimeUnitType::TimeUnitMonth),
        map(rule!(QUARTER), |_| TimeUnitType::TimeUnitQuarter),
        map(rule!(YEAR), |_| TimeUnitType::TimeUnitYear),
    ))(i)
}

pub fn timestamp_unit_sql_tsi(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(SQL_TSI_SECOND), |_| TimeUnitType::TimeUnitSecond),
        map(rule!(SQL_TSI_MINUTE), |_| TimeUnitType::TimeUnitMinute),
        map(rule!(SQL_TSI_HOUR), |_| TimeUnitType::TimeUnitHour),
        map(rule!(SQL_TSI_DAY), |_| TimeUnitType::TimeUnitDay),
        map(rule!(SQL_TSI_WEEK), |_| TimeUnitType::TimeUnitWeek),
        map(rule!(SQL_TSI_MONTH), |_| TimeUnitType::TimeUnitMonth),
        map(rule!(SQL_TSI_QUARTER), |_| TimeUnitType::TimeUnitQuarter),
        map(rule!(SQL_TSI_YEAR), |_| TimeUnitType::TimeUnitYear),
    ))(i)
}

pub fn time_unit_1(i: Input) -> IResult<TimeUnitType> {
    alt((
        map(rule!(SECOND_MICROSECOND), |_| {
            TimeUnitType::TimeUnitSecondMicrosecond
        }),
        map(rule!(MINUTE_MICROSECOND), |_| {
            TimeUnitType::TimeUnitMinuteMicrosecond
        }),
        map(rule!(MINUTE_SECOND), |_| TimeUnitType::TimeUnitMinuteSecond),
        map(rule!(HOUR_MICROSECOND), |_| {
            TimeUnitType::TimeUnitHourMicrosecond
        }),
        map(rule!(HOUR_SECOND), |_| TimeUnitType::TimeUnitHourSecond),
        map(rule!(HOUR_MINUTE), |_| TimeUnitType::TimeUnitHourMinute),
        map(rule!(DAY_MICROSECOND), |_| {
            TimeUnitType::TimeUnitDayMicrosecond
        }),
        map(rule!(DAY_SECOND), |_| TimeUnitType::TimeUnitDaySecond),
        map(rule!(DAY_MINUTE), |_| TimeUnitType::TimeUnitDayMinute),
        map(rule!(DAY_HOUR), |_| TimeUnitType::TimeUnitDayHour),
        map(rule!(YEAR_MONTH), |_| TimeUnitType::TimeUnitYearMonth),
    ))(i)
}

pub fn fulltext_search_modifier_opt(i: Input) -> IResult<isize> {
    alt((
        map(rule!(IN ~ NATURAL ~ LANGUAGE ~ MODE), |(_, _, _, _)| {
            FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE
        }),
        map(
            rule!(IN ~ NATURAL ~ LANGUAGE ~ MODE ~ WITH ~ QUERY ~ EXPANSION),
            |(_, _, _, _, _, _, _)| {
                FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE
                    | FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION
            },
        ),
        map(rule!(IN ~ BOOLEAN ~ MODE), |(_, _, _)| {
            FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE
        }),
        map(rule!(IN ~ QUERY ~ EXPANSION), |(_, _, _)| {
            FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION
        }),
    ))(i)
}
pub fn un_reserved_keyword(i: Input) -> IResult<String> {
    map(
        rule!(#un_reserved_keyword_1 | #un_reserved_keyword_2),
        |s| s,
    )(i)
}

pub fn un_reserved_keyword_1(i: Input) -> IResult<String> {
    alt((
        map(
            rule!(
                ACTION
                    | ADVISE
                    | ASCII
                    | APPLY
                    | ATTRIBUTE
                    | ATTRIBUTES
                    | BINDING_CACHE
                    | STATS_OPTIONS
                    | STATS_SAMPLE_RATE
                    | STATS_COL_CHOICE
                    | STATS_COL_LIST
                    | AUTO_ID_CACHE
                    | AUTO_INCREMENT
                    | AFTER
                    | ALWAYS
                    | AVG
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                BDR | BEGIN
                    | BIT
                    | BOOL
                    | BOOLEAN
                    | BTREE
                    | BYTE
                    | CAPTURE
                    | CAUSAL
                    | CLEANUP
                    | CLOSE
                    | CHAIN
                    | CHARSET
                    | COLUMNS
                    | CONFIG
                    | SAN
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                COMMIT
                    | COMPACT
                    | COMPRESSED
                    | CONSISTENCY
                    | CONSISTENT
                    | CURRENT
                    | DATA
                    | DATE
                    | DATETIME
                    | DAY
                    | DEALLOCATE
                    | DO
                    | DUPLICATE
                    | DYNAMIC
                    | ENCRYPTION
                    | END
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                ENFORCED
                    | ENGINE
                    | ENGINES
                    | ENUM
                    | ERROR
                    | ERRORS
                    | ESCAPE
                    | EVOLVE
                    | EXECUTE
                    | EXTENDED
                    | FIELDS
                    | FILE
                    | FIRST
                    | FIXED
                    | FLUSH
                    | FOLLOWING
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                FORMAT
                    | FULL
                    | GENERAL
                    | GLOBAL
                    | HASH
                    | HELP
                    | HOUR
                    | INSERT_METHOD
                    | LESS
                    | LOCAL
                    | LAST
                    | NAMES
                    | NVARCHAR
                    | OFFSET
                    | PACK_KEYS
                    | PARSER
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                PASSWORD
                    | PREPARE
                    | PRE_SPLIT_REGIONS
                    | PROXY
                    | QUICK
                    | REBUILD
                    | RECOMMEND
                    | REDUNDANT
                    | REORGANIZE
                    | RESOURCE
                    | RESTART
                    | ROLE
                    | ROLLBACK
                    | ROLLUP
                    | SESSION
                    | SIGNED
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                SHARD_ROW_ID_BITS
                    | SHUTDOWN
                    | SNAPSHOT
                    | START
                    | STATUS
                    | OPEN
                    | POINT
                    | SUBPARTITIONS
                    | SUBPARTITION
                    | TABLES
                    | TABLESPACE
                    | TEXT
                    | THAN
                    | TIME
                    | TIMESTAMP
                    | TRACE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                TRANSACTION
                    | TRUNCATE
                    | TSO
                    | UNBOUNDED
                    | UNKNOWN
                    | UNSET
                    | VALUE
                    | WARNINGS
                    | YEAR
                    | MODE
                    | WEEK
                    | WEIGHT_STRING
                    | ANY
                    | SOME
                    | USER
                    | IDENTIFIED
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                COLLATION
                    | COMMENT
                    | AVG_ROW_LENGTH
                    | CONNECTION
                    | CHECKSUM
                    | COMPRESSION
                    | KEY_BLOCK_SIZE
                    | MASTER
                    | MAX_ROWS
                    | MIN_ROWS
                    | NATIONAL
                    | NCHAR
                    | ROW_FORMAT
                    | QUARTER
                    | GRANTS
                    | TRIGGERS
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                DELAY_KEY_WRITE
                    | ISOLATION
                    | JSON
                    | REPEATABLE
                    | RESPECT
                    | COMMITTED
                    | UNCOMMITTED
                    | ONLY
                    | SERIAL
                    | SERIALIZABLE
                    | LEVEL
                    | VARIABLES
                    | SQL_CACHE
                    | INDEXES
                    | PROCESSLIST
                    | SQL_NO_CACHE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                DISABLE
                    | DISABLED
                    | ENABLE
                    | ENABLED
                    | REVERSE
                    | PRIVILEGES
                    | NO
                    | BINLOG
                    | FUNCTION
                    | VIEW
                    | BINDING
                    | BINDINGS
                    | MODIFY
                    | EVENTS
                    | PARTITIONS
                    | NONE
            ),
            |t| t.text().to_string(),
        ),
    ))(i)
}

pub fn un_reserved_keyword_2(i: Input) -> IResult<String> {
    alt((
        map(
            rule!(
                NULLS
                    | SUPER
                    | EXCLUSIVE
                    | STATS_PERSISTENT
                    | STATS_AUTO_RECALC
                    | ROW_COUNT
                    | COALESCE
                    | MONTH
                    | PROCESS
                    | PROFILE
                    | PROFILES
                    | MICROSECOND
                    | MINUTE
                    | PLUGINS
                    | PRECEDING
                    | QUERY
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                QUERIES
                    | SAVEPOINT
                    | SECOND
                    | SEPARATOR
                    | SHARE
                    | SHARED
                    | SLOW
                    | MAX_CONNECTIONS_PER_HOUR
                    | MAX_QUERIES_PER_HOUR
                    | MAX_UPDATES_PER_HOUR
                    | MAX_USER_CONNECTIONS
                    | REPLICATION
                    | CLIENT
                    | SLAVE
                    | RELOAD
                    | TEMPORARY
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                ROUTINE
                    | EVENT
                    | ALGORITHM
                    | DEFINER
                    | INVOKER
                    | MERGE
                    | TEMPTABLE
                    | UNDEFINED
                    | SECURITY
                    | CASCADED
                    | RECOVER
                    | CIPHER
                    | SUBJECT
                    | ISSUER
                    | X509
                    | NEVER
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                EXPIRE
                    | ACCOUNT
                    | INCREMENTAL
                    | CPU
                    | MEMBER
                    | MEMORY
                    | BLOCK
                    | IO
                    | CONTEXT
                    | SWITCHES
                    | PAGE
                    | FAULTS
                    | IPC
                    | SWAPS
                    | SOURCE
                    | TRADITIONAL
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                SQL_BUFFER_RESULT
                    | DIRECTORY
                    | HISTOGRAM
                    | HISTORY
                    | LIST
                    | NODEGROUP
                    | SYSTEM_TIME
                    | PARTIAL
                    | SIMPLE
                    | REMOVE
                    | PARTITIONING
                    | STORAGE
                    | DISK
                    | STATS_SAMPLE_PAGES
                    | SECONDARY
                    | SECONDARY_ENGINE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                SECONDARY_LOAD
                    | SECONDARY_UNLOAD
                    | VALIDATION
                    | WITHOUT
                    | RTREE
                    | HYPO
                    | EXCHANGE
                    | COLUMN_FORMAT
                    | REPAIR
                    | IMPORT
                    | IMPORTS
                    | DISCARD
                    | TABLE_CHECKSUM
                    | UNICODE
                    | AUTO_RANDOM
                    | AUTO_RANDOM_BASE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                SQL_TSI_DAY
                    | SQL_TSI_HOUR
                    | SQL_TSI_MINUTE
                    | SQL_TSI_MONTH
                    | SQL_TSI_QUARTER
                    | SQL_TSI_SECOND
                    | LANGUAGE
                    | SQL_TSI_WEEK
                    | SQL_TSI_YEAR
                    | INVISIBLE
                    | VISIBLE
                    | TYPE
                    | NOWAIT
                    | INSTANCE
                    | REPLICA
                    | LOCATION
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                LABELS
                    | LOGS
                    | HOSTS
                    | AGAINST
                    | EXPANSION
                    | INCREMENT
                    | MINVALUE
                    | NOMAXVALUE
                    | NOMINVALUE
                    | NOCACHE
                    | CACHE
                    | CYCLE
                    | NOCYCLE
                    | SEQUENCE
                    | MAX_MINUTES
                    | MAX_IDXNUM
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                PER_TABLE
                    | PER_DB
                    | NEXT
                    | NEXTVAL
                    | LASTVAL
                    | SETVAL
                    | AGO
                    | BACKUP
                    | BACKUPS
                    | CONCURRENCY
                    | MB
                    | ONLINE
                    | RATE_LIMIT
                    | RESTORE
                    | RESTORES
                    | SEND_CREDENTIALS_TO_TIKV
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                LAST_BACKUP
                    | CHECKPOINT
                    | SKIP_SCHEMA_FILES
                    | STRICT_FORMAT
                    | BACKEND
                    | CSV_BACKSLASH_ESCAPE
                    | CSV_NOT_NULL
                    | CSV_TRIM_LAST_SEPARATORS
                    | CSV_DELIMITER
                    | CSV_HEADER
                    | CSV_NULL
                    | CSV_SEPARATOR
                    | ON_DUPLICATE
                    | TIKV_IMPORTER
                    | REPLICAS
                    | POLICY
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                WAIT | CLIENT_ERRORS_SUMMARY
                    | BERNOULLI
                    | SYSTEM
                    | PERCENT
                    | PAUSE
                    | RESUME
                    | OFF
                    | OPTIONAL
                    | REQUIRED
                    | PURGE
                    | SKIP
                    | LOCKED
                    | CLUSTER
                    | CLUSTERED
                    | NONCLUSTERED
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                PRESERVE
                    | TOKEN_ISSUER
                    | TTL
                    | TTL_ENABLE
                    | TTL_JOB_INTERVAL
                    | FAILED_LOGIN_ATTEMPTS
                    | PASSWORD_LOCK_TIME
                    | DIGEST
                    | REUSE
                    | DECLARE
                    | HANDLER
                    | FOUND
                    | CALIBRATE
                    | WORKLOAD
                    | TPCC
                    | OLTP_READ_WRITE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                OLTP_READ_ONLY
                    | OLTP_WRITE_ONLY
                    | VECTOR
                    | TPCH_10
                    | WITH_SYS_TABLE
                    | WAIT_TIFLASH_READY
                    | IGNORE_STATS
                    | LOAD_STATS
                    | CHECKSUM_CONCURRENCY
                    | COMPRESSION_LEVEL
                    | COMPRESSION_TYPE
                    | ENCRYPTION_METHOD
                    | ENCRYPTION_KEYFILE
            ),
            |t| t.text().to_string(),
        ),
    ))(i)
}

pub fn tidb_keyword(i: Input) -> IResult<String> {
    alt((
        map(
            rule!(
                ADMIN
                    | BATCH
                    | BUCKETS
                    | BUILTINS
                    | CANCEL
                    | CARDINALITY
                    | CMSKETCH
                    | COLUMN_STATS_USAGE
                    | CORRELATION
                    | DDL
                    | DEPENDENCY
                    | DEPTH
                    | JOBS
                    | JOB
                    | NODE_ID
                    | NODE_STATE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                SAMPLES
                    | SAMPLERATE
                    | SESSION_STATES
                    | STATISTICS
                    | STATS
                    | STATS_BUCKETS
                    | STATS_EXTENDED
                    | STATS_HEALTHY
                    | STATS_HISTOGRAMS
                    | STATS_LOCKED
                    | STATS_META
                    | STATS_TOPN
                    | HISTOGRAMS_IN_FLIGHT
                    | TIDB
                    | TIFLASH
                    | TOPN
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(SPLIT | OPTIMISTIC | PESSIMISTIC | WIDTH | REGIONS | REGION | RESET | DRY | RUN),
            |t| t.text().to_string(),
        ),
    ))(i)
}

pub fn not_keyword_token(i: Input) -> IResult<String> {
    alt((
        map(
            rule!(
                ADDDATE
                    | APPROX_COUNT_DISTINCT
                    | APPROX_PERCENTILE
                    | BIT_AND
                    | BIT_OR
                    | BIT_XOR
                    | BRIEF
                    | CAST
                    | COPY
                    | CURTIME
                    | CURDATE
                    | DATE_ADD
                    | DATE_SUB
                    | DEFINED
                    | DOT
                    | DUMP
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                DURATION
                    | EXTRACT
                    | END_TIME
                    | GET_FORMAT
                    | GROUP_CONCAT
                    | HNSW
                    | INPLACE
                    | INSTANT
                    | INTERNAL
                    | LOG
                    | MIN
                    | MAX
                    | NOW
                    | RECENT
                    | REPLAYER
                    | RUNNING
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                PLACEMENT
                    | PLAN
                    | PLAN_CACHE
                    | POSITION
                    | PREDICATE
                    | S3
                    | STRICT
                    | SUBDATE
                    | SUBSTRING
                    | SUM
                    | START_TIME
                    | STD
                    | STDDEV
                    | STDDEV_POP
                    | STDDEV_SAMP
                    | STOP
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                VARIANCE
                    | VAR_POP
                    | VAR_SAMP
                    | TARGET
                    | TIMESTAMPADD
                    | TIMESTAMPDIFF
                    | TOKUDB_DEFAULT
                    | TOKUDB_FAST
                    | TOKUDB_LZMA
                    | TOKUDB_QUICKLZ
                    | TOKUDB_SNAPPY
                    | TOKUDB_SMALL
                    | TOKUDB_UNCOMPRESSED
                    | TOKUDB_ZLIB
                    | TOKUDB_ZSTD
                    | TOP
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                TRIM | NEXT_ROW_ID
                    | EXPR_PUSHDOWN_BLACKLIST
                    | OPT_RULE_BLACKLIST
                    | BOUND
                    | EXACT
                    | STALENESS
                    | STRONG
                    | FLASHBACK
                    | JSON_OBJECTAGG
                    | JSON_ARRAYAGG
                    | TLS
                    | FOLLOWER
                    | FOLLOWERS
                    | LEADER
                    | LEARNER
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                LEARNERS
                    | VERBOSE
                    | TRUE_CARD_COST
                    | VOTER
                    | VOTERS
                    | CONSTRAINTS
                    | PRIMARY_REGION
                    | SCHEDULE
                    | SURVIVAL_PREFERENCES
                    | LEADER_CONSTRAINTS
                    | FOLLOWER_CONSTRAINTS
                    | LEARNER_CONSTRAINTS
                    | VOTER_CONSTRAINTS
                    | TIDB_JSON
                    | IO_READ_BANDWIDTH
                    | IO_WRITE_BANDWIDTH
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                RU_PER_SEC
                    | PRIORITY
                    | HIGH
                    | MEDIUM
                    | LOW
                    | BURSTABLE
                    | BR
                    | GC_TTL
                    | METADATA
                    | START_TS
                    | UNTIL_TS
                    | RESTORED_TS
                    | FULL_BACKUP_STORAGE
                    | EXEC_ELAPSED
                    | PROCESSED_KEYS
                    | RU
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                DRYRUN
                    | COOLDOWN
                    | SWITCH_GROUP
                    | WATCH
                    | SIMILAR
                    | QUERY_LIMIT
                    | BACKGROUND
                    | TASK_TYPES
                    | UNLIMITED
                    | UTILIZATION_LIMIT
            ),
            |t| t.text().to_string(),
        ),
    ))(i)
}

pub fn ident(i: Input) -> IResult<String> {
    map(rule!(Ident), |t| t.text().to_string())(i)
}

pub fn string_lit(i: Input) -> IResult<String> {
    map(rule!(LiteralString), |t| t.text().to_string())(i)
}

pub fn identifier(i: Input) -> IResult<String> {
    map(
        rule!(#ident | #un_reserved_keyword | #not_keyword_token | #tidb_keyword),
        |(s)| s,
    )(i)
}

pub fn string_name(i: Input) -> IResult<String> {
    map(rule!(#string_lit | #identifier), |(s)| s)(i)
}

pub fn charset_name(i: Input) -> IResult<String> {
    alt((
        map(rule!(#string_name), |(s)| {
            /*
            // Validate input charset name to keep the same behavior as parser of MySQL.
            cs, err := charset.GetCharsetInfo($1)
            if err != nil {
                yylex.AppendError(ErrUnknownCharacterSet.GenWithStackByArgs($1))
                return 1
            }
            // Use charset name returned from charset.GetCharsetInfo(),
            // to keep lower case of input for generated column restore.
            $$ = cs.Name

             */
            s
        }),
        map(rule!(BINARY), |(t)| t.text().to_string()),
    ))(i)
}

pub fn get_format_selector(i: Input) -> IResult<GetFormatSelectorType> {
    alt((
        map(rule!(DATE), |(_)| GetFormatSelectorType::Date),
        map(rule!(DATETIME), |(_)| GetFormatSelectorType::Datetime),
        map(rule!(TIME), |(_)| GetFormatSelectorType::Time),
        map(rule!(TIMESTAMP), |(_)| GetFormatSelectorType::Datetime),
    ))(i)
}

pub fn function_name_optional_braces(i: Input) -> IResult<String> {
    map(
        rule!(CURRENT_USER | CURRENT_DATE | CURRENT_ROLE | UTC_DATE | TIDB_CURRENT_TSO),
        |t| t.text().to_string(),
    )(i)
}

pub fn function_name_datetime_precision(i: Input) -> IResult<String> {
    map(
        rule!(
            CURRENT_TIME
                | CURRENT_TIMESTAMP
                | LOCALTIME
                | LOCALTIMESTAMP
                | UTC_TIME
                | UTC_TIMESTAMP
        ),
        |t| t.text().to_string(),
    )(i)
}

pub fn optional_braces(i: Input) -> IResult<()> {
    map(rule!("(" ~ ")"), |_| ())(i)
}

pub fn func_datetime_prec(i: Input) -> IResult<Option<ValueExpr>> {
    alt((
        map(rule!(#optional_braces?), |_| None),
        map(rule!("(" ~ LiteralInteger ~ ")"), |(_, t, _)| {
            let val = get_isize_form_num(t.text());
            let value_expr =
                ValueExpr::new(t.text(), ValueExprKind::Isize(val), i.charset, i.collation);
            Some(value_expr)
        }),
    ))(i)
}

pub fn func_datetime_prec_list_opt(i: Input) -> IResult<Vec<ExprNode>> {
    map(rule!(LiteralInteger?), |t| match t {
        None => vec![],
        Some(token) => {
            let val = get_isize_form_num(token.text());
            let value_expr = ValueExpr::new(
                token.text(),
                ValueExprKind::Isize(val),
                i.charset,
                i.collation,
            );
            vec![ExprNode::ValueExpr(value_expr)]
        }
    })(i)
}

pub fn function_name_date_arith(i: Input) -> IResult<String> {
    map(rule!(DATE_ADD | DATE_SUB), |(t)| t.text().to_string())(i)
}

pub fn function_name_date_arith_multi_forms(i: Input) -> IResult<String> {
    map(rule!(ADDDATE | SUBDATE), |(t)| t.text().to_string())(i)
}

pub fn function_name_conflict(i: Input) -> IResult<String> {
    alt((
        map(
            rule!(
                ASCII
                    | CHARSET
                    | COALESCE
                    | COLLATION
                    | DATE
                    | DATABASE
                    | DAY
                    | HOUR
                    | IF
                    | INTERVAL
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                LOG | FORMAT
                    | LEFT
                    | MICROSECOND
                    | MINUTE
                    | MONTH
                    | NOW
                    | POINT
                    | QUARTER
                    | REPEAT
                    | REPLACE
            ),
            |t| t.text().to_string(),
        ),
        map(
            rule!(
                REVERSE
                    | RIGHT
                    | ROW_COUNT
                    | SECOND
                    | TIME
                    | TIMESTAMP
                    | TRUNCATE
                    | USER
                    | WEEK
                    | YEAR
            ),
            |t| t.text().to_string(),
        ),
    ))(i)
}

pub fn trim_direction(i: Input) -> IResult<TrimDirectionType> {
    alt((
        map(rule!(BOTH), |_| TrimDirectionType::Both),
        map(rule!(LEADING), |_| TrimDirectionType::Leading),
        map(rule!(TRAILING), |_| TrimDirectionType::Trailing),
    ))(i)
}

pub fn field_len(i: Input) -> IResult<isize> {
    map(rule!("(" ~ #length_num ~ ")"), |(_, val, _)| val as isize)(i)
}

pub fn length_num(i: Input) -> IResult<u64> {
    map(rule!(LiteralInteger), |(val)| get_u64_form_num(val.text()))(i)
}

pub fn table_name(i: Input) -> IResult<TableName> {
    alt((
        map(rule!(#identifier), |(table_name)| {
            let mut tbl_name = TableName::default();
            tbl_name.name = CIStr::new(&table_name);
            tbl_name
        }),
        map_res(
            rule!(#identifier ~ "." ~ #identifier),
            |(schema_name, _, table_name)| {
                if is_in_correct_identifier_name(&schema_name) {
                    return Err(nom::Err::Error(ErrorKind::ExpectText(
                        "parse table_name in is_in_correct_identifier_name",
                    )));
                }

                let mut tbl_name = TableName::default();

                tbl_name.schema = CIStr::new(&schema_name);
                tbl_name.name = CIStr::new(&table_name);

                Ok(tbl_name)
            },
        ),
        map(rule!("*" ~ "." ~ #identifier), |(_, _, table_name)| {
            let mut tbl_name = TableName::default();
            tbl_name.schema = CIStr::new("*");
            tbl_name.name = CIStr::new(&table_name);
            tbl_name
        }),
    ))(i)
}

pub fn i64_num(i: Input) -> IResult<i64> {
    map_res(rule!(LiteralInteger), |(val)| {
        let v = get_i64_form_num(val.text());
        match v {
            Ok(val) => Ok(val),
            Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                "{} is out of range [–9223372036854775808,9223372036854775807]. {}",
                val.text(),
                e.to_string()
            )))),
        }
    })(i)
}

pub fn signed_num(i: Input) -> IResult<i64> {
    alt((
        map(rule!(#i64_num), |(val)| val),
        map(rule!("+" ~ #i64_num), |(_, val)| val),
        map_res(rule!("-" ~ LiteralInteger), |(_, val)| {
            let unsigned_num = get_u64_form_num(val.text());
            if unsigned_num > 9223372036854775808 {
                return Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                    "current value: {}. the Signed Value should be at the range of [-9223372036854775808, 9223372036854775807].",
                    val.text(),
                ))));
            } else if unsigned_num == 9223372036854775808 {
                let d = 1_i64 << 63;
                return Ok(d);
            } else {
                let d = -(unsigned_num as i64);
                Ok(d)
            }
        }),
    ))(i)
}

pub fn collation_name(i: Input) -> IResult<String> {
    alt((
        map_res(
            rule!(#string_name),
            |(name)| match charset::get_collation_by_name(&name) {
                Ok(v) => Ok(v.name),
                Err(e) => Err(nom::Err::Error(ErrorKind::ExpectText(&format!(
                    "{}",
                    e.to_string()
                )))),
            },
        ),
        map(rule!(BINARY), |(_)| Ok(charset::COLLATION_BIN.to_string())),
    ))(i)
}
