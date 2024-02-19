use std::collections::HashMap;

use crate::domain::{ClientName, FromTo, Repository, ThreeNResult, ThreeNState};

pub struct InMemoryRepository {
    from: u64,
    to: u64,
    increment: u64,
    queue: HashMap<ClientName, FromTo>,
    results: HashMap<FromTo, (ClientName, ThreeNResult)>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            from: 1,
            to: 1_000_000_000,
            increment: 1_000_000,
            queue: HashMap::new(),
            results: HashMap::new(),
        }
    }

    pub fn debug(&self) {
        eprintln!(
            "current state: from = {}, to = {}, queue = {:?} results = {:?}",
            self.from, self.to, self.queue, self.results
        );
    }
}

impl Repository for InMemoryRepository {
    fn fetch_current_state(&self) -> ThreeNState {
        ThreeNState {
            from: self.from,
            to: self.to,
            increment: self.increment,
        }
    }

    fn update_from(&mut self, from: u64) {
        self.from = from;
    }

    fn add_to_queue(&mut self, from_to: FromTo, client_name: ClientName) {
        let _ = self.queue.insert(client_name, from_to);

        self.debug();
    }

    fn store_results(&mut self, client_name: ClientName, from: u64, to: u64, result: ThreeNResult) {
        self.results
            .insert(FromTo { from, to }, (client_name, result));

        self.debug();
    }
}
