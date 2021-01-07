use actix_web::{middleware, web, App, HttpRequest, HttpResponse};

pub async fn index(req: HttpRequest) -> HttpResponse {
    println!("REQ: {:?}", req);
    HttpResponse::Ok()
    .content_type("text/plain")
    .body("Hello world!")
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http};

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}