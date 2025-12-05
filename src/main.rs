use actix_web::*;
use serde::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .body("Running Live powered by RUST | Actix_Web   ".to_string())
                }),
            )
            .service(home)
            .service(display)
            .service(displaymsg)
            .service(query_param)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

#[get("/home")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("/home")
}

#[get("/display")]
async fn display() -> impl Responder {
    HttpResponse::Ok().body("/display")
}

#[get("/display/{msg}")]
async fn displaymsg(path: web::Path<String>) -> impl Responder {
    // let msg = format!("/display/{}", path.into_inner());
    HttpResponse::Ok().body(path.into_inner())
}

#[get("/query")]
async fn query_param(query: web::Query<Info>) -> impl Responder {
    let msg = format!("name: {},\nage: {}", query.name, query.age);
    HttpResponse::Ok().body(msg)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u16,
}
