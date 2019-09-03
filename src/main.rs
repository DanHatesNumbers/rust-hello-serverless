use lambda_http::{Body, lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

use serde::{Deserialize, Serialize};

fn main() {
    lambda!(handler)
}

#[derive(Serialize, Deserialize)]
struct Message {
    name: String
}

fn handler(
    req: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    match req.body() {
        Body::Empty => Ok(json!({"message": "Hello, world!"})),
        Body::Text(msg) => {
            let msg: Message = serde_json::from_str(&msg).unwrap();
            Ok(json!({"message": format!("Hello, {}!", msg.name )}))
        },
        _ => Err("Binary payload not supported".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Body;
    use lambda_http::http::Request;

    #[test]
    fn handler_handles_empty_request() {
        let request = Request::default();
        let expected = json!({
            "message": "Hello, world!"
        })
        .into_response();
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
    
    #[test]
    fn handler_handles_request_with_text_body() {
        let mut builder = Request::builder();
        let request = builder.body(Body::from(r#"{"name": "Dan"}"#)).unwrap();
        let expected = json!({
            "message": "Hello, Dan!"
        })
        .into_response();
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }

    #[test]
    fn handler_rejects_request_with_binary_body() {
        let mut builder = Request::builder();
        let request = builder.body(Body::from("Dan".as_bytes())).unwrap();
        let response = handler(request, Context::default());
        assert!(response.is_err())
    }
}
