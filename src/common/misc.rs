use crate::parser::token_kind::TokenKind;
use lazy_static::lazy_static;
use std::collections::HashMap;

// See https://dev.mysql.com/doc/refman/5.7/en/identifiers.html
pub fn is_in_correct_identifier_name(name: &str) -> bool {
    if name.is_empty() {
        return true;
    }
    if name.ends_with(' ') {
        return true;
    }

    false
}

// isInTokenMap indicates whether the target string is contained in tokenMap.
pub fn is_in_token_map(target: &str) -> bool {
    if token_map.get(target).is_none() {
        return false;
    }

    true
}

// tokenMap is a map of known identifiers to the parser token ID.
// Please try to keep the map in alphabetical order.
lazy_static! {
    pub static ref token_map: HashMap<String, TokenKind> = {
        let mut s = HashMap::<String, TokenKind>::new();
        s.insert("ACCOUNT".to_string(), TokenKind::ACCOUNT);
        s.insert("ACTION".to_string(), TokenKind::ACTION);
        s.insert("ADD".to_string(), TokenKind::ADD);
        s.insert("ADDDATE".to_string(), TokenKind::ADDDATE);
        s.insert("ADMIN".to_string(), TokenKind::ADMIN);
        s.insert("ADVISE".to_string(), TokenKind::ADVISE);
        s.insert("AFTER".to_string(), TokenKind::AFTER);
        s.insert("AGAINST".to_string(), TokenKind::AGAINST);
        s.insert("AGO".to_string(), TokenKind::AGO);
        s.insert("ALGORITHM".to_string(), TokenKind::ALGORITHM);
        s.insert("ALL".to_string(), TokenKind::ALL);
        s.insert("ALTER".to_string(), TokenKind::ALTER);
        s.insert("ALWAYS".to_string(), TokenKind::ALWAYS);
        s.insert("ANALYZE".to_string(), TokenKind::ANALYZE);
        s.insert("AND".to_string(), TokenKind::AND);
        s.insert("ANY".to_string(), TokenKind::ANY);
        s.insert(
            "APPROX_COUNT_DISTINCT".to_string(),
            TokenKind::APPROX_COUNT_DISTINCT,
        );
        s.insert(
            "APPROX_PERCENTILE".to_string(),
            TokenKind::APPROX_PERCENTILE,
        );
        s.insert("ARRAY".to_string(), TokenKind::ARRAY);
        s.insert("AS".to_string(), TokenKind::AS);
        s.insert("ASC".to_string(), TokenKind::ASC);
        s.insert("ASCII".to_string(), TokenKind::ASCII);
        s.insert("APPLY".to_string(), TokenKind::APPLY);
        s.insert("ATTRIBUTE".to_string(), TokenKind::ATTRIBUTE);
        s.insert("ATTRIBUTES".to_string(), TokenKind::ATTRIBUTES);
        s.insert("BATCH".to_string(), TokenKind::BATCH);
        s.insert("BACKGROUND".to_string(), TokenKind::BACKGROUND);
        s.insert("STATS_OPTIONS".to_string(), TokenKind::STATS_OPTIONS);
        s.insert(
            "STATS_SAMPLE_RATE".to_string(),
            TokenKind::STATS_SAMPLE_RATE,
        );
        s.insert("STATS_COL_CHOICE".to_string(), TokenKind::STATS_COL_CHOICE);
        s.insert("STATS_COL_LIST".to_string(), TokenKind::STATS_COL_LIST);
        s.insert("AUTO_ID_CACHE".to_string(), TokenKind::AUTO_ID_CACHE);
        s.insert("AUTO_INCREMENT".to_string(), TokenKind::AUTO_INCREMENT);
        s.insert("AUTO_RANDOM".to_string(), TokenKind::AUTO_RANDOM);
        s.insert("AUTO_RANDOM_BASE".to_string(), TokenKind::AUTO_RANDOM_BASE);
        s.insert("AVG_ROW_LENGTH".to_string(), TokenKind::AVG_ROW_LENGTH);
        s.insert("AVG".to_string(), TokenKind::AVG);
        s.insert("BACKEND".to_string(), TokenKind::BACKEND);
        s.insert("BACKUP".to_string(), TokenKind::BACKUP);
        s.insert("BACKUPS".to_string(), TokenKind::BACKUPS);
        s.insert("BDR".to_string(), TokenKind::BDR);
        s.insert("BEGIN".to_string(), TokenKind::BEGIN);
        s.insert("BETWEEN".to_string(), TokenKind::BETWEEN);
        s.insert("BERNOULLI".to_string(), TokenKind::BERNOULLI);
        s.insert("BIGINT".to_string(), TokenKind::BIGINT);
        s.insert("BINARY".to_string(), TokenKind::BINARY);
        s.insert("BINDING".to_string(), TokenKind::BINDING);
        s.insert("BINDING_CACHE".to_string(), TokenKind::BINDING_CACHE);
        s.insert("BINDINGS".to_string(), TokenKind::BINDINGS);
        s.insert("BINLOG".to_string(), TokenKind::BINLOG);
        s.insert("BIT_AND".to_string(), TokenKind::BIT_AND);
        s.insert("BIT_OR".to_string(), TokenKind::BIT_OR);
        s.insert("BIT_XOR".to_string(), TokenKind::BIT_XOR);
        s.insert("BIT".to_string(), TokenKind::BIT);
        s.insert("BLOB".to_string(), TokenKind::BLOB);
        s.insert("BLOCK".to_string(), TokenKind::BLOCK);
        s.insert("BOOL".to_string(), TokenKind::BOOL);
        s.insert("BOOLEAN".to_string(), TokenKind::BOOLEAN);
        s.insert("BOTH".to_string(), TokenKind::BOTH);
        s.insert("BOUND".to_string(), TokenKind::BOUND);
        s.insert("BR".to_string(), TokenKind::BR);
        s.insert("BRIEF".to_string(), TokenKind::BRIEF);
        s.insert("BTREE".to_string(), TokenKind::BTREE);
        s.insert("BUCKETS".to_string(), TokenKind::BUCKETS);
        s.insert("BUILTINS".to_string(), TokenKind::BUILTINS);
        s.insert("BURSTABLE".to_string(), TokenKind::BURSTABLE);
        s.insert("BY".to_string(), TokenKind::BY);
        s.insert("BYTE".to_string(), TokenKind::BYTE);
        s.insert("CACHE".to_string(), TokenKind::CACHE);
        s.insert("CALIBRATE".to_string(), TokenKind::CALIBRATE);
        s.insert("CALL".to_string(), TokenKind::CALL);
        s.insert("CANCEL".to_string(), TokenKind::CANCEL);
        s.insert("CAPTURE".to_string(), TokenKind::CAPTURE);
        s.insert("CARDINALITY".to_string(), TokenKind::CARDINALITY);
        s.insert("CASCADE".to_string(), TokenKind::CASCADE);
        s.insert("CASCADED".to_string(), TokenKind::CASCADED);
        s.insert("CASE".to_string(), TokenKind::CASE);
        s.insert("CAST".to_string(), TokenKind::CAST);
        s.insert("CAUSAL".to_string(), TokenKind::CAUSAL);
        s.insert("CHAIN".to_string(), TokenKind::CHAIN);
        s.insert("CHANGE".to_string(), TokenKind::CHANGE);
        s.insert("CHAR".to_string(), TokenKind::CHAR);
        s.insert("CHARACTER".to_string(), TokenKind::CHARACTER);
        s.insert("CHARSET".to_string(), TokenKind::CHARSET);
        s.insert("CHECK".to_string(), TokenKind::CHECK);
        s.insert("CHECKPOINT".to_string(), TokenKind::CHECKPOINT);
        s.insert("CHECKSUM".to_string(), TokenKind::CHECKSUM);
        s.insert("CIPHER".to_string(), TokenKind::CIPHER);
        s.insert("CLEANUP".to_string(), TokenKind::CLEANUP);
        s.insert("CLIENT".to_string(), TokenKind::CLIENT);
        s.insert(
            "CLIENT_ERRORS_SUMMARY".to_string(),
            TokenKind::CLIENT_ERRORS_SUMMARY,
        );
        s.insert("CLOSE".to_string(), TokenKind::CLOSE);
        s.insert("CLUSTER".to_string(), TokenKind::CLUSTER);
        s.insert("CLUSTERED".to_string(), TokenKind::CLUSTERED);
        s.insert("CMSKETCH".to_string(), TokenKind::CMSKETCH);
        s.insert("COALESCE".to_string(), TokenKind::COALESCE);
        s.insert("COLLATE".to_string(), TokenKind::COLLATE);
        s.insert("COLLATION".to_string(), TokenKind::COLLATION);
        s.insert("COLUMN_FORMAT".to_string(), TokenKind::COLUMN_FORMAT);
        s.insert(
            "COLUMN_STATS_USAGE".to_string(),
            TokenKind::COLUMN_STATS_USAGE,
        );
        s.insert("COLUMN".to_string(), TokenKind::COLUMN);
        s.insert("COLUMNS".to_string(), TokenKind::COLUMNS);
        s.insert("COMMENT".to_string(), TokenKind::COMMENT);
        s.insert("COMMIT".to_string(), TokenKind::COMMIT);
        s.insert("COMMITTED".to_string(), TokenKind::COMMITTED);
        s.insert("COMPACT".to_string(), TokenKind::COMPACT);
        s.insert("COMPRESSED".to_string(), TokenKind::COMPRESSED);
        s.insert("COMPRESSION".to_string(), TokenKind::COMPRESSION);
        s.insert("CONCURRENCY".to_string(), TokenKind::CONCURRENCY);
        s.insert("CONFIG".to_string(), TokenKind::CONFIG);
        s.insert("CONNECTION".to_string(), TokenKind::CONNECTION);
        s.insert("CONSISTENCY".to_string(), TokenKind::CONSISTENCY);
        s.insert("CONSISTENT".to_string(), TokenKind::CONSISTENT);
        s.insert("CONSTRAINT".to_string(), TokenKind::CONSTRAINT);
        s.insert("CONSTRAINTS".to_string(), TokenKind::CONSTRAINTS);
        s.insert("CONTEXT".to_string(), TokenKind::CONTEXT);
        s.insert("CONTINUE".to_string(), TokenKind::CONTINUE);
        s.insert("CONVERT".to_string(), TokenKind::CONVERT);
        s.insert("COOLDOWN".to_string(), TokenKind::COOLDOWN);
        s.insert("COPY".to_string(), TokenKind::COPY);
        s.insert("CORRELATION".to_string(), TokenKind::CORRELATION);
        s.insert("CPU".to_string(), TokenKind::CPU);
        s.insert("CREATE".to_string(), TokenKind::CREATE);
        s.insert("CROSS".to_string(), TokenKind::CROSS);
        s.insert(
            "CSV_BACKSLASH_ESCAPE".to_string(),
            TokenKind::CSV_BACKSLASH_ESCAPE,
        );
        s.insert("CSV_DELIMITER".to_string(), TokenKind::CSV_DELIMITER);
        s.insert("CSV_HEADER".to_string(), TokenKind::CSV_HEADER);
        s.insert("CSV_NOT_NULL".to_string(), TokenKind::CSV_NOT_NULL);
        s.insert("CSV_NULL".to_string(), TokenKind::CSV_NULL);
        s.insert("CSV_SEPARATOR".to_string(), TokenKind::CSV_SEPARATOR);
        s.insert(
            "CSV_TRIM_LAST_SEPARATORS".to_string(),
            TokenKind::CSV_TRIM_LAST_SEPARATORS,
        );
        s.insert(
            "WAIT_TIFLASH_READY".to_string(),
            TokenKind::WAIT_TIFLASH_READY,
        );
        s.insert("WITH_SYS_TABLE".to_string(), TokenKind::WITH_SYS_TABLE);
        s.insert("IGNORE_STATS".to_string(), TokenKind::IGNORE_STATS);
        s.insert("LOAD_STATS".to_string(), TokenKind::LOAD_STATS);
        s.insert(
            "CHECKSUM_CONCURRENCY".to_string(),
            TokenKind::CHECKSUM_CONCURRENCY,
        );
        s.insert(
            "COMPRESSION_LEVEL".to_string(),
            TokenKind::COMPRESSION_LEVEL,
        );
        s.insert("COMPRESSION_TYPE".to_string(), TokenKind::COMPRESSION_TYPE);
        s.insert(
            "ENCRYPTION_METHOD".to_string(),
            TokenKind::ENCRYPTION_METHOD,
        );
        s.insert(
            "ENCRYPTION_KEYFILE".to_string(),
            TokenKind::ENCRYPTION_KEYFILE,
        );
        s.insert("CURDATE".to_string(), TokenKind::CURDATE);
        s.insert("CURRENT_DATE".to_string(), TokenKind::CURRENT_DATE);
        s.insert("CURRENT_ROLE".to_string(), TokenKind::CURRENT_ROLE);
        s.insert("CURRENT_TIME".to_string(), TokenKind::CURRENT_TIME);
        s.insert(
            "CURRENT_TIMESTAMP".to_string(),
            TokenKind::CURRENT_TIMESTAMP,
        );
        s.insert("CURRENT_USER".to_string(), TokenKind::CURRENT_USER);
        s.insert("CURRENT".to_string(), TokenKind::CURRENT);
        s.insert("CURSOR".to_string(), TokenKind::CURSOR);
        s.insert("CURTIME".to_string(), TokenKind::CURTIME);
        s.insert("CYCLE".to_string(), TokenKind::CYCLE);
        s.insert("DATA".to_string(), TokenKind::DATA);
        s.insert("DATABASE".to_string(), TokenKind::DATABASE);
        s.insert("DATABASES".to_string(), TokenKind::DATABASES);
        s.insert("DATE_ADD".to_string(), TokenKind::DATE_ADD);
        s.insert("DATE_SUB".to_string(), TokenKind::DATE_SUB);
        s.insert("DATE".to_string(), TokenKind::DATE);
        s.insert("DATETIME".to_string(), TokenKind::DATETIME);
        s.insert("DAY_HOUR".to_string(), TokenKind::DAY_HOUR);
        s.insert("DAY_MICROSECOND".to_string(), TokenKind::DAY_MICROSECOND);
        s.insert("DAY_MINUTE".to_string(), TokenKind::DAY_MINUTE);
        s.insert("DAY_SECOND".to_string(), TokenKind::DAY_SECOND);
        s.insert("DAY".to_string(), TokenKind::DAY);
        s.insert("DDL".to_string(), TokenKind::DDL);
        s.insert("DEALLOCATE".to_string(), TokenKind::DEALLOCATE);
        s.insert("DEC".to_string(), TokenKind::DEC);
        s.insert("DECIMAL".to_string(), TokenKind::DECIMAL);
        s.insert("DECLARE".to_string(), TokenKind::DECLARE);
        s.insert("DEFAULT".to_string(), TokenKind::DEFAULT);
        s.insert("DEFINED".to_string(), TokenKind::DEFINED);
        s.insert("DEFINER".to_string(), TokenKind::DEFINER);
        s.insert("DELAY_KEY_WRITE".to_string(), TokenKind::DELAY_KEY_WRITE);
        s.insert("DELAYED".to_string(), TokenKind::DELAYED);
        s.insert("DELETE".to_string(), TokenKind::DELETE);
        s.insert("DEPENDENCY".to_string(), TokenKind::DEPENDENCY);
        s.insert("DEPTH".to_string(), TokenKind::DEPTH);
        s.insert("DESC".to_string(), TokenKind::DESC);
        s.insert("DESCRIBE".to_string(), TokenKind::DESCRIBE);
        s.insert("DIGEST".to_string(), TokenKind::DIGEST);
        s.insert("DIRECTORY".to_string(), TokenKind::DIRECTORY);
        s.insert("DISABLE".to_string(), TokenKind::DISABLE);
        s.insert("DISABLED".to_string(), TokenKind::DISABLED);
        s.insert("DISCARD".to_string(), TokenKind::DISCARD);
        s.insert("DISK".to_string(), TokenKind::DISK);
        s.insert("DISTINCT".to_string(), TokenKind::DISTINCT);
        s.insert("DISTINCTROW".to_string(), TokenKind::DISTINCTROW);
        s.insert("DIV".to_string(), TokenKind::DIV);
        s.insert("DO".to_string(), TokenKind::DO);
        s.insert("DOT".to_string(), TokenKind::DOT);
        s.insert("DOUBLE".to_string(), TokenKind::DOUBLE);
        s.insert("DROP".to_string(), TokenKind::DROP);
        s.insert("DRY".to_string(), TokenKind::DRY);
        s.insert("DRYRUN".to_string(), TokenKind::DRYRUN);
        s.insert("DUAL".to_string(), TokenKind::DUAL);
        s.insert("DUMP".to_string(), TokenKind::DUMP);
        s.insert("DUPLICATE".to_string(), TokenKind::DUPLICATE);
        s.insert("DURATION".to_string(), TokenKind::DURATION);
        s.insert("DYNAMIC".to_string(), TokenKind::DYNAMIC);
        s.insert("ELSE".to_string(), TokenKind::ELSE);
        s.insert("ELSEIF".to_string(), TokenKind::ELSEIF);
        s.insert("ENABLE".to_string(), TokenKind::ENABLE);
        s.insert("ENABLED".to_string(), TokenKind::ENABLED);
        s.insert("ENCLOSED".to_string(), TokenKind::ENCLOSED);
        s.insert("ENCRYPTION".to_string(), TokenKind::ENCRYPTION);
        s.insert("END".to_string(), TokenKind::END);
        s.insert("END_TIME".to_string(), TokenKind::END_TIME);
        s.insert("ENFORCED".to_string(), TokenKind::ENFORCED);
        s.insert("ENGINE".to_string(), TokenKind::ENGINE);
        s.insert("ENGINES".to_string(), TokenKind::ENGINES);
        s.insert("ENUM".to_string(), TokenKind::ENUM);
        s.insert("ERROR".to_string(), TokenKind::ERROR);
        s.insert("ERRORS".to_string(), TokenKind::ERRORS);
        s.insert("ESCAPE".to_string(), TokenKind::ESCAPE);
        s.insert("ESCAPED".to_string(), TokenKind::ESCAPED);
        s.insert("EVENT".to_string(), TokenKind::EVENT);
        s.insert("EVENTS".to_string(), TokenKind::EVENTS);
        s.insert("EVOLVE".to_string(), TokenKind::EVOLVE);
        s.insert("EXACT".to_string(), TokenKind::EXACT);
        s.insert("EXEC_ELAPSED".to_string(), TokenKind::EXEC_ELAPSED);
        s.insert("EXCEPT".to_string(), TokenKind::EXCEPT);
        s.insert("EXCHANGE".to_string(), TokenKind::EXCHANGE);
        s.insert("EXCLUSIVE".to_string(), TokenKind::EXCLUSIVE);
        s.insert("EXECUTE".to_string(), TokenKind::EXECUTE);
        s.insert("EXISTS".to_string(), TokenKind::EXISTS);
        s.insert("EXIT".to_string(), TokenKind::EXIT);
        s.insert("EXPANSION".to_string(), TokenKind::EXPANSION);
        s.insert("EXPIRE".to_string(), TokenKind::EXPIRE);
        s.insert("EXPLAIN".to_string(), TokenKind::EXPLAIN);
        s.insert(
            "EXPR_PUSHDOWN_BLACKLIST".to_string(),
            TokenKind::EXPR_PUSHDOWN_BLACKLIST,
        );
        s.insert("EXTENDED".to_string(), TokenKind::EXTENDED);
        s.insert("EXTRACT".to_string(), TokenKind::EXTRACT);
        s.insert("FALSE".to_string(), TokenKind::FALSE);
        s.insert("FAULTS".to_string(), TokenKind::FAULTS);
        s.insert("FETCH".to_string(), TokenKind::FETCH);
        s.insert("FIELDS".to_string(), TokenKind::FIELDS);
        s.insert("FILE".to_string(), TokenKind::FILE);
        s.insert("FIRST".to_string(), TokenKind::FIRST);
        s.insert("FIXED".to_string(), TokenKind::FIXED);
        s.insert("FLASHBACK".to_string(), TokenKind::FLASHBACK);
        s.insert("FLOAT".to_string(), TokenKind::FLOAT);
        s.insert("FLOAT4".to_string(), TokenKind::FLOAT4);
        s.insert("FLOAT8".to_string(), TokenKind::FLOAT8);
        s.insert("FLUSH".to_string(), TokenKind::FLUSH);
        s.insert("FOLLOWER".to_string(), TokenKind::FOLLOWER);
        s.insert("FOLLOWERS".to_string(), TokenKind::FOLLOWERS);
        s.insert(
            "FOLLOWER_CONSTRAINTS".to_string(),
            TokenKind::FOLLOWER_CONSTRAINTS,
        );
        s.insert("FOLLOWING".to_string(), TokenKind::FOLLOWING);
        s.insert("FOR".to_string(), TokenKind::FOR);
        s.insert("FORCE".to_string(), TokenKind::FORCE);
        s.insert("FOREIGN".to_string(), TokenKind::FOREIGN);
        s.insert("FORMAT".to_string(), TokenKind::FORMAT);
        s.insert("FOUND".to_string(), TokenKind::FOUND);
        s.insert("FROM".to_string(), TokenKind::FROM);
        s.insert("FULL".to_string(), TokenKind::FULL);
        s.insert(
            "FULL_BACKUP_STORAGE".to_string(),
            TokenKind::FULL_BACKUP_STORAGE,
        );
        s.insert("FULLTEXT".to_string(), TokenKind::FULLTEXT);
        s.insert("FUNCTION".to_string(), TokenKind::FUNCTION);
        s.insert("GC_TTL".to_string(), TokenKind::GC_TTL);
        s.insert("GENERAL".to_string(), TokenKind::GENERAL);
        s.insert("GENERATED".to_string(), TokenKind::GENERATED);
        s.insert("GET_FORMAT".to_string(), TokenKind::GET_FORMAT);
        s.insert("GLOBAL".to_string(), TokenKind::GLOBAL);
        s.insert("GRANT".to_string(), TokenKind::GRANT);
        s.insert("GRANTS".to_string(), TokenKind::GRANTS);
        s.insert("GROUP_CONCAT".to_string(), TokenKind::GROUP_CONCAT);
        s.insert("GROUP".to_string(), TokenKind::GROUP);
        s.insert("HASH".to_string(), TokenKind::HASH);
        s.insert("HANDLER".to_string(), TokenKind::HANDLER);
        s.insert("HAVING".to_string(), TokenKind::HAVING);
        s.insert("HELP".to_string(), TokenKind::HELP);
        s.insert("HIGH_PRIORITY".to_string(), TokenKind::HIGH_PRIORITY);
        s.insert("HISTORY".to_string(), TokenKind::HISTORY);
        s.insert("HISTOGRAM".to_string(), TokenKind::HISTOGRAM);
        s.insert("HNSW".to_string(), TokenKind::HNSW);
        s.insert("HOSTS".to_string(), TokenKind::HOSTS);
        s.insert("HOUR_MICROSECOND".to_string(), TokenKind::HOUR_MICROSECOND);
        s.insert("HOUR_MINUTE".to_string(), TokenKind::HOUR_MINUTE);
        s.insert("HOUR_SECOND".to_string(), TokenKind::HOUR_SECOND);
        s.insert("HOUR".to_string(), TokenKind::HOUR);
        s.insert("IDENTIFIED".to_string(), TokenKind::IDENTIFIED);
        s.insert("IF".to_string(), TokenKind::IF);
        s.insert("IGNORE".to_string(), TokenKind::IGNORE);
        s.insert("ILIKE".to_string(), TokenKind::ILIKE);
        s.insert("IMPORT".to_string(), TokenKind::IMPORT);
        s.insert("IMPORTS".to_string(), TokenKind::IMPORTS);
        s.insert("IN".to_string(), TokenKind::IN);
        s.insert("INCREMENT".to_string(), TokenKind::INCREMENT);
        s.insert("INCREMENTAL".to_string(), TokenKind::INCREMENTAL);
        s.insert("INDEX".to_string(), TokenKind::INDEX);
        s.insert("INDEXES".to_string(), TokenKind::INDEXES);
        s.insert("INFILE".to_string(), TokenKind::INFILE);
        s.insert("INNER".to_string(), TokenKind::INNER);
        s.insert("INOUT".to_string(), TokenKind::INOUT);
        s.insert("INPLACE".to_string(), TokenKind::INPLACE);
        s.insert("INSERT_METHOD".to_string(), TokenKind::INSERT_METHOD);
        s.insert("INSERT".to_string(), TokenKind::INSERT);
        s.insert("INSTANCE".to_string(), TokenKind::INSTANCE);
        s.insert("INSTANT".to_string(), TokenKind::INSTANT);
        s.insert("INT".to_string(), TokenKind::INT);
        s.insert("INT1".to_string(), TokenKind::INT1);
        s.insert("INT2".to_string(), TokenKind::INT2);
        s.insert("INT3".to_string(), TokenKind::INT3);
        s.insert("INT4".to_string(), TokenKind::INT4);
        s.insert("INT8".to_string(), TokenKind::INT8);
        s.insert("INTEGER".to_string(), TokenKind::INTEGER);
        s.insert("INTERNAL".to_string(), TokenKind::INTERNAL);
        s.insert("INTERSECT".to_string(), TokenKind::INTERSECT);
        s.insert("INTERVAL".to_string(), TokenKind::INTERVAL);
        s.insert("INTO".to_string(), TokenKind::INTO);
        s.insert("INVISIBLE".to_string(), TokenKind::INVISIBLE);
        s.insert("INVOKER".to_string(), TokenKind::INVOKER);
        s.insert("ITERATE".to_string(), TokenKind::ITERATE);
        s.insert("IO".to_string(), TokenKind::IO);
        s.insert("RU_PER_SEC".to_string(), TokenKind::RU_PER_SEC);
        s.insert("PRIORITY".to_string(), TokenKind::PRIORITY);
        s.insert("HIGH".to_string(), TokenKind::HIGH);
        s.insert("MEDIUM".to_string(), TokenKind::MEDIUM);
        s.insert("LOW".to_string(), TokenKind::LOW);
        s.insert(
            "IO_READ_BANDWIDTH".to_string(),
            TokenKind::IO_READ_BANDWIDTH,
        );
        s.insert(
            "IO_WRITE_BANDWIDTH".to_string(),
            TokenKind::IO_WRITE_BANDWIDTH,
        );
        s.insert("IPC".to_string(), TokenKind::IPC);
        s.insert("IS".to_string(), TokenKind::IS);
        s.insert("ISOLATION".to_string(), TokenKind::ISOLATION);
        s.insert("ISSUER".to_string(), TokenKind::ISSUER);
        s.insert("JOB".to_string(), TokenKind::JOB);
        s.insert("JOBS".to_string(), TokenKind::JOBS);
        s.insert("JOIN".to_string(), TokenKind::JOIN);
        s.insert("JSON_ARRAYAGG".to_string(), TokenKind::JSON_ARRAYAGG);
        s.insert("JSON_OBJECTAGG".to_string(), TokenKind::JSON_OBJECTAGG);
        s.insert("JSON".to_string(), TokenKind::JSON);
        s.insert("KEY_BLOCK_SIZE".to_string(), TokenKind::KEY_BLOCK_SIZE);
        s.insert("KEY".to_string(), TokenKind::KEY);
        s.insert("KEYS".to_string(), TokenKind::KEYS);
        s.insert("KILL".to_string(), TokenKind::KILL);
        s.insert("LABELS".to_string(), TokenKind::LABELS);
        s.insert("LANGUAGE".to_string(), TokenKind::LANGUAGE);
        s.insert("LAST_BACKUP".to_string(), TokenKind::LAST_BACKUP);
        s.insert("LAST".to_string(), TokenKind::LAST);
        s.insert("LASTVAL".to_string(), TokenKind::LASTVAL);
        s.insert("LEADER".to_string(), TokenKind::LEADER);
        s.insert(
            "LEADER_CONSTRAINTS".to_string(),
            TokenKind::LEADER_CONSTRAINTS,
        );
        s.insert("LEADING".to_string(), TokenKind::LEADING);
        s.insert("LEARNER".to_string(), TokenKind::LEARNER);
        s.insert(
            "LEARNER_CONSTRAINTS".to_string(),
            TokenKind::LEARNER_CONSTRAINTS,
        );
        s.insert("LEARNERS".to_string(), TokenKind::LEARNERS);
        s.insert("LEAVE".to_string(), TokenKind::LEAVE);
        s.insert("LEFT".to_string(), TokenKind::LEFT);
        s.insert("LESS".to_string(), TokenKind::LESS);
        s.insert("LEVEL".to_string(), TokenKind::LEVEL);
        s.insert("LIKE".to_string(), TokenKind::LIKE);
        s.insert("LIMIT".to_string(), TokenKind::LIMIT);
        s.insert("LINEAR".to_string(), TokenKind::LINEAR);
        s.insert("LINES".to_string(), TokenKind::LINES);
        s.insert("LIST".to_string(), TokenKind::LIST);
        s.insert("LOAD".to_string(), TokenKind::LOAD);
        s.insert("LOCAL".to_string(), TokenKind::LOCAL);
        s.insert("LOCALTIME".to_string(), TokenKind::LOCALTIME);
        s.insert("LOCALTIMESTAMP".to_string(), TokenKind::LOCALTIMESTAMP);
        s.insert("LOCATION".to_string(), TokenKind::LOCATION);
        s.insert("LOCK".to_string(), TokenKind::LOCK);
        s.insert("LOCKED".to_string(), TokenKind::LOCKED);
        s.insert("LOG".to_string(), TokenKind::LOG);
        s.insert("LOGS".to_string(), TokenKind::LOGS);
        s.insert("LONG".to_string(), TokenKind::LONG);
        s.insert("LONGBLOB".to_string(), TokenKind::LONGBLOB);
        s.insert("LONGTEXT".to_string(), TokenKind::LONGTEXT);
        s.insert("LOW_PRIORITY".to_string(), TokenKind::LOW_PRIORITY);
        s.insert("MASTER".to_string(), TokenKind::MASTER);
        s.insert("MATCH".to_string(), TokenKind::MATCH);
        s.insert(
            "MAX_CONNECTIONS_PER_HOUR".to_string(),
            TokenKind::MAX_CONNECTIONS_PER_HOUR,
        );
        s.insert("MAX_IDXNUM".to_string(), TokenKind::MAX_IDXNUM);
        s.insert("MAX_MINUTES".to_string(), TokenKind::MAX_MINUTES);
        s.insert(
            "MAX_QUERIES_PER_HOUR".to_string(),
            TokenKind::MAX_QUERIES_PER_HOUR,
        );
        s.insert("MAX_ROWS".to_string(), TokenKind::MAX_ROWS);
        s.insert(
            "MAX_UPDATES_PER_HOUR".to_string(),
            TokenKind::MAX_UPDATES_PER_HOUR,
        );
        s.insert(
            "MAX_USER_CONNECTIONS".to_string(),
            TokenKind::MAX_USER_CONNECTIONS,
        );
        s.insert("MAX".to_string(), TokenKind::MAX);
        s.insert("MAXVALUE".to_string(), TokenKind::MAXVALUE);
        s.insert("MB".to_string(), TokenKind::MB);
        s.insert("MEDIUMBLOB".to_string(), TokenKind::MEDIUMBLOB);
        s.insert("MEDIUMINT".to_string(), TokenKind::MEDIUMINT);
        s.insert("MEDIUMTEXT".to_string(), TokenKind::MEDIUMTEXT);
        s.insert("MEMORY".to_string(), TokenKind::MEMORY);
        s.insert("MEMBER".to_string(), TokenKind::MEMBER);
        s.insert("MERGE".to_string(), TokenKind::MERGE);
        s.insert("METADATA".to_string(), TokenKind::METADATA);
        s.insert("MICROSECOND".to_string(), TokenKind::MICROSECOND);
        s.insert("MIDDLEINT".to_string(), TokenKind::MIDDLEINT);
        s.insert("MIN_ROWS".to_string(), TokenKind::MIN_ROWS);
        s.insert("MIN".to_string(), TokenKind::MIN);
        s.insert(
            "MINUTE_MICROSECOND".to_string(),
            TokenKind::MINUTE_MICROSECOND,
        );
        s.insert("MINUTE_SECOND".to_string(), TokenKind::MINUTE_SECOND);
        s.insert("MINUTE".to_string(), TokenKind::MINUTE);
        s.insert("MINVALUE".to_string(), TokenKind::MINVALUE);
        s.insert("MOD".to_string(), TokenKind::MOD);
        s.insert("MODE".to_string(), TokenKind::MODE);
        s.insert("MODIFY".to_string(), TokenKind::MODIFY);
        s.insert("MONTH".to_string(), TokenKind::MONTH);
        s.insert("NAMES".to_string(), TokenKind::NAMES);
        s.insert("NATIONAL".to_string(), TokenKind::NATIONAL);
        s.insert("NATURAL".to_string(), TokenKind::NATURAL);
        s.insert("NCHAR".to_string(), TokenKind::NCHAR);
        s.insert("NEVER".to_string(), TokenKind::NEVER);
        s.insert("NEXT_ROW_ID".to_string(), TokenKind::NEXT_ROW_ID);
        s.insert("NEXT".to_string(), TokenKind::NEXT);
        s.insert("NEXTVAL".to_string(), TokenKind::NEXTVAL);
        s.insert(
            "NO_WRITE_TO_BINLOG".to_string(),
            TokenKind::NO_WRITE_TO_BINLOG,
        );
        s.insert("NO".to_string(), TokenKind::NO);
        s.insert("NOCACHE".to_string(), TokenKind::NOCACHE);
        s.insert("NOCYCLE".to_string(), TokenKind::NOCYCLE);
        s.insert("NODE_ID".to_string(), TokenKind::NODE_ID);
        s.insert("NODE_STATE".to_string(), TokenKind::NODE_STATE);
        s.insert("NODEGROUP".to_string(), TokenKind::NODEGROUP);
        s.insert("NOMAXVALUE".to_string(), TokenKind::NOMAXVALUE);
        s.insert("NOMINVALUE".to_string(), TokenKind::NOMINVALUE);
        s.insert("NONCLUSTERED".to_string(), TokenKind::NONCLUSTERED);
        s.insert("NONE".to_string(), TokenKind::NONE);
        s.insert("NOT".to_string(), TokenKind::NOT);
        s.insert("NOW".to_string(), TokenKind::NOW);
        s.insert("NOWAIT".to_string(), TokenKind::NOWAIT);
        s.insert("NULL".to_string(), TokenKind::NULL);
        s.insert("NULLS".to_string(), TokenKind::NULLS);
        s.insert("NUMERIC".to_string(), TokenKind::NUMERIC);
        s.insert("NVARCHAR".to_string(), TokenKind::NVARCHAR);
        s.insert("OF".to_string(), TokenKind::OF);
        s.insert("OFF".to_string(), TokenKind::OFF);
        s.insert("OFFSET".to_string(), TokenKind::OFFSET);
        s.insert("OLTP_READ_ONLY".to_string(), TokenKind::OLTP_READ_ONLY);
        s.insert("OLTP_READ_WRITE".to_string(), TokenKind::OLTP_READ_WRITE);
        s.insert("OLTP_WRITE_ONLY".to_string(), TokenKind::OLTP_WRITE_ONLY);
        s.insert("TPCH_10".to_string(), TokenKind::TPCH_10);
        s.insert("ON_DUPLICATE".to_string(), TokenKind::ON_DUPLICATE);
        s.insert("ON".to_string(), TokenKind::ON);
        s.insert("ONLINE".to_string(), TokenKind::ONLINE);
        s.insert("ONLY".to_string(), TokenKind::ONLY);
        s.insert("OPEN".to_string(), TokenKind::OPEN);
        s.insert(
            "OPT_RULE_BLACKLIST".to_string(),
            TokenKind::OPT_RULE_BLACKLIST,
        );
        s.insert("OPTIMISTIC".to_string(), TokenKind::OPTIMISTIC);
        s.insert("OPTIMIZE".to_string(), TokenKind::OPTIMIZE);
        s.insert("OPTION".to_string(), TokenKind::OPTION);
        s.insert("OPTIONAL".to_string(), TokenKind::OPTIONAL);
        s.insert("OPTIONALLY".to_string(), TokenKind::OPTIONALLY);
        s.insert("OR".to_string(), TokenKind::OR);
        s.insert("ORDER".to_string(), TokenKind::ORDER);
        s.insert("OUT".to_string(), TokenKind::OUT);
        s.insert("OUTER".to_string(), TokenKind::OUTER);
        s.insert("OUTFILE".to_string(), TokenKind::OUTFILE);
        s.insert("PACK_KEYS".to_string(), TokenKind::PACK_KEYS);
        s.insert("PAGE".to_string(), TokenKind::PAGE);
        s.insert("PARSER".to_string(), TokenKind::PARSER);
        s.insert("PARTIAL".to_string(), TokenKind::PARTIAL);
        s.insert("PARTITION".to_string(), TokenKind::PARTITION);
        s.insert("PARTITIONING".to_string(), TokenKind::PARTITIONING);
        s.insert("PARTITIONS".to_string(), TokenKind::PARTITIONS);
        s.insert("PASSWORD".to_string(), TokenKind::PASSWORD);
        s.insert("PAUSE".to_string(), TokenKind::PAUSE);
        s.insert("PERCENT".to_string(), TokenKind::PERCENT);
        s.insert("PER_DB".to_string(), TokenKind::PER_DB);
        s.insert("PER_TABLE".to_string(), TokenKind::PER_TABLE);
        s.insert("PESSIMISTIC".to_string(), TokenKind::PESSIMISTIC);
        s.insert("PLACEMENT".to_string(), TokenKind::PLACEMENT);
        s.insert("PLAN".to_string(), TokenKind::PLAN);
        s.insert("PLAN_CACHE".to_string(), TokenKind::PLAN_CACHE);
        s.insert("PLUGINS".to_string(), TokenKind::PLUGINS);
        s.insert("POINT".to_string(), TokenKind::POINT);
        s.insert("POLICY".to_string(), TokenKind::POLICY);
        s.insert("POSITION".to_string(), TokenKind::POSITION);
        s.insert(
            "PRE_SPLIT_REGIONS".to_string(),
            TokenKind::PRE_SPLIT_REGIONS,
        );
        s.insert("PRECEDING".to_string(), TokenKind::PRECEDING);
        s.insert("PREDICATE".to_string(), TokenKind::PREDICATE);
        s.insert("PRECISION".to_string(), TokenKind::PRECISION);
        s.insert("PREPARE".to_string(), TokenKind::PREPARE);
        s.insert("PRESERVE".to_string(), TokenKind::PRESERVE);
        s.insert("PRIMARY".to_string(), TokenKind::PRIMARY);
        s.insert("PRIMARY_REGION".to_string(), TokenKind::PRIMARY_REGION);
        s.insert("PRIVILEGES".to_string(), TokenKind::PRIVILEGES);
        s.insert("PROCEDURE".to_string(), TokenKind::PROCEDURE);
        s.insert("PROCESS".to_string(), TokenKind::PROCESS);
        s.insert("PROCESSED_KEYS".to_string(), TokenKind::PROCESSED_KEYS);
        s.insert("PROCESSLIST".to_string(), TokenKind::PROCESSLIST);
        s.insert("PROFILE".to_string(), TokenKind::PROFILE);
        s.insert("PROFILES".to_string(), TokenKind::PROFILES);
        s.insert("PROXY".to_string(), TokenKind::PROXY);
        s.insert("PURGE".to_string(), TokenKind::PURGE);
        s.insert("QUARTER".to_string(), TokenKind::QUARTER);
        s.insert("QUERIES".to_string(), TokenKind::QUERIES);
        s.insert("QUERY".to_string(), TokenKind::QUERY);
        s.insert("QUERY_LIMIT".to_string(), TokenKind::QUERY_LIMIT);
        s.insert("QUICK".to_string(), TokenKind::QUICK);
        s.insert("RANGE".to_string(), TokenKind::RANGE);
        s.insert("RATE_LIMIT".to_string(), TokenKind::RATE_LIMIT);
        s.insert("READ".to_string(), TokenKind::READ);
        s.insert("REAL".to_string(), TokenKind::REAL);
        s.insert("REBUILD".to_string(), TokenKind::REBUILD);
        s.insert("RECENT".to_string(), TokenKind::RECENT);
        s.insert("RECOMMEND".to_string(), TokenKind::RECOMMEND);
        s.insert("RECOVER".to_string(), TokenKind::RECOVER);
        s.insert("RECURSIVE".to_string(), TokenKind::RECURSIVE);
        s.insert("REDUNDANT".to_string(), TokenKind::REDUNDANT);
        s.insert("REFERENCES".to_string(), TokenKind::REFERENCES);
        s.insert("REGEXP".to_string(), TokenKind::REGEXP);
        s.insert("REGION".to_string(), TokenKind::REGION);
        s.insert("REGIONS".to_string(), TokenKind::REGIONS);
        s.insert("RELEASE".to_string(), TokenKind::RELEASE);
        s.insert("RELOAD".to_string(), TokenKind::RELOAD);
        s.insert("REMOVE".to_string(), TokenKind::REMOVE);
        s.insert("RENAME".to_string(), TokenKind::RENAME);
        s.insert("REORGANIZE".to_string(), TokenKind::REORGANIZE);
        s.insert("REPAIR".to_string(), TokenKind::REPAIR);
        s.insert("REPEAT".to_string(), TokenKind::REPEAT);
        s.insert("REPEATABLE".to_string(), TokenKind::REPEATABLE);
        s.insert("REPLACE".to_string(), TokenKind::REPLACE);
        s.insert("REPLAYER".to_string(), TokenKind::REPLAYER);
        s.insert("REPLICA".to_string(), TokenKind::REPLICA);
        s.insert("REPLICAS".to_string(), TokenKind::REPLICAS);
        s.insert("REPLICATION".to_string(), TokenKind::REPLICATION);
        s.insert("RU".to_string(), TokenKind::RU);
        s.insert("REQUIRE".to_string(), TokenKind::REQUIRE);
        s.insert("REQUIRED".to_string(), TokenKind::REQUIRED);
        s.insert("RESET".to_string(), TokenKind::RESET);
        s.insert("RESOURCE".to_string(), TokenKind::RESOURCE);
        s.insert("RESPECT".to_string(), TokenKind::RESPECT);
        s.insert("RESTART".to_string(), TokenKind::RESTART);
        s.insert("RESTORE".to_string(), TokenKind::RESTORE);
        s.insert("RESTORES".to_string(), TokenKind::RESTORES);
        s.insert("RESTORED_TS".to_string(), TokenKind::RESTORED_TS);
        s.insert("RESTRICT".to_string(), TokenKind::RESTRICT);
        s.insert("REVERSE".to_string(), TokenKind::REVERSE);
        s.insert("REVOKE".to_string(), TokenKind::REVOKE);
        s.insert("RIGHT".to_string(), TokenKind::RIGHT);
        s.insert("RLIKE".to_string(), TokenKind::RLIKE);
        s.insert("ROLE".to_string(), TokenKind::ROLE);
        s.insert("ROLLBACK".to_string(), TokenKind::ROLLBACK);
        s.insert("ROLLUP".to_string(), TokenKind::ROLLUP);
        s.insert("ROUTINE".to_string(), TokenKind::ROUTINE);
        s.insert("ROW_COUNT".to_string(), TokenKind::ROW_COUNT);
        s.insert("ROW_FORMAT".to_string(), TokenKind::ROW_FORMAT);
        s.insert("ROW".to_string(), TokenKind::ROW);
        s.insert("ROWS".to_string(), TokenKind::ROWS);
        s.insert("RTREE".to_string(), TokenKind::RTREE);
        s.insert("HYPO".to_string(), TokenKind::HYPO);
        s.insert("RESUME".to_string(), TokenKind::RESUME);
        s.insert("RUN".to_string(), TokenKind::RUN);
        s.insert("RUNNING".to_string(), TokenKind::RUNNING);
        s.insert("S3".to_string(), TokenKind::S3);
        s.insert("SAMPLES".to_string(), TokenKind::SAMPLES);
        s.insert("SAMPLERATE".to_string(), TokenKind::SAMPLERATE);
        s.insert("SAN".to_string(), TokenKind::SAN);
        s.insert("SAVEPOINT".to_string(), TokenKind::SAVEPOINT);
        s.insert("SCHEDULE".to_string(), TokenKind::SCHEDULE);
        s.insert("SCHEMA".to_string(), TokenKind::SCHEMA);
        s.insert("SCHEMAS".to_string(), TokenKind::SCHEMAS);
        s.insert(
            "SECOND_MICROSECOND".to_string(),
            TokenKind::SECOND_MICROSECOND,
        );
        s.insert("SECOND".to_string(), TokenKind::SECOND);
        s.insert("SECONDARY".to_string(), TokenKind::SECONDARY);
        s.insert("SECONDARY_ENGINE".to_string(), TokenKind::SECONDARY_ENGINE);
        s.insert("SECONDARY_LOAD".to_string(), TokenKind::SECONDARY_LOAD);
        s.insert("SECONDARY_UNLOAD".to_string(), TokenKind::SECONDARY_UNLOAD);
        s.insert("SECURITY".to_string(), TokenKind::SECURITY);
        s.insert("SELECT".to_string(), TokenKind::SELECT);
        s.insert(
            "SEND_CREDENTIALS_TO_TIKV".to_string(),
            TokenKind::SEND_CREDENTIALS_TO_TIKV,
        );
        s.insert("SEPARATOR".to_string(), TokenKind::SEPARATOR);
        s.insert("SEQUENCE".to_string(), TokenKind::SEQUENCE);
        s.insert("SERIAL".to_string(), TokenKind::SERIAL);
        s.insert("SERIALIZABLE".to_string(), TokenKind::SERIALIZABLE);
        s.insert("SESSION".to_string(), TokenKind::SESSION);
        s.insert("SESSION_STATES".to_string(), TokenKind::SESSION_STATES);
        s.insert("SET".to_string(), TokenKind::SET);
        s.insert("SETVAL".to_string(), TokenKind::SETVAL);
        s.insert(
            "SHARD_ROW_ID_BITS".to_string(),
            TokenKind::SHARD_ROW_ID_BITS,
        );
        s.insert("SHARE".to_string(), TokenKind::SHARE);
        s.insert("SHARED".to_string(), TokenKind::SHARED);
        s.insert("SHOW".to_string(), TokenKind::SHOW);
        s.insert("SHUTDOWN".to_string(), TokenKind::SHUTDOWN);
        s.insert("SIGNED".to_string(), TokenKind::SIGNED);
        s.insert("SIMILAR".to_string(), TokenKind::SIMILAR);
        s.insert("SIMPLE".to_string(), TokenKind::SIMPLE);
        s.insert("SKIP".to_string(), TokenKind::SKIP);
        s.insert(
            "SKIP_SCHEMA_FILES".to_string(),
            TokenKind::SKIP_SCHEMA_FILES,
        );
        s.insert("SLAVE".to_string(), TokenKind::SLAVE);
        s.insert("SLOW".to_string(), TokenKind::SLOW);
        s.insert("SMALLINT".to_string(), TokenKind::SMALLINT);
        s.insert("SNAPSHOT".to_string(), TokenKind::SNAPSHOT);
        s.insert("SOME".to_string(), TokenKind::SOME);
        s.insert("SOURCE".to_string(), TokenKind::SOURCE);
        s.insert("SPATIAL".to_string(), TokenKind::SPATIAL);
        s.insert("SPLIT".to_string(), TokenKind::SPLIT);
        s.insert("SQL_BIG_RESULT".to_string(), TokenKind::SQL_BIG_RESULT);
        s.insert(
            "SQL_BUFFER_RESULT".to_string(),
            TokenKind::SQL_BUFFER_RESULT,
        );
        s.insert("SQL_CACHE".to_string(), TokenKind::SQL_CACHE);
        s.insert(
            "SQL_CALC_FOUND_ROWS".to_string(),
            TokenKind::SQL_CALC_FOUND_ROWS,
        );
        s.insert("SQL_NO_CACHE".to_string(), TokenKind::SQL_NO_CACHE);
        s.insert("SQL_SMALL_RESULT".to_string(), TokenKind::SQL_SMALL_RESULT);
        s.insert("SQL_TSI_DAY".to_string(), TokenKind::SQL_TSI_DAY);
        s.insert("SQL_TSI_HOUR".to_string(), TokenKind::SQL_TSI_HOUR);
        s.insert("SQL_TSI_MINUTE".to_string(), TokenKind::SQL_TSI_MINUTE);
        s.insert("SQL_TSI_MONTH".to_string(), TokenKind::SQL_TSI_MONTH);
        s.insert("SQL_TSI_QUARTER".to_string(), TokenKind::SQL_TSI_QUARTER);
        s.insert("SQL_TSI_SECOND".to_string(), TokenKind::SQL_TSI_SECOND);
        s.insert("SQL_TSI_WEEK".to_string(), TokenKind::SQL_TSI_WEEK);
        s.insert("SQL_TSI_YEAR".to_string(), TokenKind::SQL_TSI_YEAR);
        s.insert("SQL".to_string(), TokenKind::SQL);
        s.insert("SQLEXCEPTION".to_string(), TokenKind::SQLEXCEPTION);
        s.insert("SQLSTATE".to_string(), TokenKind::SQLSTATE);
        s.insert("SQLWARNING".to_string(), TokenKind::SQLWARNING);
        s.insert("SSL".to_string(), TokenKind::SSL);
        s.insert("STALENESS".to_string(), TokenKind::STALENESS);
        s.insert("START".to_string(), TokenKind::START);
        s.insert("START_TIME".to_string(), TokenKind::START_TIME);
        s.insert("START_TS".to_string(), TokenKind::START_TS);
        s.insert("STARTING".to_string(), TokenKind::STARTING);
        s.insert("STATISTICS".to_string(), TokenKind::STATISTICS);
        s.insert(
            "STATS_AUTO_RECALC".to_string(),
            TokenKind::STATS_AUTO_RECALC,
        );
        s.insert("STATS_BUCKETS".to_string(), TokenKind::STATS_BUCKETS);
        s.insert("STATS_EXTENDED".to_string(), TokenKind::STATS_EXTENDED);
        s.insert("STATS_HEALTHY".to_string(), TokenKind::STATS_HEALTHY);
        s.insert("STATS_HISTOGRAMS".to_string(), TokenKind::STATS_HISTOGRAMS);
        s.insert("STATS_TOPN".to_string(), TokenKind::STATS_TOPN);
        s.insert("STATS_META".to_string(), TokenKind::STATS_META);
        s.insert("STATS_LOCKED".to_string(), TokenKind::STATS_LOCKED);
        s.insert(
            "HISTOGRAMS_IN_FLIGHT".to_string(),
            TokenKind::HISTOGRAMS_IN_FLIGHT,
        );
        s.insert("STATS_PERSISTENT".to_string(), TokenKind::STATS_PERSISTENT);
        s.insert(
            "STATS_SAMPLE_PAGES".to_string(),
            TokenKind::STATS_SAMPLE_PAGES,
        );
        s.insert("STATS".to_string(), TokenKind::STATS);
        s.insert("STATUS".to_string(), TokenKind::STATUS);
        s.insert("STD".to_string(), TokenKind::STD);
        s.insert("STDDEV_POP".to_string(), TokenKind::STDDEV_POP);
        s.insert("STDDEV_SAMP".to_string(), TokenKind::STDDEV_SAMP);
        s.insert("STDDEV".to_string(), TokenKind::STDDEV);
        s.insert("STOP".to_string(), TokenKind::STOP);
        s.insert("STORAGE".to_string(), TokenKind::STORAGE);
        s.insert("STORED".to_string(), TokenKind::STORED);
        s.insert("STRAIGHT_JOIN".to_string(), TokenKind::STRAIGHT_JOIN);
        s.insert("STRICT".to_string(), TokenKind::STRICT);
        s.insert("STRICT_FORMAT".to_string(), TokenKind::STRICT_FORMAT);
        s.insert("STRONG".to_string(), TokenKind::STRONG);
        s.insert("SUBDATE".to_string(), TokenKind::SUBDATE);
        s.insert("SUBJECT".to_string(), TokenKind::SUBJECT);
        s.insert("SUBPARTITION".to_string(), TokenKind::SUBPARTITION);
        s.insert("SUBPARTITIONS".to_string(), TokenKind::SUBPARTITIONS);
        s.insert("SUBSTR".to_string(), TokenKind::SUBSTR);
        s.insert("SUBSTRING".to_string(), TokenKind::SUBSTRING);
        s.insert("SUM".to_string(), TokenKind::SUM);
        s.insert("SUPER".to_string(), TokenKind::SUPER);
        s.insert(
            "SURVIVAL_PREFERENCES".to_string(),
            TokenKind::SURVIVAL_PREFERENCES,
        );
        s.insert("SWAPS".to_string(), TokenKind::SWAPS);
        s.insert("SWITCHES".to_string(), TokenKind::SWITCHES);
        s.insert("SWITCH_GROUP".to_string(), TokenKind::SWITCH_GROUP);
        s.insert("SYSTEM".to_string(), TokenKind::SYSTEM);
        s.insert("SYSTEM_TIME".to_string(), TokenKind::SYSTEM_TIME);
        s.insert("TARGET".to_string(), TokenKind::TARGET);
        s.insert("TASK_TYPES".to_string(), TokenKind::TASK_TYPES);
        s.insert("TABLE_CHECKSUM".to_string(), TokenKind::TABLE_CHECKSUM);
        s.insert("TABLE".to_string(), TokenKind::TABLE);
        s.insert("TABLES".to_string(), TokenKind::TABLES);
        s.insert("TABLESAMPLE".to_string(), TokenKind::TABLESAMPLE);
        s.insert("TABLESPACE".to_string(), TokenKind::TABLESPACE);
        s.insert("TEMPORARY".to_string(), TokenKind::TEMPORARY);
        s.insert("TEMPTABLE".to_string(), TokenKind::TEMPTABLE);
        s.insert("TERMINATED".to_string(), TokenKind::TERMINATED);
        s.insert("TEXT".to_string(), TokenKind::TEXT);
        s.insert("THAN".to_string(), TokenKind::THAN);
        s.insert("THEN".to_string(), TokenKind::THEN);
        s.insert("TIDB".to_string(), TokenKind::TIDB);
        s.insert("TIDB_CURRENT_TSO".to_string(), TokenKind::TIDB_CURRENT_TSO);
        s.insert("TIDB_JSON".to_string(), TokenKind::TIDB_JSON);
        s.insert("TIFLASH".to_string(), TokenKind::TIFLASH);
        s.insert("TIKV_IMPORTER".to_string(), TokenKind::TIKV_IMPORTER);
        s.insert("TIME".to_string(), TokenKind::TIME);
        s.insert("TIMESTAMP".to_string(), TokenKind::TIMESTAMP);
        s.insert("TIMESTAMPADD".to_string(), TokenKind::TIMESTAMPADD);
        s.insert("TIMESTAMPDIFF".to_string(), TokenKind::TIMESTAMPDIFF);
        s.insert("TINYBLOB".to_string(), TokenKind::TINYBLOB);
        s.insert("TINYINT".to_string(), TokenKind::TINYINT);
        s.insert("TINYTEXT".to_string(), TokenKind::TINYTEXT);
        s.insert("TLS".to_string(), TokenKind::TLS);
        s.insert("TO".to_string(), TokenKind::TO);
        s.insert("TOKEN_ISSUER".to_string(), TokenKind::TOKEN_ISSUER);
        s.insert("TOKUDB_DEFAULT".to_string(), TokenKind::TOKUDB_DEFAULT);
        s.insert("TOKUDB_FAST".to_string(), TokenKind::TOKUDB_FAST);
        s.insert("TOKUDB_LZMA".to_string(), TokenKind::TOKUDB_LZMA);
        s.insert("TOKUDB_QUICKLZ".to_string(), TokenKind::TOKUDB_QUICKLZ);
        s.insert("TOKUDB_SMALL".to_string(), TokenKind::TOKUDB_SMALL);
        s.insert("TOKUDB_SNAPPY".to_string(), TokenKind::TOKUDB_SNAPPY);
        s.insert(
            "TOKUDB_UNCOMPRESSED".to_string(),
            TokenKind::TOKUDB_UNCOMPRESSED,
        );
        s.insert("TOKUDB_ZLIB".to_string(), TokenKind::TOKUDB_ZLIB);
        s.insert("TOKUDB_ZSTD".to_string(), TokenKind::TOKUDB_ZSTD);
        s.insert("TOP".to_string(), TokenKind::TOP);
        s.insert("TOPN".to_string(), TokenKind::TOPN);
        s.insert("TPCC".to_string(), TokenKind::TPCC);
        s.insert("TRACE".to_string(), TokenKind::TRACE);
        s.insert("TRADITIONAL".to_string(), TokenKind::TRADITIONAL);
        s.insert("TRAILING".to_string(), TokenKind::TRAILING);
        s.insert("TRANSACTION".to_string(), TokenKind::TRANSACTION);
        s.insert("TRIGGER".to_string(), TokenKind::TRIGGER);
        s.insert("TRIGGERS".to_string(), TokenKind::TRIGGERS);
        s.insert("TRIM".to_string(), TokenKind::TRIM);
        s.insert("TRUE".to_string(), TokenKind::TRUE);
        s.insert("TRUNCATE".to_string(), TokenKind::TRUNCATE);
        s.insert("TRUE_CARD_COST".to_string(), TokenKind::TRUE_CARD_COST);
        s.insert("TSO".to_string(), TokenKind::TSO);
        s.insert("TTL".to_string(), TokenKind::TTL);
        s.insert("TTL_ENABLE".to_string(), TokenKind::TTL_ENABLE);
        s.insert("TTL_JOB_INTERVAL".to_string(), TokenKind::TTL_JOB_INTERVAL);
        s.insert("TYPE".to_string(), TokenKind::TYPE);
        s.insert("UNBOUNDED".to_string(), TokenKind::UNBOUNDED);
        s.insert("UNCOMMITTED".to_string(), TokenKind::UNCOMMITTED);
        s.insert("UNDEFINED".to_string(), TokenKind::UNDEFINED);
        s.insert("UNICODE".to_string(), TokenKind::UNICODE);
        s.insert("UNION".to_string(), TokenKind::UNION);
        s.insert("UNIQUE".to_string(), TokenKind::UNIQUE);
        s.insert("UNKNOWN".to_string(), TokenKind::UNKNOWN);
        s.insert("UNLOCK".to_string(), TokenKind::UNLOCK);
        s.insert("UNLIMITED".to_string(), TokenKind::UNLIMITED);
        s.insert("UNSET".to_string(), TokenKind::UNSET);
        s.insert("UNSIGNED".to_string(), TokenKind::UNSIGNED);
        s.insert("UNTIL".to_string(), TokenKind::UNTIL);
        s.insert("UNTIL_TS".to_string(), TokenKind::UNTIL_TS);
        s.insert("UPDATE".to_string(), TokenKind::UPDATE);
        s.insert("USAGE".to_string(), TokenKind::USAGE);
        s.insert("USE".to_string(), TokenKind::USE);
        s.insert("USER".to_string(), TokenKind::USER);
        s.insert("USING".to_string(), TokenKind::USING);
        s.insert("UTC_DATE".to_string(), TokenKind::UTC_DATE);
        s.insert("UTC_TIME".to_string(), TokenKind::UTC_TIME);
        s.insert("UTC_TIMESTAMP".to_string(), TokenKind::UTC_TIMESTAMP);
        s.insert(
            "UTILIZATION_LIMIT".to_string(),
            TokenKind::UTILIZATION_LIMIT,
        );
        s.insert("VALIDATION".to_string(), TokenKind::VALIDATION);
        s.insert("VALUE".to_string(), TokenKind::VALUE);
        s.insert("VALUES".to_string(), TokenKind::VALUES);
        s.insert("VAR_POP".to_string(), TokenKind::VAR_POP);
        s.insert("VAR_SAMP".to_string(), TokenKind::VAR_SAMP);
        s.insert("VARBINARY".to_string(), TokenKind::VARBINARY);
        s.insert("VARCHAR".to_string(), TokenKind::VARCHAR);
        s.insert("VARCHARACTER".to_string(), TokenKind::VARCHARACTER);
        s.insert("VARIABLES".to_string(), TokenKind::VARIABLES);
        s.insert("VARIANCE".to_string(), TokenKind::VARIANCE);
        s.insert("VARYING".to_string(), TokenKind::VARYING);
        s.insert("VECTOR".to_string(), TokenKind::VECTOR);
        s.insert("VERBOSE".to_string(), TokenKind::VERBOSE);
        s.insert("VOTER".to_string(), TokenKind::VOTER);
        s.insert(
            "VOTER_CONSTRAINTS".to_string(),
            TokenKind::VOTER_CONSTRAINTS,
        );
        s.insert("VOTERS".to_string(), TokenKind::VOTERS);
        s.insert("VIEW".to_string(), TokenKind::VIEW);
        s.insert("VIRTUAL".to_string(), TokenKind::VIRTUAL);
        s.insert("VISIBLE".to_string(), TokenKind::VISIBLE);
        s.insert("WARNINGS".to_string(), TokenKind::WARNINGS);
        s.insert("WATCH".to_string(), TokenKind::WATCH);
        s.insert("WEEK".to_string(), TokenKind::WEEK);
        s.insert("WEIGHT_STRING".to_string(), TokenKind::WEIGHT_STRING);
        s.insert("WHEN".to_string(), TokenKind::WHEN);
        s.insert("WHERE".to_string(), TokenKind::WHERE);
        s.insert("WHILE".to_string(), TokenKind::WHILE);
        s.insert("WIDTH".to_string(), TokenKind::WIDTH);
        s.insert("WITH".to_string(), TokenKind::WITH);
        s.insert("WITHOUT".to_string(), TokenKind::WITHOUT);
        s.insert("WRITE".to_string(), TokenKind::WRITE);
        s.insert("WORKLOAD".to_string(), TokenKind::WORKLOAD);
        s.insert("X509".to_string(), TokenKind::X509);
        s.insert("XOR".to_string(), TokenKind::XOR);
        s.insert("YEAR_MONTH".to_string(), TokenKind::YEAR_MONTH);
        s.insert("YEAR".to_string(), TokenKind::YEAR);
        s.insert("ZEROFILL".to_string(), TokenKind::ZEROFILL);
        s.insert("WAIT".to_string(), TokenKind::WAIT);
        s.insert(
            "FAILED_LOGIN_ATTEMPTS".to_string(),
            TokenKind::FAILED_LOGIN_ATTEMPTS,
        );
        s.insert(
            "PASSWORD_LOCK_TIME".to_string(),
            TokenKind::PASSWORD_LOCK_TIME,
        );
        s.insert("REUSE".to_string(), TokenKind::REUSE);

        s
    };
}
