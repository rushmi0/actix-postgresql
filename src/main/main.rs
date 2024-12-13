mod app;
mod module;
mod services;
mod storage;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
