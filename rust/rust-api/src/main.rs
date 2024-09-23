use actix_web::{ App, HttpServer};
mod routes {
    pub mod greetings;
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::greetings::hello_world)
            .service(routes::greetings::greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}