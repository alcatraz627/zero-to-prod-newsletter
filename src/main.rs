use zero2prod::{get_random_port, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (listener, port) = get_random_port();
    println!("Running on http://0.0.0.0:{port}");

    run(listener)?.await
}
