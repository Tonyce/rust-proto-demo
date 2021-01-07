use actix_web::{middleware, web, App, HttpRequest, HttpServer};

use saturn::route::{self, proto};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(route::index))
            .service(web::resource("/proto").to(proto::protobuf))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
