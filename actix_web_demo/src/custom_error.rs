use actix_web::{get,web, 
    http::{header::ContentType,StatusCode},

    App, error , HttpResponse, HttpServer,Result,HttpRequest};
use derive_more::{Display, Error};

#[derive(Debug,Display,Error)]
enum MyError {
     #[display("Internal Error")]
    InternalError,
    #[display("Bad Request")]
    BadClientData,
    #[display("Timeout")]
    Timeout,
}


impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
          HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        
        match *self{
            MyError::InternalError =>  StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::REQUEST_TIMEOUT,
        }

    }
}


#[get("/")]
async fn index() -> Result<&'static str,MyError>{
    Err(MyError::BadClientData)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
        .service(index)
        .route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}