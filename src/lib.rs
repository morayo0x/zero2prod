use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// === === === GET handlers === === ===
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish() // build the HttpResponse from the HttpResponseBuilder
}

// === === === POST handlers === === ===
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// always listening for incoming connections
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // let stream = TcpListener::bind("127.0.0.1:0")?;
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // POST/ subscriptions request
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
