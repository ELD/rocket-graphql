table! {
    use diesel::sql_types::*;
    use crate::models::EpisodeSqlType;
    humans (id) {
        id -> Int4,
        name -> Varchar,
        appears_in -> Array<EpisodeSqlType>,
        home_planet -> Varchar,
    }
}
