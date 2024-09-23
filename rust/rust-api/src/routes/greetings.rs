
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[derive(Deserialize)]
struct Person {
    first_name: String,
    last_name: String
}
#[get("/greet")]
async fn greet(person: web::Query<Person>) -> impl Responder {
    let greeting =  format!("Hello {} {} !", person.first_name, person.last_name);
    HttpResponse::Ok().body(greeting)
}