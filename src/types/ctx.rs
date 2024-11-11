/// https://sqlite.org/datatype3.html#storage_classes_and_datatypes
#[derive(Debug)]
pub enum SqliteStorageClass {
    NULL,
    INTEGER,
    REAL,
    TEXT,
    BLOB,
}

trait StrExtension {
    /// returns if s contains any of the elements of v
    fn contains_any(self, v: Vec<&str>) -> bool;
}

impl StrExtension for &str {
    fn contains_any(self, v: Vec<&str>) -> bool {
        for e in v {
            if self.contains(e) {
                return true;
            }
        }
        false
    }
}

impl SqliteStorageClass {
    /// https://sqlite.org/datatype3.html#determination_of_column_affinity
    fn from_str(s: &str) -> Self {
        return if s.contains_any(vec!["VARCHAR", "CLOB", "TEXT"]) {
            Self::TEXT
        } else if s == "" || s.contains("BLOB") {
            Self::BLOB
        } else if s.contains_any(vec!["REAL", "FLOA", "DOUB"]) {
            Self::REAL
        } else if s.contains("INT") {
            Self::INTEGER
        } else {
            // includes TRUE, FALSE and anything else
            Self::INTEGER
        };
    }
}

pub struct Table {
    pub name: String,
    pub columns: Vec<SqliteStorageClass>,
}

/// Context holds information necessary for the analysis of sql statements.
pub struct Context {
    pub tables: Vec<Table>,
    pub save_points: Vec<String>,
}
