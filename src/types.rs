#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Keyword {
    ABORT,
    ACTION,
    ADD,
    AFTER,
    ALL,
    ALTER,
    ALWAYS,
    ANALYZE,
    AND,
    AS,
    ASC,
    ATTACH,
    AUTOINCREMENT,
    BEFORE,
    BEGIN,
    BETWEEN,
    BY,
    CASCADE,
    CASE,
    CAST,
    CHECK,
    COLLATE,
    COLUMN,
    COMMIT,
    CONFLICT,
    CONSTRAINT,
    CREATE,
    CROSS,
    CURRENT,
    CURRENT_DATE,
    CURRENT_TIME,
    CURRENT_TIMESTAMP,
    DATABASE,
    DEFAULT,
    DEFERRABLE,
    DEFERRED,
    DELETE,
    DESC,
    DETACH,
    DISTINCT,
    DO,
    DROP,
    EACH,
    ELSE,
    END,
    ESCAPE,
    EXCEPT,
    EXCLUDE,
    EXCLUSIVE,
    EXISTS,
    EXPLAIN,
    FAIL,
    FILTER,
    FIRST,
    FOLLOWING,
    FOR,
    FOREIGN,
    FROM,
    FULL,
    GENERATED,
    GLOB,
    GROUP,
    GROUPS,
    HAVING,
    IF,
    IGNORE,
    IMMEDIATE,
    IN,
    INDEX,
    INDEXED,
    INITIALLY,
    INNER,
    INSERT,
    INSTEAD,
    INTERSECT,
    INTO,
    IS,
    ISNULL,
    JOIN,
    KEY,
    LAST,
    LEFT,
    LIKE,
    LIMIT,
    MATCH,
    MATERIALIZED,
    NATURAL,
    NO,
    NOT,
    NOTHING,
    NOTNULL,
    NULL,
    NULLS,
    OF,
    OFFSET,
    ON,
    OR,
    ORDER,
    OTHERS,
    OUTER,
    OVER,
    PARTITION,
    PLAN,
    PRAGMA,
    PRECEDING,
    PRIMARY,
    QUERY,
    RAISE,
    RANGE,
    RECURSIVE,
    REFERENCES,
    REGEXP,
    REINDEX,
    RELEASE,
    RENAME,
    REPLACE,
    RESTRICT,
    RETURNING,
    RIGHT,
    ROLLBACK,
    ROW,
    ROWS,
    SAVEPOINT,
    SELECT,
    SET,
    TABLE,
    TEMP,
    TEMPORARY,
    THEN,
    TIES,
    TO,
    TRANSACTION,
    TRIGGER,
    UNBOUNDED,
    UNION,
    UNIQUE,
    UPDATE,
    USING,
    VACUUM,
    VALUES,
    VIEW,
    VIRTUAL,
    WHEN,
    WHERE,
    WINDOW,
    WITH,
    WITHOUT,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        Some(match s.to_uppercase().as_str() {
            "ABORT" => Keyword::ABORT,
            "ACTION" => Keyword::ACTION,
            "ADD" => Keyword::ADD,
            "AFTER" => Keyword::AFTER,
            "ALL" => Keyword::ALL,
            "ALTER" => Keyword::ALTER,
            "ALWAYS" => Keyword::ALWAYS,
            "ANALYZE" => Keyword::ANALYZE,
            "AND" => Keyword::AND,
            "AS" => Keyword::AS,
            "ASC" => Keyword::ASC,
            "ATTACH" => Keyword::ATTACH,
            "AUTOINCREMENT" => Keyword::AUTOINCREMENT,
            "BEFORE" => Keyword::BEFORE,
            "BEGIN" => Keyword::BEGIN,
            "BETWEEN" => Keyword::BETWEEN,
            "BY" => Keyword::BY,
            "CASCADE" => Keyword::CASCADE,
            "CASE" => Keyword::CASE,
            "CAST" => Keyword::CAST,
            "CHECK" => Keyword::CHECK,
            "COLLATE" => Keyword::COLLATE,
            "COLUMN" => Keyword::COLUMN,
            "COMMIT" => Keyword::COMMIT,
            "CONFLICT" => Keyword::CONFLICT,
            "CONSTRAINT" => Keyword::CONSTRAINT,
            "CREATE" => Keyword::CREATE,
            "CROSS" => Keyword::CROSS,
            "CURRENT" => Keyword::CURRENT,
            "CURRENT_DATE" => Keyword::CURRENT_DATE,
            "CURRENT_TIME" => Keyword::CURRENT_TIME,
            "CURRENT_TIMESTAMP" => Keyword::CURRENT_TIMESTAMP,
            "DATABASE" => Keyword::DATABASE,
            "DEFAULT" => Keyword::DEFAULT,
            "DEFERRABLE" => Keyword::DEFERRABLE,
            "DEFERRED" => Keyword::DEFERRED,
            "DELETE" => Keyword::DELETE,
            "DESC" => Keyword::DESC,
            "DETACH" => Keyword::DETACH,
            "DISTINCT" => Keyword::DISTINCT,
            "DO" => Keyword::DO,
            "DROP" => Keyword::DROP,
            "EACH" => Keyword::EACH,
            "ELSE" => Keyword::ELSE,
            "END" => Keyword::END,
            "ESCAPE" => Keyword::ESCAPE,
            "EXCEPT" => Keyword::EXCEPT,
            "EXCLUDE" => Keyword::EXCLUDE,
            "EXCLUSIVE" => Keyword::EXCLUSIVE,
            "EXISTS" => Keyword::EXISTS,
            "EXPLAIN" => Keyword::EXPLAIN,
            "FAIL" => Keyword::FAIL,
            "FILTER" => Keyword::FILTER,
            "FIRST" => Keyword::FIRST,
            "FOLLOWING" => Keyword::FOLLOWING,
            "FOR" => Keyword::FOR,
            "FOREIGN" => Keyword::FOREIGN,
            "FROM" => Keyword::FROM,
            "FULL" => Keyword::FULL,
            "GENERATED" => Keyword::GENERATED,
            "GLOB" => Keyword::GLOB,
            "GROUP" => Keyword::GROUP,
            "GROUPS" => Keyword::GROUPS,
            "HAVING" => Keyword::HAVING,
            "IF" => Keyword::IF,
            "IGNORE" => Keyword::IGNORE,
            "IMMEDIATE" => Keyword::IMMEDIATE,
            "IN" => Keyword::IN,
            "INDEX" => Keyword::INDEX,
            "INDEXED" => Keyword::INDEXED,
            "INITIALLY" => Keyword::INITIALLY,
            "INNER" => Keyword::INNER,
            "INSERT" => Keyword::INSERT,
            "INSTEAD" => Keyword::INSTEAD,
            "INTERSECT" => Keyword::INTERSECT,
            "INTO" => Keyword::INTO,
            "IS" => Keyword::IS,
            "ISNULL" => Keyword::ISNULL,
            "JOIN" => Keyword::JOIN,
            "KEY" => Keyword::KEY,
            "LAST" => Keyword::LAST,
            "LEFT" => Keyword::LEFT,
            "LIKE" => Keyword::LIKE,
            "LIMIT" => Keyword::LIMIT,
            "MATCH" => Keyword::MATCH,
            "MATERIALIZED" => Keyword::MATERIALIZED,
            "NATURAL" => Keyword::NATURAL,
            "NO" => Keyword::NO,
            "NOT" => Keyword::NOT,
            "NOTHING" => Keyword::NOTHING,
            "NOTNULL" => Keyword::NOTNULL,
            "NULL" => Keyword::NULL,
            "NULLS" => Keyword::NULLS,
            "OF" => Keyword::OF,
            "OFFSET" => Keyword::OFFSET,
            "ON" => Keyword::ON,
            "OR" => Keyword::OR,
            "ORDER" => Keyword::ORDER,
            "OTHERS" => Keyword::OTHERS,
            "OUTER" => Keyword::OUTER,
            "OVER" => Keyword::OVER,
            "PARTITION" => Keyword::PARTITION,
            "PLAN" => Keyword::PLAN,
            "PRAGMA" => Keyword::PRAGMA,
            "PRECEDING" => Keyword::PRECEDING,
            "PRIMARY" => Keyword::PRIMARY,
            "QUERY" => Keyword::QUERY,
            "RAISE" => Keyword::RAISE,
            "RANGE" => Keyword::RANGE,
            "RECURSIVE" => Keyword::RECURSIVE,
            "REFERENCES" => Keyword::REFERENCES,
            "REGEXP" => Keyword::REGEXP,
            "REINDEX" => Keyword::REINDEX,
            "RELEASE" => Keyword::RELEASE,
            "RENAME" => Keyword::RENAME,
            "REPLACE" => Keyword::REPLACE,
            "RESTRICT" => Keyword::RESTRICT,
            "RETURNING" => Keyword::RETURNING,
            "RIGHT" => Keyword::RIGHT,
            "ROLLBACK" => Keyword::ROLLBACK,
            "ROW" => Keyword::ROW,
            "ROWS" => Keyword::ROWS,
            "SAVEPOINT" => Keyword::SAVEPOINT,
            "SELECT" => Keyword::SELECT,
            "SET" => Keyword::SET,
            "TABLE" => Keyword::TABLE,
            "TEMP" => Keyword::TEMP,
            "TEMPORARY" => Keyword::TEMPORARY,
            "THEN" => Keyword::THEN,
            "TIES" => Keyword::TIES,
            "TO" => Keyword::TO,
            "TRANSACTION" => Keyword::TRANSACTION,
            "TRIGGER" => Keyword::TRIGGER,
            "UNBOUNDED" => Keyword::UNBOUNDED,
            "UNION" => Keyword::UNION,
            "UNIQUE" => Keyword::UNIQUE,
            "UPDATE" => Keyword::UPDATE,
            "USING" => Keyword::USING,
            "VACUUM" => Keyword::VACUUM,
            "VALUES" => Keyword::VALUES,
            "VIEW" => Keyword::VIEW,
            "VIRTUAL" => Keyword::VIRTUAL,
            "WHEN" => Keyword::WHEN,
            "WHERE" => Keyword::WHERE,
            "WINDOW" => Keyword::WINDOW,
            "WITH" => Keyword::WITH,
            "WITHOUT" => Keyword::WITHOUT,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum Type {
    Keyword(Keyword),
    Ident(String),
    Number(f64),
    String(String),
}

#[derive(Debug)]
pub struct Token {
    pub ttype: Type,
    pub start: usize,
    pub end: usize,
}