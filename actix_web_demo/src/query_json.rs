use actix_web::{post,get,web, App, Responder , HttpResponse, HttpServer,Result,HttpRequest};
use serde::Deserialize;
use std::{
    cell:Cell,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    
}

#[derive(Deserialize)]
struct Info {
    username: String   ,
}


// Query
#[get("/search")]
async fn search(info : web::Query<Info>) -> String {
    format!("Welcom: {}!", info.username)
}


// url encoded form data
#[post("/submit")]
async fn submit_form(info : web::Form<Info>) -> String {
    format!("Welcom: {}!", info.username)
}



// Json
#[post("/submit")]
async fn submit(info : web::Json<Info>) -> String {
    format!("Welcome: {}!", info.username)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
        .service(search)
        .service(submit)
        .route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
