
use actix_protobuf::*;
use actix_web::*;

#[derive(Clone, PartialEq, Message)]
pub struct MyObj {
    #[prost(int32, tag = "1")]
    pub number: i32,

    #[prost(string, tag = "2")]
    pub name: String,
}

pub async fn protobuf(msg: ProtoBuf<MyObj>) -> Result<HttpResponse> {
    println!("model: {:?}", msg);
    HttpResponse::Ok().protobuf(msg.0) // <- send response
}



#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http};

    #[actix_rt::test]
    async fn test_index_ok() {
        // let req = test::TestRequest::with_header("content-type", "application/protobuf").to_http_request();
        let resp = protobuf(ProtoBuf(MyObj {
            name: "tset".to_string(),
            number: 1,
        })).await.unwrap();
        let response_body = match resp.body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        println!("{:?}", response_body);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}