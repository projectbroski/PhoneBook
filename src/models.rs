use serde::{Serialize,Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status:String
}

#[derive(Serialize)]
pub struct Test {
    pub id:i32,
    pub name: String
}

#[derive(Serialize,Deserialize)]
#[pg_mapper(table="users")]
pub struct Profiles {
    pub id: i32,
    pub email:String,
    pub name: String
}