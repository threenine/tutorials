use actix_web::{web, get, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    first_name: String,
    last_name: String
}
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World! ")
}

#[get("/greet")]
async fn greet(person: web::Query<Person>) -> impl Responder {
    let greeting =  format!("Hello {} {} !", person.first_name, person.last_name);
    HttpResponse::Ok().body(greeting)
}