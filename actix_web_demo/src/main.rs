use actix_web::{
    body::BoxBody,
    post,get,web, App, Responder , HttpResponse, HttpServer,Result,HttpRequest};
use serde::Deserialize;
use serde::Serialize;

    
    #[derive(Serialize)]
struct MyObj {
    name : &'static str,
}


impl Responder for MyObj {
    type Body = BoxBody;
    fn respond_to(self,_req:&HttpRequest)->HttpResponse<Self::Body> {
        
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }

}

#[get("/") ]
async fn index() -> impl Responder {
    MyObj { name : "Actix-web"}
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
