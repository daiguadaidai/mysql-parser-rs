use logos::Logos;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[allow(non_camel_case_types)]
#[derive(Logos, EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    EOI,

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"--[^\n\f]*", logos::skip)]
    Comment,

    #[regex(r"/\*[^\+]([^\*]|(\*[^/]))*\*/", logos::skip)]
    CommentBlock,

    #[regex(r#"[_a-zA-Z][_$a-zA-Z0-9]*"#)]
    #[regex(r#"`[_a-zA-Z][_$a-zA-Z0-9]*`"#)]
    Ident,

    #[regex(r#"@[_a-zA-Z][_$a-zA-Z0-9]*"#)]
    SingleAtIdent,

    #[regex(r#"@@[_a-zA-Z][_$a-zA-Z0-9]*"#)]
    DoubleAtIdent,

    #[regex(r#"\$[_a-zA-Z][_$a-zA-Z0-9]*"#)]
    IdentVariable,

    #[regex(r#"\$[0-9]+"#)]
    ColumnPosition,

    #[regex(r#""([^"\\]|\\.|"")*""#)]
    #[regex(r#"'([^'\\]|\\.|'')*'"#)]
    LiteralString,

    #[regex(r"0[xX][a-fA-F0-9]+")]
    LiteralHex,

    #[regex(r"0[bB][01]+")]
    #[regex(r"0[bB]'[01]+'")]
    LiteralBit,

    #[regex(r"[0-9]+(_|[0-9])*")]
    LiteralInteger,

    #[regex(r"[0-9]+[eE][+-]?[0-9]+")]
    #[regex(r"([0-9]*\.[0-9]+([eE][+-]?[0-9]+)?)|([0-9]+\.[0-9]*([eE][+-]?[0-9]+)?)")]
    LiteralFloat,

    // Symbols
    #[token("/*+")]
    HintPrefix,
    #[token("*/")]
    HintSuffix,
    #[token("==")]
    DoubleEq,
    #[token("=")]
    Eq,
    #[token("<>")]
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Lte,
    #[token(">=")]
    Gte,
    #[token("<=>")]
    Spaceship,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("//")]
    IntDiv,
    #[token("%")]
    Modulo,
    #[token("||")]
    PipesAsOr,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(":=")]
    AssignmentEq,
    #[token(";")]
    SemiColon,
    #[token("\\")]
    Backslash,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("^")]
    Caret,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("$")]
    Dollar,
    #[token("->")]
    RArrow,
    #[token("->>")]
    LongRArrow,
    #[token("=>")]
    FatRArrow,
    #[token("#>")]
    HashRArrow,
    #[token("#>>")]
    HashLongRArrow,
    /// A case insensitive match regular expression operator in PostgreSQL
    #[token("~*")]
    TildeAsterisk,
    /// A case sensitive not match regular expression operator in PostgreSQL
    #[token("!*")]
    ExclamationMarkTilde,
    /// A case insensitive not match regular expression operator in PostgreSQL
    #[token("!~*")]
    ExclamationMarkTildeAsterisk,
    /// A bitwise and operator in PostgreSQL
    #[token("&")]
    BitWiseAnd,
    /// A bitwise or operator in PostgreSQL
    #[token("|")]
    BitWiseOr,
    /// A bitwise xor operator in PostgreSQL
    #[token("#")]
    BitWiseXor,
    /// A bitwise not operator in PostgreSQL
    #[token("~")]
    BitWiseNot,
    /// A bitwise shift left operator in PostgreSQL
    #[token("<<")]
    ShiftLeft,
    /// A bitwise shift right operator in PostgreSQL
    #[token(">>")]
    ShiftRight,
    /// Exclamation Mark `!` used for PostgreSQL factorial operator
    #[token("!")]
    Factorial,
    /// Double Exclamation Mark `!!` used for PostgreSQL prefix factorial operator
    #[token("!!")]
    DoubleExclamationMark,
    /// AtSign `@` used for PostgreSQL abs operator
    #[token("@")]
    Abs,
    /// A square root math operator in PostgreSQL
    #[token("|/")]
    SquareRoot,
    /// A cube root math operator in PostgreSQL
    #[token("||/")]
    CubeRoot,
    /// Placeholder used in prepared stmt
    /// Also used as JSON operator.
    #[token("?")]
    Placeholder,
    /// Used as JSON operator.
    #[token("?|")]
    QuestionOr,
    /// Used as JSON operator.
    #[token("?&")]
    QuestionAnd,
    /// Used as JSON operator.
    #[token("<@")]
    ArrowAt,
    /// Used as JSON operator.
    #[token("@>")]
    AtArrow,
    /// Used as JSON operator.
    #[token("@?")]
    AtQuestion,
    /// Used as JSON operator.
    #[token("@@")]
    AtAt,
    /// Used as JSON operator.
    #[token("#-")]
    HashMinus,

    // Keywords
    //
    // Steps to add keyword:
    // 1. Add the keyword to token kind variants by alphabetical order.
    // 2. Search in this file to see if the new keyword is a commented out reserved keyword. If
    //    so, uncomment the keyword in the reserved list.
    #[token("ADD", ignore(ascii_case))]
    ADD,
    #[token("ALL", ignore(ascii_case))]
    ALL,
    #[token("ALTER", ignore(ascii_case))]
    ALTER,
    #[token("ANALYZE", ignore(ascii_case))]
    ANALYZE,
    #[token("AND", ignore(ascii_case))]
    AND,
    #[token("ARRAY", ignore(ascii_case))]
    ARRAY,
    #[token("AS", ignore(ascii_case))]
    AS,
    #[token("ASC", ignore(ascii_case))]
    ASC,
    #[token("BETWEEN", ignore(ascii_case))]
    BETWEEN,
    #[token("BIGINT", ignore(ascii_case))]
    BIGINT,
    #[token("BINARY", ignore(ascii_case))]
    BINARY,
    #[token("BLOB", ignore(ascii_case))]
    BLOB,
    #[token("BOTH", ignore(ascii_case))]
    BOTH,
    #[token("BY", ignore(ascii_case))]
    BY,
    #[token("CALL", ignore(ascii_case))]
    CALL,
    #[token("CASCADE", ignore(ascii_case))]
    CASCADE,
    #[token("CASE", ignore(ascii_case))]
    CASE,
    #[token("CHANGE", ignore(ascii_case))]
    CHANGE,
    #[token("CHAR", ignore(ascii_case))]
    CHAR,
    #[token("CHARACTER", ignore(ascii_case))]
    CHARACTER,
    #[token("CHECK", ignore(ascii_case))]
    CHECK,
    #[token("COLLATE", ignore(ascii_case))]
    COLLATE,
    #[token("COLUMN", ignore(ascii_case))]
    COLUMN,
    #[token("CONSTRAINT", ignore(ascii_case))]
    CONSTRAINT,
    #[token("CONTINUE", ignore(ascii_case))]
    CONTINUE,
    #[token("CONVERT", ignore(ascii_case))]
    CONVERT,
    #[token("COUNT", ignore(ascii_case))]
    COUNT,
    #[token("CREATE", ignore(ascii_case))]
    CREATE,
    #[token("CROSS", ignore(ascii_case))]
    CROSS,
    #[token("CUME_DIST", ignore(ascii_case))]
    CUME_DIST,
    #[token("CURRENT_DATE", ignore(ascii_case))]
    CURRENT_DATE,
    #[token("CURRENT_ROLE", ignore(ascii_case))]
    CURRENT_ROLE,
    #[token("CURRENT_TIME", ignore(ascii_case))]
    CURRENT_TIME,
    #[token("CURRENT_TIMESTAMP", ignore(ascii_case))]
    CURRENT_TIMESTAMP,
    #[token("CURRENT_USER", ignore(ascii_case))]
    CURRENT_USER,
    #[token("CURSOR", ignore(ascii_case))]
    CURSOR,
    #[token("DATABASE", ignore(ascii_case))]
    DATABASE,
    #[token("DATABASES", ignore(ascii_case))]
    DATABASES,
    #[token("DAY_HOUR", ignore(ascii_case))]
    DAY_HOUR,
    #[token("DAY_MICROSECOND", ignore(ascii_case))]
    DAY_MICROSECOND,
    #[token("DAY_MINUTE", ignore(ascii_case))]
    DAY_MINUTE,
    #[token("DAY_SECOND", ignore(ascii_case))]
    DAY_SECOND,
    #[token("DECIMAL", ignore(ascii_case))]
    DECIMAL,
    #[token("DEFAULT", ignore(ascii_case))]
    DEFAULT,
    #[token("DELAYED", ignore(ascii_case))]
    DELAYED,
    #[token("DELETE", ignore(ascii_case))]
    DELETE,
    #[token("DENSE_RANK", ignore(ascii_case))]
    DENSE_RANK,
    #[token("DEC", ignore(ascii_case))]
    DEC,
    #[token("DESC", ignore(ascii_case))]
    DESC,
    #[token("DESCRIBE", ignore(ascii_case))]
    DESCRIBE,
    #[token("DISTINCT", ignore(ascii_case))]
    DISTINCT,
    #[token("DISTINCTROW", ignore(ascii_case))]
    DISTINCTROW,
    #[token("DIV", ignore(ascii_case))]
    DIV,
    #[token("DOUBLE", ignore(ascii_case))]
    DOUBLE,
    #[token("DROP", ignore(ascii_case))]
    DROP,
    #[token("DUAL", ignore(ascii_case))]
    DUAL,
    #[token("ELSE", ignore(ascii_case))]
    ELSE,
    #[token("ELSEIF", ignore(ascii_case))]
    ELSEIF,
    #[token("ENCLOSED", ignore(ascii_case))]
    ENCLOSED,
    #[token("ESCAPED", ignore(ascii_case))]
    ESCAPED,
    #[token("EXCEPT", ignore(ascii_case))]
    EXCEPT,
    #[token("EXISTS", ignore(ascii_case))]
    EXISTS,
    #[token("EXIT", ignore(ascii_case))]
    EXIT,
    #[token("EXPLAIN", ignore(ascii_case))]
    EXPLAIN,
    #[token("FALSE", ignore(ascii_case))]
    FALSE,
    #[token("FETCH", ignore(ascii_case))]
    FETCH,
    #[token("FIRST_VALUE", ignore(ascii_case))]
    FIRST_VALUE,
    #[token("FLOAT", ignore(ascii_case))]
    FLOAT,
    #[token("FLOAT4", ignore(ascii_case))]
    FLOAT4,
    #[token("FLOAT8", ignore(ascii_case))]
    FLOAT8,
    #[token("FOR", ignore(ascii_case))]
    FOR,
    #[token("FORCE", ignore(ascii_case))]
    FORCE,
    #[token("FOREIGN", ignore(ascii_case))]
    FOREIGN,
    #[token("FROM", ignore(ascii_case))]
    FROM,
    #[token("FULLTEXT", ignore(ascii_case))]
    FULLTEXT,
    #[token("GENERATED", ignore(ascii_case))]
    GENERATED,
    #[token("GRANT", ignore(ascii_case))]
    GRANT,
    #[token("GROUP", ignore(ascii_case))]
    GROUP,
    #[token("GROUPS", ignore(ascii_case))]
    GROUPS,
    #[token("HAVING", ignore(ascii_case))]
    HAVING,
    #[token("HIGH_PRIORITY", ignore(ascii_case))]
    HIGH_PRIORITY,
    #[token("HOUR_MICROSECOND", ignore(ascii_case))]
    HOUR_MICROSECOND,
    #[token("HOUR_MINUTE", ignore(ascii_case))]
    HOUR_MINUTE,
    #[token("HOUR_SECOND", ignore(ascii_case))]
    HOUR_SECOND,
    #[token("IF", ignore(ascii_case))]
    IF,
    #[token("IGNORE", ignore(ascii_case))]
    IGNORE,
    #[token("ILIKE", ignore(ascii_case))]
    ILIKE,
    #[token("IN", ignore(ascii_case))]
    IN,
    #[token("INDEX", ignore(ascii_case))]
    INDEX,
    #[token("INFILE", ignore(ascii_case))]
    INFILE,
    #[token("INNER", ignore(ascii_case))]
    INNER,
    #[token("INOUT", ignore(ascii_case))]
    INOUT,
    #[token("INSERT", ignore(ascii_case))]
    INSERT,
    #[token("INT", ignore(ascii_case))]
    INT,
    #[token("INT1", ignore(ascii_case))]
    INT1,
    #[token("INT2", ignore(ascii_case))]
    INT2,
    #[token("INT3", ignore(ascii_case))]
    INT3,
    #[token("INT4", ignore(ascii_case))]
    INT4,
    #[token("INT8", ignore(ascii_case))]
    INT8,
    #[token("INTEGER", ignore(ascii_case))]
    INTEGER,
    #[token("INTERSECT", ignore(ascii_case))]
    INTERSECT,
    #[token("INTERVAL", ignore(ascii_case))]
    INTERVAL,
    #[token("INTO", ignore(ascii_case))]
    INTO,
    #[token("IS", ignore(ascii_case))]
    IS,
    #[token("ITERATE", ignore(ascii_case))]
    ITERATE,
    #[token("JOIN", ignore(ascii_case))]
    JOIN,
    #[token("KEY", ignore(ascii_case))]
    KEY,
    #[token("KEYS", ignore(ascii_case))]
    KEYS,
    #[token("KILL", ignore(ascii_case))]
    KILL,
    #[token("LAG", ignore(ascii_case))]
    LAG,
    #[token("LAST_VALUE", ignore(ascii_case))]
    LAST_VALUE,
    #[token("LEAD", ignore(ascii_case))]
    LEAD,
    #[token("LEADING", ignore(ascii_case))]
    LEADING,
    #[token("LEAVE", ignore(ascii_case))]
    LEAVE,
    #[token("LEFT", ignore(ascii_case))]
    LEFT,
    #[token("LIKE", ignore(ascii_case))]
    LIKE,
    #[token("LIMIT", ignore(ascii_case))]
    LIMIT,
    #[token("LINEAR", ignore(ascii_case))]
    LINEAR,
    #[token("LINES", ignore(ascii_case))]
    LINES,
    #[token("LOAD", ignore(ascii_case))]
    LOAD,
    #[token("LOCALTIME", ignore(ascii_case))]
    LOCALTIME,
    #[token("LOCALTIMESTAMP", ignore(ascii_case))]
    LOCALTIMESTAMP,
    #[token("LOCK", ignore(ascii_case))]
    LOCK,
    #[token("LONG", ignore(ascii_case))]
    LONG,
    #[token("LONGBLOB", ignore(ascii_case))]
    LONGBLOB,
    #[token("LONGTEXT", ignore(ascii_case))]
    LONGTEXT,
    #[token("LOW_PRIORITY", ignore(ascii_case))]
    LOW_PRIORITY,
    #[token("MATCH", ignore(ascii_case))]
    MATCH,
    #[token("MAXVALUE", ignore(ascii_case))]
    MAXVALUE,
    #[token("MEDIUMBLOB", ignore(ascii_case))]
    MEDIUMBLOB,
    #[token("MEDIUMINT", ignore(ascii_case))]
    MEDIUMINT,
    #[token("MEDIUMTEXT", ignore(ascii_case))]
    MEDIUMTEXT,
    #[token("MIDDLEINT", ignore(ascii_case))]
    MIDDLEINT,
    #[token("MINUTE_MICROSECOND", ignore(ascii_case))]
    MINUTE_MICROSECOND,
    #[token("MINUTE_SECOND", ignore(ascii_case))]
    MINUTE_SECOND,
    #[token("MOD", ignore(ascii_case))]
    MOD,
    #[token("NATURAL", ignore(ascii_case))]
    NATURAL,
    #[token("NOW", ignore(ascii_case))]
    NOW,
    #[token("NOT", ignore(ascii_case))]
    NOT,
    #[token("NOT2", ignore(ascii_case))]
    NOT2,
    #[token("NO_WRITE_TO_BINLOG", ignore(ascii_case))]
    NO_WRITE_TO_BINLOG,
    #[token("NTH_VALUE", ignore(ascii_case))]
    NTH_VALUE,
    #[token("NTILE", ignore(ascii_case))]
    NTILE,
    #[token("NULL", ignore(ascii_case))]
    NULL,
    #[token("NUMERIC", ignore(ascii_case))]
    NUMERIC,
    #[token("OF", ignore(ascii_case))]
    OF,
    #[token("ON", ignore(ascii_case))]
    ON,
    #[token("OPTIMIZE", ignore(ascii_case))]
    OPTIMIZE,
    #[token("OPTION", ignore(ascii_case))]
    OPTION,
    #[token("OPTIONALLY", ignore(ascii_case))]
    OPTIONALLY,
    #[token("OR", ignore(ascii_case))]
    OR,
    #[token("ORDER", ignore(ascii_case))]
    ORDER,
    #[token("OUT", ignore(ascii_case))]
    OUT,
    #[token("OUTER", ignore(ascii_case))]
    OUTER,
    #[token("OUTFILE", ignore(ascii_case))]
    OUTFILE,
    #[token("OVER", ignore(ascii_case))]
    OVER,
    #[token("PARTITION", ignore(ascii_case))]
    PARTITION,
    #[token("PERCENT_RANK", ignore(ascii_case))]
    PERCENT_RANK,
    #[token("PRECISION", ignore(ascii_case))]
    PRECISION,
    #[token("PRIMARY", ignore(ascii_case))]
    PRIMARY,
    #[token("PROCEDURE", ignore(ascii_case))]
    PROCEDURE,
    #[token("RANGE", ignore(ascii_case))]
    RANGE,
    #[token("RANK", ignore(ascii_case))]
    RANK,
    #[token("READ", ignore(ascii_case))]
    READ,
    #[token("REAL", ignore(ascii_case))]
    REAL,
    #[token("RECURSIVE", ignore(ascii_case))]
    RECURSIVE,
    #[token("REFERENCES", ignore(ascii_case))]
    REFERENCES,
    #[token("REGEXP", ignore(ascii_case))]
    REGEXP,
    #[token("RELEASE", ignore(ascii_case))]
    RELEASE,
    #[token("RENAME", ignore(ascii_case))]
    RENAME,
    #[token("REPEAT", ignore(ascii_case))]
    REPEAT,
    #[token("REPLACE", ignore(ascii_case))]
    REPLACE,
    #[token("REQUIRE", ignore(ascii_case))]
    REQUIRE,
    #[token("RESTRICT", ignore(ascii_case))]
    RESTRICT,
    #[token("REVOKE", ignore(ascii_case))]
    REVOKE,
    #[token("RIGHT", ignore(ascii_case))]
    RIGHT,
    #[token("RLIKE", ignore(ascii_case))]
    RLIKE,
    #[token("ROW", ignore(ascii_case))]
    ROW,
    #[token("ROWS", ignore(ascii_case))]
    ROWS,
    #[token("ROW_NUMBER", ignore(ascii_case))]
    ROW_NUMBER,
    #[token("SECOND_MICROSECOND", ignore(ascii_case))]
    SECOND_MICROSECOND,
    #[token("SELECT", ignore(ascii_case))]
    SELECT,
    #[token("SET", ignore(ascii_case))]
    SET,
    #[token("SHOW", ignore(ascii_case))]
    SHOW,
    #[token("SMALLINT", ignore(ascii_case))]
    SMALLINT,
    #[token("SPATIAL", ignore(ascii_case))]
    SPATIAL,
    #[token("SQL", ignore(ascii_case))]
    SQL,
    #[token("SQLEXCEPTION", ignore(ascii_case))]
    SQLEXCEPTION,
    #[token("SQLSTATE", ignore(ascii_case))]
    SQLSTATE,
    #[token("SQLWARNING", ignore(ascii_case))]
    SQLWARNING,
    #[token("SQL_BIG_RESULT", ignore(ascii_case))]
    SQL_BIG_RESULT,
    #[token("SQL_CALC_FOUND_ROWS", ignore(ascii_case))]
    SQL_CALC_FOUND_ROWS,
    #[token("SQL_SMALL_RESULT", ignore(ascii_case))]
    SQL_SMALL_RESULT,
    #[token("SSL", ignore(ascii_case))]
    SSL,
    #[token("STARTING", ignore(ascii_case))]
    STARTING,
    #[token("STORED", ignore(ascii_case))]
    STORED,
    #[token("STRAIGHT_JOIN", ignore(ascii_case))]
    STRAIGHT_JOIN,
    #[token("TABLE", ignore(ascii_case))]
    TABLE,
    #[token("TABLESAMPLE", ignore(ascii_case))]
    TABLESAMPLE,
    #[token("TERMINATED", ignore(ascii_case))]
    TERMINATED,
    #[token("THEN", ignore(ascii_case))]
    THEN,
    #[token("TIDB_CURRENT_TSO", ignore(ascii_case))]
    TIDB_CURRENT_TSO,
    #[token("TINYBLOB", ignore(ascii_case))]
    TINYBLOB,
    #[token("TINYINT", ignore(ascii_case))]
    TINYINT,
    #[token("TINYTEXT", ignore(ascii_case))]
    TINYTEXT,
    #[token("TO", ignore(ascii_case))]
    TO,
    #[token("TRAILING", ignore(ascii_case))]
    TRAILING,
    #[token("TRIGGER", ignore(ascii_case))]
    TRIGGER,
    #[token("TRUE", ignore(ascii_case))]
    TRUE,
    #[token("UNION", ignore(ascii_case))]
    UNION,
    #[token("UNIQUE", ignore(ascii_case))]
    UNIQUE,
    #[token("UNLOCK", ignore(ascii_case))]
    UNLOCK,
    #[token("UNSIGNED", ignore(ascii_case))]
    UNSIGNED,
    #[token("UNTIL", ignore(ascii_case))]
    UNTIL,
    #[token("UPDATE", ignore(ascii_case))]
    UPDATE,
    #[token("USAGE", ignore(ascii_case))]
    USAGE,
    #[token("USE", ignore(ascii_case))]
    USE,
    #[token("USING", ignore(ascii_case))]
    USING,
    #[token("UTC_DATE", ignore(ascii_case))]
    UTC_DATE,
    #[token("UTC_TIME", ignore(ascii_case))]
    UTC_TIME,
    #[token("UTC_TIMESTAMP", ignore(ascii_case))]
    UTC_TIMESTAMP,
    #[token("VALUES", ignore(ascii_case))]
    VALUES,
    #[token("VARBINARY", ignore(ascii_case))]
    VARBINARY,
    #[token("VARCHAR", ignore(ascii_case))]
    VARCHAR,
    #[token("VARCHARACTER", ignore(ascii_case))]
    VARCHARACTER,
    #[token("VARYING", ignore(ascii_case))]
    VARYING,
    #[token("VIRTUAL", ignore(ascii_case))]
    VIRTUAL,
    #[token("WHEN", ignore(ascii_case))]
    WHEN,
    #[token("WHERE", ignore(ascii_case))]
    WHERE,
    #[token("WHILE", ignore(ascii_case))]
    WHILE,
    #[token("WINDOW", ignore(ascii_case))]
    WINDOW,
    #[token("WITH", ignore(ascii_case))]
    WITH,
    #[token("WRITE", ignore(ascii_case))]
    WRITE,
    #[token("XOR", ignore(ascii_case))]
    XOR,
    #[token("YEAR_MONTH", ignore(ascii_case))]
    YEAR_MONTH,
    #[token("ZEROFILL", ignore(ascii_case))]
    ZEROFILL,
    #[token("ACCOUNT", ignore(ascii_case))]
    ACCOUNT,
    #[token("ACTION", ignore(ascii_case))]
    ACTION,
    #[token("ADVISE", ignore(ascii_case))]
    ADVISE,
    #[token("AFTER", ignore(ascii_case))]
    AFTER,
    #[token("AGAINST", ignore(ascii_case))]
    AGAINST,
    #[token("AGO", ignore(ascii_case))]
    AGO,
    #[token("ALGORITHM", ignore(ascii_case))]
    ALGORITHM,
    #[token("ALWAYS", ignore(ascii_case))]
    ALWAYS,
    #[token("ANY", ignore(ascii_case))]
    ANY,
    #[token("APPLY", ignore(ascii_case))]
    APPLY,
    #[token("ASCII", ignore(ascii_case))]
    ASCII,
    #[token("ATTRIBUTE", ignore(ascii_case))]
    ATTRIBUTE,
    #[token("ATTRIBUTES", ignore(ascii_case))]
    ATTRIBUTES,
    #[token("AUTO_ID_CACHE", ignore(ascii_case))]
    AUTO_ID_CACHE,
    #[token("AUTO_INCREMENT", ignore(ascii_case))]
    AUTO_INCREMENT,
    #[token("AUTO_RANDOM", ignore(ascii_case))]
    AUTO_RANDOM,
    #[token("AUTO_RANDOM_BASE", ignore(ascii_case))]
    AUTO_RANDOM_BASE,
    #[token("AVG", ignore(ascii_case))]
    AVG,
    #[token("AVG_ROW_LENGTH", ignore(ascii_case))]
    AVG_ROW_LENGTH,
    #[token("BACKEND", ignore(ascii_case))]
    BACKEND,
    #[token("BACKUP", ignore(ascii_case))]
    BACKUP,
    #[token("BACKUPS", ignore(ascii_case))]
    BACKUPS,
    #[token("BDR", ignore(ascii_case))]
    BDR,
    #[token("BEGIN", ignore(ascii_case))]
    BEGIN,
    #[token("BERNOULLI", ignore(ascii_case))]
    BERNOULLI,
    #[token("BINDING", ignore(ascii_case))]
    BINDING,
    #[token("BINDINGS", ignore(ascii_case))]
    BINDINGS,
    #[token("BINDING_CACHE", ignore(ascii_case))]
    BINDING_CACHE,
    #[token("BINLOG", ignore(ascii_case))]
    BINLOG,
    #[token("BIT", ignore(ascii_case))]
    BIT,
    #[token("BLOCK", ignore(ascii_case))]
    BLOCK,
    #[token("BOOL", ignore(ascii_case))]
    BOOL,
    #[token("BOOLEAN", ignore(ascii_case))]
    BOOLEAN,
    #[token("BTREE", ignore(ascii_case))]
    BTREE,
    #[token("BYTE", ignore(ascii_case))]
    BYTE,
    #[token("CACHE", ignore(ascii_case))]
    CACHE,
    #[token("CALIBRATE", ignore(ascii_case))]
    CALIBRATE,
    #[token("CAPTURE", ignore(ascii_case))]
    CAPTURE,
    #[token("CASCADED", ignore(ascii_case))]
    CASCADED,
    #[token("CAUSAL", ignore(ascii_case))]
    CAUSAL,
    #[token("CHAIN", ignore(ascii_case))]
    CHAIN,
    #[token("CHARSET", ignore(ascii_case))]
    CHARSET,
    #[token("CHECKPOINT", ignore(ascii_case))]
    CHECKPOINT,
    #[token("CHECKSUM", ignore(ascii_case))]
    CHECKSUM,
    #[token("CHECKSUM_CONCURRENCY", ignore(ascii_case))]
    CHECKSUM_CONCURRENCY,
    #[token("CIPHER", ignore(ascii_case))]
    CIPHER,
    #[token("CLEANUP", ignore(ascii_case))]
    CLEANUP,
    #[token("CLIENT", ignore(ascii_case))]
    CLIENT,
    #[token("CLIENT_ERRORS_SUMMARY", ignore(ascii_case))]
    CLIENT_ERRORS_SUMMARY,
    #[token("CLOSE", ignore(ascii_case))]
    CLOSE,
    #[token("CLUSTER", ignore(ascii_case))]
    CLUSTER,
    #[token("CLUSTERED", ignore(ascii_case))]
    CLUSTERED,
    #[token("COALESCE", ignore(ascii_case))]
    COALESCE,
    #[token("COLLATION", ignore(ascii_case))]
    COLLATION,
    #[token("COLUMNS", ignore(ascii_case))]
    COLUMNS,
    #[token("COLUMN_FORMAT", ignore(ascii_case))]
    COLUMN_FORMAT,
    #[token("COMMENT", ignore(ascii_case))]
    COMMENT,
    #[token("COMMIT", ignore(ascii_case))]
    COMMIT,
    #[token("COMMITTED", ignore(ascii_case))]
    COMMITTED,
    #[token("COMPACT", ignore(ascii_case))]
    COMPACT,
    #[token("COMPRESSED", ignore(ascii_case))]
    COMPRESSED,
    #[token("COMPRESSION", ignore(ascii_case))]
    COMPRESSION,
    #[token("COMPRESSION_LEVEL", ignore(ascii_case))]
    COMPRESSION_LEVEL,
    #[token("COMPRESSION_TYPE", ignore(ascii_case))]
    COMPRESSION_TYPE,
    #[token("CONCURRENCY", ignore(ascii_case))]
    CONCURRENCY,
    #[token("CONFIG", ignore(ascii_case))]
    CONFIG,
    #[token("CONNECTION", ignore(ascii_case))]
    CONNECTION,
    #[token("CONSISTENCY", ignore(ascii_case))]
    CONSISTENCY,
    #[token("CONSISTENT", ignore(ascii_case))]
    CONSISTENT,
    #[token("CONTEXT", ignore(ascii_case))]
    CONTEXT,
    #[token("CPU", ignore(ascii_case))]
    CPU,
    #[token("CSV_BACKSLASH_ESCAPE", ignore(ascii_case))]
    CSV_BACKSLASH_ESCAPE,
    #[token("CSV_DELIMITER", ignore(ascii_case))]
    CSV_DELIMITER,
    #[token("CSV_HEADER", ignore(ascii_case))]
    CSV_HEADER,
    #[token("CSV_NOT_NULL", ignore(ascii_case))]
    CSV_NOT_NULL,
    #[token("CSV_NULL", ignore(ascii_case))]
    CSV_NULL,
    #[token("CSV_SEPARATOR", ignore(ascii_case))]
    CSV_SEPARATOR,
    #[token("CSV_TRIM_LAST_SEPARATORS", ignore(ascii_case))]
    CSV_TRIM_LAST_SEPARATORS,
    #[token("CURRENT", ignore(ascii_case))]
    CURRENT,
    #[token("CYCLE", ignore(ascii_case))]
    CYCLE,
    #[token("DATA", ignore(ascii_case))]
    DATA,
    #[token("DATE", ignore(ascii_case))]
    DATE,
    #[token("DATETIME", ignore(ascii_case))]
    DATETIME,
    #[token("DAY", ignore(ascii_case))]
    DAY,
    #[token("DEALLOCATE", ignore(ascii_case))]
    DEALLOCATE,
    #[token("DECLARE", ignore(ascii_case))]
    DECLARE,
    #[token("DEFINER", ignore(ascii_case))]
    DEFINER,
    #[token("DELAY_KEY_WRITE", ignore(ascii_case))]
    DELAY_KEY_WRITE,
    #[token("DIGEST", ignore(ascii_case))]
    DIGEST,
    #[token("DIRECTORY", ignore(ascii_case))]
    DIRECTORY,
    #[token("DISABLE", ignore(ascii_case))]
    DISABLE,
    #[token("DISABLED", ignore(ascii_case))]
    DISABLED,
    #[token("DISCARD", ignore(ascii_case))]
    DISCARD,
    #[token("DISK", ignore(ascii_case))]
    DISK,
    #[token("DO", ignore(ascii_case))]
    DO,
    #[token("DUPLICATE", ignore(ascii_case))]
    DUPLICATE,
    #[token("DYNAMIC", ignore(ascii_case))]
    DYNAMIC,
    #[token("ENABLE", ignore(ascii_case))]
    ENABLE,
    #[token("ENABLED", ignore(ascii_case))]
    ENABLED,
    #[token("ENCRYPTION", ignore(ascii_case))]
    ENCRYPTION,
    #[token("ENCRYPTION_KEYFILE", ignore(ascii_case))]
    ENCRYPTION_KEYFILE,
    #[token("ENCRYPTION_METHOD", ignore(ascii_case))]
    ENCRYPTION_METHOD,
    #[token("END", ignore(ascii_case))]
    END,
    #[token("ENFORCED", ignore(ascii_case))]
    ENFORCED,
    #[token("ENGINE", ignore(ascii_case))]
    ENGINE,
    #[token("ENGINES", ignore(ascii_case))]
    ENGINES,
    #[token("ENUM", ignore(ascii_case))]
    ENUM,
    #[token("ERROR", ignore(ascii_case))]
    ERROR,
    #[token("ERRORS", ignore(ascii_case))]
    ERRORS,
    #[token("ESCAPE", ignore(ascii_case))]
    ESCAPE,
    #[token("EVENT", ignore(ascii_case))]
    EVENT,
    #[token("EVENTS", ignore(ascii_case))]
    EVENTS,
    #[token("EVOLVE", ignore(ascii_case))]
    EVOLVE,
    #[token("EXCHANGE", ignore(ascii_case))]
    EXCHANGE,
    #[token("EXCLUSIVE", ignore(ascii_case))]
    EXCLUSIVE,
    #[token("EXECUTE", ignore(ascii_case))]
    EXECUTE,
    #[token("EXPANSION", ignore(ascii_case))]
    EXPANSION,
    #[token("EXPIRE", ignore(ascii_case))]
    EXPIRE,
    #[token("EXTENDED", ignore(ascii_case))]
    EXTENDED,
    #[token("FAILED_LOGIN_ATTEMPTS", ignore(ascii_case))]
    FAILED_LOGIN_ATTEMPTS,
    #[token("FAULTS", ignore(ascii_case))]
    FAULTS,
    #[token("FIELDS", ignore(ascii_case))]
    FIELDS,
    #[token("FILE", ignore(ascii_case))]
    FILE,
    #[token("FIRST", ignore(ascii_case))]
    FIRST,
    #[token("FIXED", ignore(ascii_case))]
    FIXED,
    #[token("FLUSH", ignore(ascii_case))]
    FLUSH,
    #[token("FOLLOWING", ignore(ascii_case))]
    FOLLOWING,
    #[token("FORMAT", ignore(ascii_case))]
    FORMAT,
    #[token("FOUND", ignore(ascii_case))]
    FOUND,
    #[token("FULL", ignore(ascii_case))]
    FULL,
    #[token("FUNCTION", ignore(ascii_case))]
    FUNCTION,
    #[token("GENERAL", ignore(ascii_case))]
    GENERAL,
    #[token("GLOBAL", ignore(ascii_case))]
    GLOBAL,
    #[token("GRANTS", ignore(ascii_case))]
    GRANTS,
    #[token("HANDLER", ignore(ascii_case))]
    HANDLER,
    #[token("HASH", ignore(ascii_case))]
    HASH,
    #[token("HELP", ignore(ascii_case))]
    HELP,
    #[token("HISTOGRAM", ignore(ascii_case))]
    HISTOGRAM,
    #[token("HISTORY", ignore(ascii_case))]
    HISTORY,
    #[token("HOSTS", ignore(ascii_case))]
    HOSTS,
    #[token("HOUR", ignore(ascii_case))]
    HOUR,
    #[token("HYPO", ignore(ascii_case))]
    HYPO,
    #[token("IDENTIFIED", ignore(ascii_case))]
    IDENTIFIED,
    #[token("IGNORE_STATS", ignore(ascii_case))]
    IGNORE_STATS,
    #[token("IMPORT", ignore(ascii_case))]
    IMPORT,
    #[token("IMPORTS", ignore(ascii_case))]
    IMPORTS,
    #[token("INCREMENT", ignore(ascii_case))]
    INCREMENT,
    #[token("INCREMENTAL", ignore(ascii_case))]
    INCREMENTAL,
    #[token("INDEXES", ignore(ascii_case))]
    INDEXES,
    #[token("INSERT_METHOD", ignore(ascii_case))]
    INSERT_METHOD,
    #[token("INSTANCE", ignore(ascii_case))]
    INSTANCE,
    #[token("INVISIBLE", ignore(ascii_case))]
    INVISIBLE,
    #[token("INVOKER", ignore(ascii_case))]
    INVOKER,
    #[token("IO", ignore(ascii_case))]
    IO,
    #[token("IPC", ignore(ascii_case))]
    IPC,
    #[token("ISOLATION", ignore(ascii_case))]
    ISOLATION,
    #[token("ISSUER", ignore(ascii_case))]
    ISSUER,
    #[token("JSON", ignore(ascii_case))]
    JSON,
    #[token("KEY_BLOCK_SIZE", ignore(ascii_case))]
    KEY_BLOCK_SIZE,
    #[token("LABELS", ignore(ascii_case))]
    LABELS,
    #[token("LANGUAGE", ignore(ascii_case))]
    LANGUAGE,
    #[token("LAST", ignore(ascii_case))]
    LAST,
    #[token("LASTVAL", ignore(ascii_case))]
    LASTVAL,
    #[token("LAST_BACKUP", ignore(ascii_case))]
    LAST_BACKUP,
    #[token("LESS", ignore(ascii_case))]
    LESS,
    #[token("LEVEL", ignore(ascii_case))]
    LEVEL,
    #[token("LIST", ignore(ascii_case))]
    LIST,
    #[token("LOAD_STATS", ignore(ascii_case))]
    LOAD_STATS,
    #[token("LOCAL", ignore(ascii_case))]
    LOCAL,
    #[token("LOCATION", ignore(ascii_case))]
    LOCATION,
    #[token("LOCKED", ignore(ascii_case))]
    LOCKED,
    #[token("LOG", ignore(ascii_case))]
    LOG,
    #[token("LOGS", ignore(ascii_case))]
    LOGS,
    #[token("MASTER", ignore(ascii_case))]
    MASTER,
    #[token("MAX_CONNECTIONS_PER_HOUR", ignore(ascii_case))]
    MAX_CONNECTIONS_PER_HOUR,
    #[token("MAX_IDXNUM", ignore(ascii_case))]
    MAX_IDXNUM,
    #[token("MAX_MINUTES", ignore(ascii_case))]
    MAX_MINUTES,
    #[token("MAX_QUERIES_PER_HOUR", ignore(ascii_case))]
    MAX_QUERIES_PER_HOUR,
    #[token("MAX_ROWS", ignore(ascii_case))]
    MAX_ROWS,
    #[token("MAX_UPDATES_PER_HOUR", ignore(ascii_case))]
    MAX_UPDATES_PER_HOUR,
    #[token("MAX_USER_CONNECTIONS", ignore(ascii_case))]
    MAX_USER_CONNECTIONS,
    #[token("MB", ignore(ascii_case))]
    MB,
    #[token("MEMBER", ignore(ascii_case))]
    MEMBER,
    #[token("MEMORY", ignore(ascii_case))]
    MEMORY,
    #[token("MERGE", ignore(ascii_case))]
    MERGE,
    #[token("MICROSECOND", ignore(ascii_case))]
    MICROSECOND,
    #[token("MINUTE", ignore(ascii_case))]
    MINUTE,
    #[token("MINVALUE", ignore(ascii_case))]
    MINVALUE,
    #[token("MIN_ROWS", ignore(ascii_case))]
    MIN_ROWS,
    #[token("MODE", ignore(ascii_case))]
    MODE,
    #[token("MODIFY", ignore(ascii_case))]
    MODIFY,
    #[token("MONTH", ignore(ascii_case))]
    MONTH,
    #[token("NAMES", ignore(ascii_case))]
    NAMES,
    #[token("NATIONAL", ignore(ascii_case))]
    NATIONAL,
    #[token("NCHAR", ignore(ascii_case))]
    NCHAR,
    #[token("NEVER", ignore(ascii_case))]
    NEVER,
    #[token("NEXT", ignore(ascii_case))]
    NEXT,
    #[token("NEXTVAL", ignore(ascii_case))]
    NEXTVAL,
    #[token("NO", ignore(ascii_case))]
    NO,
    #[token("NOCACHE", ignore(ascii_case))]
    NOCACHE,
    #[token("NOCYCLE", ignore(ascii_case))]
    NOCYCLE,
    #[token("NODEGROUP", ignore(ascii_case))]
    NODEGROUP,
    #[token("NOMAXVALUE", ignore(ascii_case))]
    NOMAXVALUE,
    #[token("NOMINVALUE", ignore(ascii_case))]
    NOMINVALUE,
    #[token("NONCLUSTERED", ignore(ascii_case))]
    NONCLUSTERED,
    #[token("NONE", ignore(ascii_case))]
    NONE,
    #[token("NOWAIT", ignore(ascii_case))]
    NOWAIT,
    #[token("NULLS", ignore(ascii_case))]
    NULLS,
    #[token("NVARCHAR", ignore(ascii_case))]
    NVARCHAR,
    #[token("OFF", ignore(ascii_case))]
    OFF,
    #[token("OFFSET", ignore(ascii_case))]
    OFFSET,
    #[token("OLTP_READ_ONLY", ignore(ascii_case))]
    OLTP_READ_ONLY,
    #[token("OLTP_READ_WRITE", ignore(ascii_case))]
    OLTP_READ_WRITE,
    #[token("OLTP_WRITE_ONLY", ignore(ascii_case))]
    OLTP_WRITE_ONLY,
    #[token("ONLINE", ignore(ascii_case))]
    ONLINE,
    #[token("ONLY", ignore(ascii_case))]
    ONLY,
    #[token("ON_DUPLICATE", ignore(ascii_case))]
    ON_DUPLICATE,
    #[token("OPEN", ignore(ascii_case))]
    OPEN,
    #[token("OPTIONAL", ignore(ascii_case))]
    OPTIONAL,
    #[token("PACK_KEYS", ignore(ascii_case))]
    PACK_KEYS,
    #[token("PAGE", ignore(ascii_case))]
    PAGE,
    #[token("PARSER", ignore(ascii_case))]
    PARSER,
    #[token("PARTIAL", ignore(ascii_case))]
    PARTIAL,
    #[token("PARTITIONING", ignore(ascii_case))]
    PARTITIONING,
    #[token("PARTITIONS", ignore(ascii_case))]
    PARTITIONS,
    #[token("PASSWORD", ignore(ascii_case))]
    PASSWORD,
    #[token("PASSWORD_LOCK_TIME", ignore(ascii_case))]
    PASSWORD_LOCK_TIME,
    #[token("PAUSE", ignore(ascii_case))]
    PAUSE,
    #[token("PERCENT", ignore(ascii_case))]
    PERCENT,
    #[token("PER_DB", ignore(ascii_case))]
    PER_DB,
    #[token("PER_TABLE", ignore(ascii_case))]
    PER_TABLE,
    #[token("PLUGINS", ignore(ascii_case))]
    PLUGINS,
    #[token("POINT", ignore(ascii_case))]
    POINT,
    #[token("POLICY", ignore(ascii_case))]
    POLICY,
    #[token("PRECEDING", ignore(ascii_case))]
    PRECEDING,
    #[token("PREPARE", ignore(ascii_case))]
    PREPARE,
    #[token("PRESERVE", ignore(ascii_case))]
    PRESERVE,
    #[token("PRE_SPLIT_REGIONS", ignore(ascii_case))]
    PRE_SPLIT_REGIONS,
    #[token("PRIVILEGES", ignore(ascii_case))]
    PRIVILEGES,
    #[token("PROCESS", ignore(ascii_case))]
    PROCESS,
    #[token("PROCESSLIST", ignore(ascii_case))]
    PROCESSLIST,
    #[token("PROFILE", ignore(ascii_case))]
    PROFILE,
    #[token("PROFILES", ignore(ascii_case))]
    PROFILES,
    #[token("PROXY", ignore(ascii_case))]
    PROXY,
    #[token("PURGE", ignore(ascii_case))]
    PURGE,
    #[token("QUARTER", ignore(ascii_case))]
    QUARTER,
    #[token("QUERIES", ignore(ascii_case))]
    QUERIES,
    #[token("QUERY", ignore(ascii_case))]
    QUERY,
    #[token("QUICK", ignore(ascii_case))]
    QUICK,
    #[token("RATE_LIMIT", ignore(ascii_case))]
    RATE_LIMIT,
    #[token("REBUILD", ignore(ascii_case))]
    REBUILD,
    #[token("RECOMMEND", ignore(ascii_case))]
    RECOMMEND,
    #[token("RECOVER", ignore(ascii_case))]
    RECOVER,
    #[token("REDUNDANT", ignore(ascii_case))]
    REDUNDANT,
    #[token("RELOAD", ignore(ascii_case))]
    RELOAD,
    #[token("REMOVE", ignore(ascii_case))]
    REMOVE,
    #[token("REORGANIZE", ignore(ascii_case))]
    REORGANIZE,
    #[token("REPAIR", ignore(ascii_case))]
    REPAIR,
    #[token("REPEATABLE", ignore(ascii_case))]
    REPEATABLE,
    #[token("REPLICA", ignore(ascii_case))]
    REPLICA,
    #[token("REPLICAS", ignore(ascii_case))]
    REPLICAS,
    #[token("REPLICATION", ignore(ascii_case))]
    REPLICATION,
    #[token("REQUIRED", ignore(ascii_case))]
    REQUIRED,
    #[token("RESOURCE", ignore(ascii_case))]
    RESOURCE,
    #[token("RESPECT", ignore(ascii_case))]
    RESPECT,
    #[token("RESTART", ignore(ascii_case))]
    RESTART,
    #[token("RESTORE", ignore(ascii_case))]
    RESTORE,
    #[token("RESTORES", ignore(ascii_case))]
    RESTORES,
    #[token("RESUME", ignore(ascii_case))]
    RESUME,
    #[token("REUSE", ignore(ascii_case))]
    REUSE,
    #[token("REVERSE", ignore(ascii_case))]
    REVERSE,
    #[token("ROLE", ignore(ascii_case))]
    ROLE,
    #[token("ROLLBACK", ignore(ascii_case))]
    ROLLBACK,
    #[token("ROLLUP", ignore(ascii_case))]
    ROLLUP,
    #[token("ROUTINE", ignore(ascii_case))]
    ROUTINE,
    #[token("ROW_COUNT", ignore(ascii_case))]
    ROW_COUNT,
    #[token("ROW_FORMAT", ignore(ascii_case))]
    ROW_FORMAT,
    #[token("RTREE", ignore(ascii_case))]
    RTREE,
    #[token("SAN", ignore(ascii_case))]
    SAN,
    #[token("SAVEPOINT", ignore(ascii_case))]
    SAVEPOINT,
    #[token("SECOND", ignore(ascii_case))]
    SECOND,
    #[token("SECONDARY", ignore(ascii_case))]
    SECONDARY,
    #[token("SECONDARY_ENGINE", ignore(ascii_case))]
    SECONDARY_ENGINE,
    #[token("SECONDARY_LOAD", ignore(ascii_case))]
    SECONDARY_LOAD,
    #[token("SECONDARY_UNLOAD", ignore(ascii_case))]
    SECONDARY_UNLOAD,
    #[token("SECURITY", ignore(ascii_case))]
    SECURITY,
    #[token("SEND_CREDENTIALS_TO_TIKV", ignore(ascii_case))]
    SEND_CREDENTIALS_TO_TIKV,
    #[token("SEPARATOR", ignore(ascii_case))]
    SEPARATOR,
    #[token("SEQUENCE", ignore(ascii_case))]
    SEQUENCE,
    #[token("SERIAL", ignore(ascii_case))]
    SERIAL,
    #[token("SERIALIZABLE", ignore(ascii_case))]
    SERIALIZABLE,
    #[token("SESSION", ignore(ascii_case))]
    SESSION,
    #[token("SETVAL", ignore(ascii_case))]
    SETVAL,
    #[token("SHARD_ROW_ID_BITS", ignore(ascii_case))]
    SHARD_ROW_ID_BITS,
    #[token("SHARE", ignore(ascii_case))]
    SHARE,
    #[token("SHARED", ignore(ascii_case))]
    SHARED,
    #[token("SHUTDOWN", ignore(ascii_case))]
    SHUTDOWN,
    #[token("SIGNED", ignore(ascii_case))]
    SIGNED,
    #[token("SIMPLE", ignore(ascii_case))]
    SIMPLE,
    #[token("SKIP", ignore(ascii_case))]
    SKIP,
    #[token("SKIP_SCHEMA_FILES", ignore(ascii_case))]
    SKIP_SCHEMA_FILES,
    #[token("SLAVE", ignore(ascii_case))]
    SLAVE,
    #[token("SLOW", ignore(ascii_case))]
    SLOW,
    #[token("SNAPSHOT", ignore(ascii_case))]
    SNAPSHOT,
    #[token("SOME", ignore(ascii_case))]
    SOME,
    #[token("SOURCE", ignore(ascii_case))]
    SOURCE,
    #[token("SQL_BUFFER_RESULT", ignore(ascii_case))]
    SQL_BUFFER_RESULT,
    #[token("SQL_CACHE", ignore(ascii_case))]
    SQL_CACHE,
    #[token("SQL_NO_CACHE", ignore(ascii_case))]
    SQL_NO_CACHE,
    #[token("SQL_TSI_DAY", ignore(ascii_case))]
    SQL_TSI_DAY,
    #[token("SQL_TSI_HOUR", ignore(ascii_case))]
    SQL_TSI_HOUR,
    #[token("SQL_TSI_MINUTE", ignore(ascii_case))]
    SQL_TSI_MINUTE,
    #[token("SQL_TSI_MONTH", ignore(ascii_case))]
    SQL_TSI_MONTH,
    #[token("SQL_TSI_QUARTER", ignore(ascii_case))]
    SQL_TSI_QUARTER,
    #[token("SQL_TSI_SECOND", ignore(ascii_case))]
    SQL_TSI_SECOND,
    #[token("SQL_TSI_WEEK", ignore(ascii_case))]
    SQL_TSI_WEEK,
    #[token("SQL_TSI_YEAR", ignore(ascii_case))]
    SQL_TSI_YEAR,
    #[token("START", ignore(ascii_case))]
    START,
    #[token("STATS_AUTO_RECALC", ignore(ascii_case))]
    STATS_AUTO_RECALC,
    #[token("STATS_COL_CHOICE", ignore(ascii_case))]
    STATS_COL_CHOICE,
    #[token("STATS_COL_LIST", ignore(ascii_case))]
    STATS_COL_LIST,
    #[token("STATS_OPTIONS", ignore(ascii_case))]
    STATS_OPTIONS,
    #[token("STATS_PERSISTENT", ignore(ascii_case))]
    STATS_PERSISTENT,
    #[token("STATS_SAMPLE_PAGES", ignore(ascii_case))]
    STATS_SAMPLE_PAGES,
    #[token("STATS_SAMPLE_RATE", ignore(ascii_case))]
    STATS_SAMPLE_RATE,
    #[token("STATUS", ignore(ascii_case))]
    STATUS,
    #[token("STORAGE", ignore(ascii_case))]
    STORAGE,
    #[token("STRICT_FORMAT", ignore(ascii_case))]
    STRICT_FORMAT,
    #[token("SUBJECT", ignore(ascii_case))]
    SUBJECT,
    #[token("SUBPARTITION", ignore(ascii_case))]
    SUBPARTITION,
    #[token("SUBPARTITIONS", ignore(ascii_case))]
    SUBPARTITIONS,
    #[token("SUBSTR", ignore(ascii_case))]
    SUBSTR,
    #[token("SUPER", ignore(ascii_case))]
    SUPER,
    #[token("SWAPS", ignore(ascii_case))]
    SWAPS,
    #[token("SWITCHES", ignore(ascii_case))]
    SWITCHES,
    #[token("SYSDATE", ignore(ascii_case))]
    SYSDATE,
    #[token("SYSTEM", ignore(ascii_case))]
    SYSTEM,
    #[token("SYSTEM_TIME", ignore(ascii_case))]
    SYSTEM_TIME,
    #[token("TABLES", ignore(ascii_case))]
    TABLES,
    #[token("TABLESPACE", ignore(ascii_case))]
    TABLESPACE,
    #[token("TABLE_CHECKSUM", ignore(ascii_case))]
    TABLE_CHECKSUM,
    #[token("TEMPORARY", ignore(ascii_case))]
    TEMPORARY,
    #[token("TEMPTABLE", ignore(ascii_case))]
    TEMPTABLE,
    #[token("TEXT", ignore(ascii_case))]
    TEXT,
    #[token("THAN", ignore(ascii_case))]
    THAN,
    #[token("TIKV_IMPORTER", ignore(ascii_case))]
    TIKV_IMPORTER,
    #[token("TIME", ignore(ascii_case))]
    TIME,
    #[token("TIMESTAMP", ignore(ascii_case))]
    TIMESTAMP,
    #[token("TOKEN_ISSUER", ignore(ascii_case))]
    TOKEN_ISSUER,
    #[token("TPCC", ignore(ascii_case))]
    TPCC,
    #[token("TPCH_10", ignore(ascii_case))]
    TPCH_10,
    #[token("TRACE", ignore(ascii_case))]
    TRACE,
    #[token("TRADITIONAL", ignore(ascii_case))]
    TRADITIONAL,
    #[token("TRANSACTION", ignore(ascii_case))]
    TRANSACTION,
    #[token("TRANSLATE", ignore(ascii_case))]
    TRANSLATE,
    #[token("TRIGGERS", ignore(ascii_case))]
    TRIGGERS,
    #[token("TRUNCATE", ignore(ascii_case))]
    TRUNCATE,
    #[token("TSO", ignore(ascii_case))]
    TSO,
    #[token("TTL", ignore(ascii_case))]
    TTL,
    #[token("TTL_ENABLE", ignore(ascii_case))]
    TTL_ENABLE,
    #[token("TTL_JOB_INTERVAL", ignore(ascii_case))]
    TTL_JOB_INTERVAL,
    #[token("TYPE", ignore(ascii_case))]
    TYPE,
    #[token("UNBOUNDED", ignore(ascii_case))]
    UNBOUNDED,
    #[token("UNCOMMITTED", ignore(ascii_case))]
    UNCOMMITTED,
    #[token("UNDEFINED", ignore(ascii_case))]
    UNDEFINED,
    #[token("UNICODE", ignore(ascii_case))]
    UNICODE,
    #[token("UNKNOWN", ignore(ascii_case))]
    UNKNOWN,
    #[token("UNSET", ignore(ascii_case))]
    UNSET,
    #[token("USER", ignore(ascii_case))]
    USER,
    #[token("VALIDATION", ignore(ascii_case))]
    VALIDATION,
    #[token("VALUE", ignore(ascii_case))]
    VALUE,
    #[token("VARIABLES", ignore(ascii_case))]
    VARIABLES,
    #[token("VECTOR", ignore(ascii_case))]
    VECTOR,
    #[token("VIEW", ignore(ascii_case))]
    VIEW,
    #[token("VISIBLE", ignore(ascii_case))]
    VISIBLE,
    #[token("WAIT", ignore(ascii_case))]
    WAIT,
    #[token("WAIT_TIFLASH_READY", ignore(ascii_case))]
    WAIT_TIFLASH_READY,
    #[token("WARNINGS", ignore(ascii_case))]
    WARNINGS,
    #[token("WEEK", ignore(ascii_case))]
    WEEK,
    #[token("WEIGHT_STRING", ignore(ascii_case))]
    WEIGHT_STRING,
    #[token("WITHOUT", ignore(ascii_case))]
    WITHOUT,
    #[token("WITH_SYS_TABLE", ignore(ascii_case))]
    WITH_SYS_TABLE,
    #[token("WORKLOAD", ignore(ascii_case))]
    WORKLOAD,
    #[token("X509", ignore(ascii_case))]
    X509,
    #[token("YEAR", ignore(ascii_case))]
    YEAR,
    #[token("ADMIN", ignore(ascii_case))]
    ADMIN,
    #[token("BATCH", ignore(ascii_case))]
    BATCH,
    #[token("BUCKETS", ignore(ascii_case))]
    BUCKETS,
    #[token("BUILTINS", ignore(ascii_case))]
    BUILTINS,
    #[token("CANCEL", ignore(ascii_case))]
    CANCEL,
    #[token("CARDINALITY", ignore(ascii_case))]
    CARDINALITY,
    #[token("CMSKETCH", ignore(ascii_case))]
    CMSKETCH,
    #[token("COLUMN_STATS_USAGE", ignore(ascii_case))]
    COLUMN_STATS_USAGE,
    #[token("CORRELATION", ignore(ascii_case))]
    CORRELATION,
    #[token("DDL", ignore(ascii_case))]
    DDL,
    #[token("DEPENDENCY", ignore(ascii_case))]
    DEPENDENCY,
    #[token("DEPTH", ignore(ascii_case))]
    DEPTH,
    #[token("DRY", ignore(ascii_case))]
    DRY,
    #[token("HISTOGRAMS_IN_FLIGHT", ignore(ascii_case))]
    HISTOGRAMS_IN_FLIGHT,
    #[token("JOB", ignore(ascii_case))]
    JOB,
    #[token("JOBS", ignore(ascii_case))]
    JOBS,
    #[token("NODE_ID", ignore(ascii_case))]
    NODE_ID,
    #[token("NODE_STATE", ignore(ascii_case))]
    NODE_STATE,
    #[token("OPTIMISTIC", ignore(ascii_case))]
    OPTIMISTIC,
    #[token("PESSIMISTIC", ignore(ascii_case))]
    PESSIMISTIC,
    #[token("REGION", ignore(ascii_case))]
    REGION,
    #[token("REGIONS", ignore(ascii_case))]
    REGIONS,
    #[token("RESET", ignore(ascii_case))]
    RESET,
    #[token("RUN", ignore(ascii_case))]
    RUN,
    #[token("SAMPLERATE", ignore(ascii_case))]
    SAMPLERATE,
    #[token("SAMPLES", ignore(ascii_case))]
    SAMPLES,
    #[token("SESSION_STATES", ignore(ascii_case))]
    SESSION_STATES,
    #[token("SPLIT", ignore(ascii_case))]
    SPLIT,
    #[token("STATISTICS", ignore(ascii_case))]
    STATISTICS,
    #[token("STATS", ignore(ascii_case))]
    STATS,
    #[token("STATS_BUCKETS", ignore(ascii_case))]
    STATS_BUCKETS,
    #[token("STATS_EXTENDED", ignore(ascii_case))]
    STATS_EXTENDED,
    #[token("STATS_HEALTHY", ignore(ascii_case))]
    STATS_HEALTHY,
    #[token("STATS_HISTOGRAMS", ignore(ascii_case))]
    STATS_HISTOGRAMS,
    #[token("STATS_LOCKED", ignore(ascii_case))]
    STATS_LOCKED,
    #[token("STATS_META", ignore(ascii_case))]
    STATS_META,
    #[token("STATS_TOPN", ignore(ascii_case))]
    STATS_TOPN,
    #[token("TIDB", ignore(ascii_case))]
    TIDB,
    #[token("TIFLASH", ignore(ascii_case))]
    TIFLASH,
    #[token("TOPN", ignore(ascii_case))]
    TOPN,
    #[token("WIDTH", ignore(ascii_case))]
    WIDTH,
    #[token("ADDDATE", ignore(ascii_case))]
    ADDDATE,
    #[token("APPROX_COUNT_DISTINCT", ignore(ascii_case))]
    APPROX_COUNT_DISTINCT,
    #[token("APPROX_PERCENTILE", ignore(ascii_case))]
    APPROX_PERCENTILE,
    #[token("BIT_AND", ignore(ascii_case))]
    BIT_AND,
    #[token("BIT_OR", ignore(ascii_case))]
    BIT_OR,
    #[token("BIT_XOR", ignore(ascii_case))]
    BIT_XOR,
    #[token("BRIEF", ignore(ascii_case))]
    BRIEF,
    #[token("CAST", ignore(ascii_case))]
    CAST,
    #[token("COPY", ignore(ascii_case))]
    COPY,
    #[token("CURTIME", ignore(ascii_case))]
    CURTIME,
    #[token("CURDATE", ignore(ascii_case))]
    CURDATE,
    #[token("DATE_ADD", ignore(ascii_case))]
    DATE_ADD,
    #[token("DATE_SUB", ignore(ascii_case))]
    DATE_SUB,
    #[token("DEFINED", ignore(ascii_case))]
    DEFINED,
    #[token("DOT", ignore(ascii_case))]
    DOT,
    #[token("DUMP", ignore(ascii_case))]
    DUMP,
    #[token("DURATION", ignore(ascii_case))]
    DURATION,
    #[token("EXTRACT", ignore(ascii_case))]
    EXTRACT,
    #[token("END_TIME", ignore(ascii_case))]
    END_TIME,
    #[token("GET_FORMAT", ignore(ascii_case))]
    GET_FORMAT,
    #[token("GROUP_CONCAT", ignore(ascii_case))]
    GROUP_CONCAT,
    #[token("HNSW", ignore(ascii_case))]
    HNSW,
    #[token("INPLACE", ignore(ascii_case))]
    INPLACE,
    #[token("INSTANT", ignore(ascii_case))]
    INSTANT,
    #[token("INTERNAL", ignore(ascii_case))]
    INTERNAL,
    #[token("MIN", ignore(ascii_case))]
    MIN,
    #[token("MAX", ignore(ascii_case))]
    MAX,
    #[token("RECENT", ignore(ascii_case))]
    RECENT,
    #[token("REPLAYER", ignore(ascii_case))]
    REPLAYER,
    #[token("RUNNING", ignore(ascii_case))]
    RUNNING,
    #[token("PLACEMENT", ignore(ascii_case))]
    PLACEMENT,
    #[token("PLAN", ignore(ascii_case))]
    PLAN,
    #[token("PLAN_CACHE", ignore(ascii_case))]
    PLAN_CACHE,
    #[token("POSITION", ignore(ascii_case))]
    POSITION,
    #[token("PREDICATE", ignore(ascii_case))]
    PREDICATE,
    #[token("S3", ignore(ascii_case))]
    S3,
    #[token("STRICT", ignore(ascii_case))]
    STRICT,
    #[token("SUBDATE", ignore(ascii_case))]
    SUBDATE,
    #[token("SUBSTRING", ignore(ascii_case))]
    SUBSTRING,
    #[token("SUM", ignore(ascii_case))]
    SUM,
    #[token("START_TIME", ignore(ascii_case))]
    START_TIME,
    #[token("STD", ignore(ascii_case))]
    STD,
    #[token("STDDEV", ignore(ascii_case))]
    STDDEV,
    #[token("STDDEV_POP", ignore(ascii_case))]
    STDDEV_POP,
    #[token("STDDEV_SAMP", ignore(ascii_case))]
    STDDEV_SAMP,
    #[token("STOP", ignore(ascii_case))]
    STOP,
    #[token("VARIANCE", ignore(ascii_case))]
    VARIANCE,
    #[token("VAR_POP", ignore(ascii_case))]
    VAR_POP,
    #[token("VAR_SAMP", ignore(ascii_case))]
    VAR_SAMP,
    #[token("TARGET", ignore(ascii_case))]
    TARGET,
    #[token("TIMESTAMPADD", ignore(ascii_case))]
    TIMESTAMPADD,
    #[token("TIMESTAMPDIFF", ignore(ascii_case))]
    TIMESTAMPDIFF,
    #[token("TOKUDB_DEFAULT", ignore(ascii_case))]
    TOKUDB_DEFAULT,
    #[token("TOKUDB_FAST", ignore(ascii_case))]
    TOKUDB_FAST,
    #[token("TOKUDB_LZMA", ignore(ascii_case))]
    TOKUDB_LZMA,
    #[token("TOKUDB_QUICKLZ", ignore(ascii_case))]
    TOKUDB_QUICKLZ,
    #[token("TOKUDB_SNAPPY", ignore(ascii_case))]
    TOKUDB_SNAPPY,
    #[token("TOKUDB_SMALL", ignore(ascii_case))]
    TOKUDB_SMALL,
    #[token("TOKUDB_UNCOMPRESSED", ignore(ascii_case))]
    TOKUDB_UNCOMPRESSED,
    #[token("TOKUDB_ZLIB", ignore(ascii_case))]
    TOKUDB_ZLIB,
    #[token("TOKUDB_ZSTD", ignore(ascii_case))]
    TOKUDB_ZSTD,
    #[token("TOP", ignore(ascii_case))]
    TOP,
    #[token("TRIM", ignore(ascii_case))]
    TRIM,
    #[token("NEXT_ROW_ID", ignore(ascii_case))]
    NEXT_ROW_ID,
    #[token("EXPR_PUSHDOWN_BLACKLIST", ignore(ascii_case))]
    EXPR_PUSHDOWN_BLACKLIST,
    #[token("OPT_RULE_BLACKLIST", ignore(ascii_case))]
    OPT_RULE_BLACKLIST,
    #[token("BOUND", ignore(ascii_case))]
    BOUND,
    #[token("EXACT", ignore(ascii_case))]
    EXACT,
    #[token("STALENESS", ignore(ascii_case))]
    STALENESS,
    #[token("STRONG", ignore(ascii_case))]
    STRONG,
    #[token("FLASHBACK", ignore(ascii_case))]
    FLASHBACK,
    #[token("JSON_OBJECTAGG", ignore(ascii_case))]
    JSON_OBJECTAGG,
    #[token("JSON_ARRAYAGG", ignore(ascii_case))]
    JSON_ARRAYAGG,
    #[token("TLS", ignore(ascii_case))]
    TLS,
    #[token("FOLLOWER", ignore(ascii_case))]
    FOLLOWER,
    #[token("FOLLOWERS", ignore(ascii_case))]
    FOLLOWERS,
    #[token("LEADER", ignore(ascii_case))]
    LEADER,
    #[token("LEARNER", ignore(ascii_case))]
    LEARNER,
    #[token("LEARNERS", ignore(ascii_case))]
    LEARNERS,
    #[token("VERBOSE", ignore(ascii_case))]
    VERBOSE,
    #[token("TRUE_CARD_COST", ignore(ascii_case))]
    TRUE_CARD_COST,
    #[token("VOTER", ignore(ascii_case))]
    VOTER,
    #[token("VOTERS", ignore(ascii_case))]
    VOTERS,
    #[token("CONSTRAINTS", ignore(ascii_case))]
    CONSTRAINTS,
    #[token("PRIMARY_REGION", ignore(ascii_case))]
    PRIMARY_REGION,
    #[token("SCHEDULE", ignore(ascii_case))]
    SCHEDULE,
    #[token("SCHEMA", ignore(ascii_case))]
    SCHEMA,
    #[token("SCHEMAS", ignore(ascii_case))]
    SCHEMAS,
    #[token("SURVIVAL_PREFERENCES", ignore(ascii_case))]
    SURVIVAL_PREFERENCES,
    #[token("LEADER_CONSTRAINTS", ignore(ascii_case))]
    LEADER_CONSTRAINTS,
    #[token("FOLLOWER_CONSTRAINTS", ignore(ascii_case))]
    FOLLOWER_CONSTRAINTS,
    #[token("LEARNER_CONSTRAINTS", ignore(ascii_case))]
    LEARNER_CONSTRAINTS,
    #[token("VOTER_CONSTRAINTS", ignore(ascii_case))]
    VOTER_CONSTRAINTS,
    #[token("TIDB_JSON", ignore(ascii_case))]
    TIDB_JSON,
    #[token("IO_READ_BANDWIDTH", ignore(ascii_case))]
    IO_READ_BANDWIDTH,
    #[token("IO_WRITE_BANDWIDTH", ignore(ascii_case))]
    IO_WRITE_BANDWIDTH,
    #[token("RU_PER_SEC", ignore(ascii_case))]
    RU_PER_SEC,
    #[token("PRIORITY", ignore(ascii_case))]
    PRIORITY,
    #[token("HIGH", ignore(ascii_case))]
    HIGH,
    #[token("MEDIUM", ignore(ascii_case))]
    MEDIUM,
    #[token("LOW", ignore(ascii_case))]
    LOW,
    #[token("BURSTABLE", ignore(ascii_case))]
    BURSTABLE,
    #[token("BR", ignore(ascii_case))]
    BR,
    #[token("GC_TTL", ignore(ascii_case))]
    GC_TTL,
    #[token("METADATA", ignore(ascii_case))]
    METADATA,
    #[token("START_TS", ignore(ascii_case))]
    START_TS,
    #[token("UNTIL_TS", ignore(ascii_case))]
    UNTIL_TS,
    #[token("RESTORED_TS", ignore(ascii_case))]
    RESTORED_TS,
    #[token("FULL_BACKUP_STORAGE", ignore(ascii_case))]
    FULL_BACKUP_STORAGE,
    #[token("EXEC_ELAPSED", ignore(ascii_case))]
    EXEC_ELAPSED,
    #[token("PROCESSED_KEYS", ignore(ascii_case))]
    PROCESSED_KEYS,
    #[token("RU", ignore(ascii_case))]
    RU,
    #[token("DRYRUN", ignore(ascii_case))]
    DRYRUN,
    #[token("COOLDOWN", ignore(ascii_case))]
    COOLDOWN,
    #[token("SWITCH_GROUP", ignore(ascii_case))]
    SWITCH_GROUP,
    #[token("WATCH", ignore(ascii_case))]
    WATCH,
    #[token("SIMILAR", ignore(ascii_case))]
    SIMILAR,
    #[token("QUERY_LIMIT", ignore(ascii_case))]
    QUERY_LIMIT,
    #[token("BACKGROUND", ignore(ascii_case))]
    BACKGROUND,
    #[token("TASK_TYPES", ignore(ascii_case))]
    TASK_TYPES,
    #[token("UNLIMITED", ignore(ascii_case))]
    UNLIMITED,
    #[token("UTILIZATION_LIMIT", ignore(ascii_case))]
    UTILIZATION_LIMIT,
}

