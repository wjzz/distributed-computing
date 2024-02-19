mod domain;
mod http;
mod inmemory_repository;
mod usecase;

use domain::{ClientName, Repository, ThreeNResponse, ThreeNResult};
use http::{serve, Request};
use inmemory_repository::InMemoryRepository;
use serde::Deserialize;
use usecase::{handle_ready, handle_solved};

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

fn router<R: Repository>(repo: &mut R, request: ThreeNRequest) -> ThreeNResponse {
    match request {
        ThreeNRequest::Ready { client_name } => handle_ready(repo, client_name),
        ThreeNRequest::Solved {
            client_name,
            from,
            to,
            result,
        } => handle_solved(repo, client_name, from, to, result),
    }
}

pub fn handler<R: Repository>(request_body: Request, repo: &mut R) -> String {
    // println!("request body = {}", request_body);
    if let Ok(request) = serde_json::from_str(&request_body) {
        let response = router(repo, request);
        let response = serde_json::to_string(&response).unwrap();
        // println!("response = {}", response);
        response
    } else {
        "incorrect request".to_string()
    }
}

fn main() {
    let port = 5555;
    let mut repo = InMemoryRepository::new();
    serve(port, handler, &mut repo);
}
