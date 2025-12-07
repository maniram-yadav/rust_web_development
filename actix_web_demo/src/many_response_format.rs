use actix_web::{Either, Error, HttpResponse,App,HttpServer,get, web};

type RegisterResult = Either<HttpResponse,Result<&'static str, Error>>;

#[get("/")]    
async fn index() ->RegisterResult{
    
    if true {
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::Right(Ok("Hello world"))
    }
}    




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(index)
        })
     .bind(("127.0.0.1", 8080))?
        .run()
        .await 
}
