use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status:String
}

#[derive(Serialize)]
pub struct Test {
    pub id:i32,
    pub name: String
}