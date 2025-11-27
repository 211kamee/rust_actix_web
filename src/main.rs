use actix_web::*;

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
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

#[get("/home")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("/home")
}
