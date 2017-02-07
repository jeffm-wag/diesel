use diesel::*;
#[cfg(feature = "postgres")]
use diesel::pg::Pg;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
use diesel::types::{HasSqlType, FromSqlRow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInformation {
    pub column_name: String,
    pub type_name: String,
    pub nullable: bool,
}

pub struct ColumnType {
    pub path: Vec<String>,
    pub is_array: bool,
    pub is_nullable: bool,
}

impl ColumnInformation {
    pub fn new<T, U>(column_name: T, type_name: U, nullable: bool) -> Self where
        T: Into<String>,
        U: Into<String>,
    {
        ColumnInformation {
            column_name: column_name.into(),
            type_name: type_name.into(),
            nullable: nullable,
        }
    }
}

#[cfg(feature = "postgres")]
impl<ST> Queryable<ST, Pg> for ColumnInformation where
    Pg: HasSqlType<ST>,
    (String, String, String): FromSqlRow<ST, Pg>,
{
    type Row = (String, String, String);

    fn build(row: Self::Row) -> Self {
        ColumnInformation::new(row.0, row.1, row.2 == "YES")
    }
}

#[cfg(feature = "sqlite")]
impl<ST> Queryable<ST, Sqlite> for ColumnInformation where
    Sqlite: HasSqlType<ST>,
    (i32, String, String, bool, Option<String>, bool): FromSqlRow<ST, Sqlite>,
{
    type Row = (i32, String, String, bool, Option<String>, bool);

    fn build(row: Self::Row) -> Self {
        ColumnInformation::new(row.1, row.2, !row.3)
    }
}