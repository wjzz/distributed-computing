mod http;

use http::{serve, Request};
use serde::{Deserialize, Serialize};

type ClientName = String;
type ThreeNResult = String;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ThreeNRequest {
    Ready {
        client_name: ClientName,
    },
    Solved {
        client_name: ClientName,
        from: String,
        to: String,
        result: ThreeNResult,
    },
}

#[derive(Serialize)]
enum ThreeNResponse {
    Solve { from: u64, to: u64 },
    Ok,
}

fn handle_three_n_request(request: ThreeNRequest) -> ThreeNResponse {
    let from = 1;
    let to = 100;

    match request {
        ThreeNRequest::Ready { client_name } => ThreeNResponse::Solve { from, to },
        ThreeNRequest::Solved {
            client_name,
            from,
            to,
            result,
        } => ThreeNResponse::Ok,
    }
}

pub fn handler(request_body: Request) -> String {
    println!("request body = {}", request_body);
    if let Ok(request) = serde_json::from_str(&request_body) {
        let response = handle_three_n_request(request);
        let response = serde_json::to_string(&response).unwrap();
        println!("response = {}", response);
        response
    } else {
        "incorrect request".to_string()
    }
}

fn main() {
    let port = 5555;
    serve(port, handler);
}
