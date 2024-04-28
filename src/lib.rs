use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub fn get_random_port() -> (TcpListener, u16) {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    return (listener, port);
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hola {}!", &name)
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
