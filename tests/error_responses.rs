//! An error response reaches the caller with its status, headers and decoded body, so a 429's `Retry-After` says when
//! to send the request again

use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use docspring::{Api, Error, GetTemplateError, GetTemplateParams};

// A server on a local port that answers one request with `response`. Returns its address.
fn serve_once(response: String) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a local port");
    let address = listener.local_addr().expect("the port's address");
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("the request");
        // The request's headers: a GET has no body
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            let read = stream.read(&mut buffer).expect("the request's headers");
            assert!(read > 0, "the client closed the connection mid-request");
            request.extend_from_slice(&buffer[..read]);
        }
        stream.write_all(response.as_bytes()).expect("the response");
    });
    format!("http://{address}")
}

#[tokio::test]
async fn a_429_keeps_its_retry_after_header() {
    let body = r#"{"error":{"code":"rate_limited","message":"Too many requests","details":[],"request_id":"req_1"}}"#;
    let host = serve_once(format!(
        "HTTP/1.1 429 Too Many Requests\r\nContent-Type: application/json\r\nRetry-After: 7\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    ));
    let client = docspring::Client::builder()
        .token_id("api_token_id")
        .token_secret("api_token_secret")
        .host(host)
        .build()
        .expect("a client");

    let error = client
        .get_template(
            GetTemplateParams::builder()
                .template_id("tpl_1234567890abcdef01")
                .build(),
        )
        .await
        .expect_err("the 429");

    // The message says what the API said, not just the status
    let message = error.to_string();
    assert!(
        message.contains("429") && message.contains("Too many requests"),
        "{message}"
    );
    let Error::ResponseError(response) = error else {
        panic!("expected a response error, got {error:?}");
    };
    assert_eq!(response.status, 429);
    assert_eq!(
        response.headers.get("retry-after").expect("Retry-After"),
        "7"
    );
    let Some(GetTemplateError::Status429(body)) = response.entity else {
        panic!("expected the 429's variant, got {:?}", response.entity);
    };
    assert_eq!(body.error.message, "Too many requests");
}
