use actix_web::{web::Json, *};
use serde::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut _port: u16 = 8080;

    let msg: String = "Fahhhhh".to_string();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(msg.clone()))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body(format!("API is live.")) }),
            )
            .service(home)
            .service(display)
            .service(displaymsg)
            .service(query_param)
            .service(login)
    })
    .bind(("0.0.0.0", _port))?;

    println!("Bound to : http://localhost:{}", _port);

    server.run().await
}

#[get("/home")]
async fn home(msg: web::Data<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("/home\n{}", msg.as_ref()))
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

#[post("/login")]
async fn login(data: Json<Login>) -> impl Responder {
    // let msg = format!("{{user: {},key: {}}}", data.user, data.key);
    // serde_json::to_string(&datunwrap() // is responding with a string, not json
    HttpResponse::Ok().json(data)
}

#[derive(Serialize, Deserialize)]
struct Login {
    user: String,
    key: String,
}
