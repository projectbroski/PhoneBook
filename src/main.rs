mod config;
mod models;

use crate::models::{Status,Test};

use actix_web::{HttpServer,App,web,Responder,HttpResponse};
use std::io;

use dotenv::dotenv;
use tokio_postgres::NoTls

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status {status:"OK".to_string()})
}

async fn test() -> impl Responder {
    HttpResponse::Ok()
        .json(Test {id:1,name:"John".to_string()})
}

#[actix_web::main]
async fn main() -> io::Result <()> {
    dotenv().ok();
    let config =  crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Running on port {}",config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/",web::get().to(status))
            .route("/test",web::get().to(test))
    })
    .bind(format!("{}:{}",config.server.host,config.server.port))?
    .run()
    .await
}