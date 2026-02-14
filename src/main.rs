use actix_web::{web::Json, *};
use serde::*;
use std::{
    fs::{read_to_string, write},
    sync::Mutex,
};

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u16,
}

#[derive(Serialize, Deserialize)]
struct Login {
    user: String,
    key: String,
}

#[derive(Serialize, Deserialize)]
struct CollectiveData {
    visits: u128,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut _port: u16 = 8080;
    let file = read_to_string(String::from("./assets/data.json")).unwrap_or_else(|err| {
        eprintln!("{}", err);
        println!("Attempting to create the file");
        write("./assets/data.json", String::new()).unwrap();
        return String::new();
    });
    let data = match serde_json::from_str(&file) {
        Ok(res) => res,
        Err(_e) => CollectiveData { visits: 0 },
    };

    let collective_data = web::Data::new(Mutex::new(data));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(collective_data.clone())
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body(format!("API is live.")) }),
            )
            .service(
                web::scope("/api")
                    .service(home)
                    .service(display)
                    .service(displaymsg)
                    .service(query_param)
                    .service(login)
                    .service(visits),
            )
            .default_service(web::to(not_found))
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

#[get("/visits")]
async fn visits(collective_data: web::Data<Mutex<CollectiveData>>) -> impl Responder {
    let mut collection_data = collective_data.lock().unwrap();
    collection_data.visits += 1;

    let updated_data = serde_json::to_string_pretty(&(*collection_data)).unwrap();
    write("./assets/data.json", updated_data).unwrap();
    let response_msg = format!("/visits\n\nTotal Visits : {}", collection_data.visits);
    HttpResponse::Ok().body(response_msg)
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

#[post("/login")]
async fn login(data: Json<Login>) -> impl Responder {
    // let msg = format!("{{user: {},key: {}}}", data.user, data.key);
    // serde_json::to_string(&datunwrap() // is responding with a string, not json
    HttpResponse::Ok().json(data)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Route not found!")
}
