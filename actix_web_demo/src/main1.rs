use actix_web::{web,get, post, put, delete, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex as Mut;

struct AppState {
 app_name: String,
 count:Mut<i32>,
}

struct MyData {
    value: String,
}


#[get("/")]
async fn hello(data : web::Data<AppState>) -> impl Responder {
    
    let mut    count = data.count.lock().unwrap();
    *count += 1;
    let app_name = &data.app_name;
    let current_count = *count;
    println!("App Name: {} , Count : {}", app_name,current_count); 
    "Get API is working| for ".to_owned() + app_name + " with count: " + &current_count.to_string()
}


fn scoped_config(cfg:&mut web::ServiceConfig){
    cfg.service(web::resource("/scoped")
    .route(web::get().to(|| async {HttpResponse::Ok().body("This is scope test") }))
    .route(web::head().to(HttpResponse::MethodNotAllowed))
    );

}

fn config(cfg: &mut web::ServiceConfig){
    
    cfg.service(
        web::resource("/app")
          .route(web::get().to(|| async {HttpResponse::Ok().body("Config test")}))
          .route(web::head().to(HttpResponse::MethodNotAllowed))
    )   ;

}

#[post("/create")]
async fn create() -> impl Responder {
    HttpResponse::Created().body("resource created successfully|")
}

#[put("/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("resource updated successfully|")
}



#[delete("/delete")]
async fn delete() -> impl Responder {
    HttpResponse::NoContent().finish()
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .configure(config)
            .data(AppState {
                app_name: String::from("Actix Web Demo"),
                count: Mut::new(0),
            })
            .service(web::scope("/scoped_app").configure(scoped_config))
            .service(hello)
            .service(create)
            .service(update)
            .service(delete)
              })
             .bind("127.0.0.1:8080")?
             .run()
             .await
}
