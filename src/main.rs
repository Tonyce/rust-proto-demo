use actix_web::{middleware, web, App, HttpRequest, HttpServer};

use saturn::route::{self, proto};

use saturn::items;


pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", create_large_shirt("red".to_string()));
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
