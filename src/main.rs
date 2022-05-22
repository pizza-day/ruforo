#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ruforo::init::init();
    ruforo::init::init_db().await;
    //ruforo::permission::init().await;
    ruforo::init::start().await
}