impl TokenKind {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::LiteralInteger
            | Self::LiteralFloat
            | Self::LiteralString
            | Self::LiteralHex
            | Self::SingleAtIdent => true,
            _ => false,
        }
    }

    pub fn is_keyword(&self) -> bool {
        !match self {
            Self::Ident
            | Self::LiteralString
            | Self::LiteralHex
            | Self::LiteralInteger
            | Self::LiteralFloat
            | Self::HintPrefix
            | Self::HintSuffix
            | Self::DoubleEq
            | Self::Eq
            | Self::NotEq
            | Self::Lt
            | Self::Gt
            | Self::Lte
            | Self::Gte
            | Self::Spaceship
            | Self::Plus
            | Self::Minus
            | Self::Multiply
            | Self::Divide
            | Self::IntDiv
            | Self::Modulo
            | Self::PipesAsOr
            | Self::LParen
            | Self::RParen
            | Self::Comma
            | Self::Dot
            | Self::Colon
            | Self::DoubleColon
            | Self::AssignmentEq
            | Self::SemiColon
            | Self::Backslash
            | Self::LBracket
            | Self::RBracket
            | Self::BitWiseAnd
            | Self::BitWiseOr
            | Self::Caret
            | Self::Factorial
            | Self::LBrace
            | Self::RBrace
            | Self::Dollar
            | Self::RArrow
            | Self::LongRArrow
            | Self::HashRArrow
            | Self::HashLongRArrow
            | Self::FatRArrow
            | Self::BitWiseXor
            | Self::BitWiseNot
            | Self::TildeAsterisk
            | Self::ExclamationMarkTilde
            | Self::ExclamationMarkTildeAsterisk
            | Self::ShiftLeft
            | Self::ShiftRight
            | Self::DoubleExclamationMark
            | Self::Abs
            | Self::SquareRoot
            | Self::CubeRoot
            | Self::Placeholder
            | Self::QuestionOr
            | Self::QuestionAnd
            | Self::ArrowAt
            | Self::AtArrow
            | Self::AtQuestion
            | Self::AtAt
            | Self::HashMinus
            | Self::EOI => true,
            _ => false,
        }
    }

    pub fn is_reserved_ident(&self, after_as: bool) -> bool {
        match self {
            Self::ADD
            | Self::ALL
            | Self::ALTER
            | Self::ANALYZE
            | Self::AND
            | Self::ARRAY
            | Self::AS
            | Self::ASC
            | Self::BETWEEN
            | Self::BIGINT
            | Self::BINARY
            | Self::BLOB
            | Self::BOTH
            | Self::BY
            | Self::CALL
            | Self::CASCADE
            | Self::CASE
            | Self::CHANGE
            | Self::CHAR
            | Self::CHARACTER
            | Self::CHECK
            | Self::COLLATE
            | Self::COLUMN
            | Self::CONSTRAINT
            | Self::CONTINUE
            | Self::CONVERT
            | Self::CREATE
            | Self::CROSS
            | Self::CUME_DIST
            | Self::CURRENT_DATE
            | Self::CURRENT_ROLE
            | Self::CURRENT_TIME
            | Self::CURRENT_TIMESTAMP
            | Self::CURRENT_USER
            | Self::CURSOR
            | Self::DATABASE
            | Self::DATABASES
            | Self::DAY_HOUR
            | Self::DAY_MICROSECOND
            | Self::DAY_MINUTE
            | Self::DAY_SECOND
            | Self::DECIMAL
            | Self::DEFAULT
            | Self::DELAYED
            | Self::DELETE
            | Self::DENSE_RANK
            | Self::DESC
            | Self::DESCRIBE
            | Self::DISTINCT
            | Self::DISTINCTROW
            | Self::DIV
            | Self::DOUBLE
            | Self::DROP
            | Self::DUAL
            | Self::ELSE
            | Self::ELSEIF
            | Self::ENCLOSED
            | Self::ESCAPED
            | Self::EXCEPT
            | Self::EXISTS
            | Self::EXIT
            | Self::EXPLAIN
            | Self::FALSE
            | Self::FETCH
            | Self::FIRST_VALUE
            | Self::FLOAT
            | Self::FLOAT4
            | Self::FLOAT8
            | Self::FOR
            | Self::FORCE
            | Self::FOREIGN
            | Self::FROM
            | Self::FULLTEXT
            | Self::GENERATED
            | Self::GRANT
            | Self::GROUP
            | Self::GROUPS
            | Self::HAVING
            | Self::HIGH_PRIORITY
            | Self::HOUR_MICROSECOND
            | Self::HOUR_MINUTE
            | Self::HOUR_SECOND
            | Self::IF
            | Self::IGNORE
            | Self::ILIKE
            | Self::IN
            | Self::INDEX
            | Self::INFILE
            | Self::INNER
            | Self::INOUT
            | Self::INSERT
            | Self::INT
            | Self::INT1
            | Self::INT2
            | Self::INT3
            | Self::INT4
            | Self::INT8
            | Self::INTEGER
            | Self::INTERSECT
            | Self::INTERVAL
            | Self::INTO
            | Self::IS
            | Self::ITERATE
            | Self::JOIN
            | Self::KEY
            | Self::KEYS
            | Self::KILL
            | Self::LAG
            | Self::LAST_VALUE
            | Self::LEAD
            | Self::LEADING
            | Self::LEAVE
            | Self::LEFT
            | Self::LIKE
            | Self::LIMIT
            | Self::LINEAR
            | Self::LINES
            | Self::LOAD
            | Self::LOCALTIME
            | Self::LOCALTIMESTAMP
            | Self::LOCK
            | Self::LONG
            | Self::LONGBLOB
            | Self::LONGTEXT
            | Self::LOW_PRIORITY
            | Self::MATCH
            | Self::MAXVALUE
            | Self::MEDIUMBLOB
            | Self::MEDIUMINT
            | Self::MEDIUMTEXT
            | Self::MIDDLEINT
            | Self::MINUTE_MICROSECOND
            | Self::MINUTE_SECOND
            | Self::MOD
            | Self::NATURAL
            | Self::NOT
            | Self::NO_WRITE_TO_BINLOG
            | Self::NTH_VALUE
            | Self::NTILE
            | Self::NULL
            | Self::NUMERIC
            | Self::OF
            | Self::ON
            | Self::OPTIMIZE
            | Self::OPTION
            | Self::OPTIONALLY
            | Self::OR
            | Self::ORDER
            | Self::OUT
            | Self::OUTER
            | Self::OUTFILE
            | Self::OVER
            | Self::PARTITION
            | Self::PERCENT_RANK
            | Self::PRECISION
            | Self::PRIMARY
            | Self::PROCEDURE
            | Self::RANGE
            | Self::RANK
            | Self::READ
            | Self::REAL
            | Self::RECURSIVE
            | Self::REFERENCES
            | Self::REGEXP
            | Self::RELEASE
            | Self::RENAME
            | Self::REPEAT
            | Self::REPLACE
            | Self::REQUIRE
            | Self::RESTRICT
            | Self::REVOKE
            | Self::RIGHT
            | Self::RLIKE
            | Self::ROW
            | Self::ROWS
            | Self::ROW_NUMBER
            | Self::SECOND_MICROSECOND
            | Self::SELECT
            | Self::SET
            | Self::SHOW
            | Self::SMALLINT
            | Self::SPATIAL
            | Self::SQL
            | Self::SQLEXCEPTION
            | Self::SQLSTATE
            | Self::SQLWARNING
            | Self::SQL_BIG_RESULT
            | Self::SQL_CALC_FOUND_ROWS
            | Self::SQL_SMALL_RESULT
            | Self::SSL
            | Self::STARTING
            | Self::STORED
            | Self::STRAIGHT_JOIN
            | Self::TABLE
            | Self::TABLESAMPLE
            | Self::TERMINATED
            | Self::THEN
            | Self::TIDB_CURRENT_TSO
            | Self::TINYBLOB
            | Self::TINYINT
            | Self::TINYTEXT
            | Self::TO
            | Self::TRAILING
            | Self::TRIGGER
            | Self::TRUE
            | Self::UNION
            | Self::UNIQUE
            | Self::UNLOCK
            | Self::UNSIGNED
            | Self::UNTIL
            | Self::UPDATE
            | Self::USAGE
            | Self::USE
            | Self::USING
            | Self::UTC_DATE
            | Self::UTC_TIME
            | Self::UTC_TIMESTAMP
            | Self::VALUES
            | Self::VARBINARY
            | Self::VARCHAR
            | Self::VARCHARACTER
            | Self::VARYING
            | Self::VIRTUAL
            | Self::WHEN
            | Self::WHERE
            | Self::WHILE
            | Self::WINDOW
            | Self::WITH
            | Self::WRITE
            | Self::XOR
            | Self::YEAR_MONTH
            | Self::ZEROFILL
                if !after_as =>
            {
                true
            }
            _ => false,
        }
    }

    pub fn all_reserved_keywords() -> Vec<String> {
        let mut result = Vec::new();
        for token in Self::iter() {
            result.push(format!("{:?}", token));
        }
        result
    }
}
