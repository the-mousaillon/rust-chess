
#[actix::main]
async fn main() -> std::io::Result<()> {
    rust_chess::server::run_dev_app().await
}