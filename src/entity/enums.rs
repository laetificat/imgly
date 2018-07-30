use diesel::sql_types::*;
use diesel::backend::Backend;
use diesel::deserialize::{ self, FromSql };
use diesel::serialize::{ self, ToSql, Output };
use std::io;

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "SmallInt"]
pub enum Visibility {
    PUBLIC,
    PRIVATE,
    DIRECT,
    UNLISTED
}

impl<DB: Backend> ToSql<SmallInt, DB> for Visibility where i16: ToSql<SmallInt, DB>, {
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result where W: io::Write, {
        let v = match *self {
            Visibility::PUBLIC => 1,
            Visibility::PRIVATE => 2,
            Visibility::DIRECT => 3,
            Visibility::UNLISTED => 4
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<SmallInt, DB> for Visibility where i16: FromSql<SmallInt, DB>, {
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = i16::from_sql(bytes)?;
        Ok(match v {
            1 => Visibility::PUBLIC,
            2 => Visibility::PRIVATE,
            3 => Visibility::DIRECT,
            4 => Visibility::UNLISTED,
            _ => return Err("Something wrong happened".into())
        })
    }
}