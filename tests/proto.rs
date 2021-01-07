

use saturn::route::proto;
use actix_web::dev::Service;
use actix_web::{http, test, web, App, Error};
use prost::Message;

#[actix_rt::test]
async fn test_index() -> Result<(), Error> {
    let app = App::new().route("/proto", web::post().to(proto::protobuf));
    let mut app = test::init_service(app).await;

    let payload = proto::MyObj {
        name: "tset".to_string(),
        number: 1,
    };
    let mut p2p_vec = vec![];
    payload.encode(&mut p2p_vec).unwrap();

    let req = test::TestRequest::post().uri("/proto").header("content-type", "application/protobuf").set_payload(p2p_vec).to_request();
    let resp = app.call(req).await.unwrap();

    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    let r: proto::MyObj = proto::MyObj::decode(&response_body[..]).unwrap();

    assert_eq!(r.name, r##"tset"##);

    Ok(())
}
