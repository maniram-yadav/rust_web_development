use actix_web::{get,web, App, Responder , HttpResponse, HttpServer,Result,HttpRequest};

struct Info {
    username: String,
    age: u8,
}

// #[get("/users/{username}/{age}")]
// async fn index(path : web::Path<(String,String)>, json : web::Json<Info>) -> impl Responder  {
//     let path = path.into_inner();
//     format!("{} {} {} {}, path.0, path.1,json.username,json.age")
// }


// impl1 1
// #[get("/users/{user_id}/{friend}")] // <- define path parameters
// async fn index(path: web::Path<(u32, String)>) -> Result<String> {
//     let (user_id, friend) = path.into_inner();
//     Ok(format!("Welcome {}, user_id {}!", friend, user_id))
// }


// implindex 2
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index2(req : HttpRequest) -> Result<String> {
    let user_id : u32 = req.match_info().get("user_id").unwrap().parse().unwrap();
    let friend:String = req.match_info().get("friend").unwrap().to_string();

    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
        // .service(index)
        .service(index2)
        .route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

    // multi threaded server
    // let _= HttpServer::new(||App::new().route("/".web.get()
    //         .to(HttpResponse::Ok))).workers(2);


}