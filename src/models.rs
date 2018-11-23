use super::schema::*;
use diesel::*;
use diesel::pg::Pg;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use juniper::FieldResult;

#[derive(SqlType)]
#[postgres(type_name = "episode")]
pub struct EpisodeSqlType;

#[derive(GraphQLEnum, Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "EpisodeSqlType"]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

impl ToSql<EpisodeSqlType, Pg> for Episode {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Episode::NewHope => out.write_all(b"a_new_hope")?,
            Episode::Empire => out.write_all(b"empire_strikes_back")?,
            Episode::Jedi => out.write_all(b"return_of_the_jedi")?,
        };
        Ok(IsNull::No)
    }
}

impl FromSql<EpisodeSqlType, Pg> for Episode {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"a_new_hope" => Ok(Episode::NewHope),
            b"empire_strikes_back" => Ok(Episode::Empire),
            b"return_of_the_jedi" => Ok(Episode::Jedi),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(GraphQLObject, Queryable)]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "humans"]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}
