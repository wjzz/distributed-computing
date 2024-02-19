use std::collections::HashMap;

use crate::domain::{ClientName, Repository, Task, ThreeNResult, ThreeNState};

pub struct InMemoryRepository {
    from: u64,
    to: u64,
    increment: u64,
    queue: HashMap<ClientName, Task>,
    results: HashMap<Task, (ClientName, ThreeNResult)>,
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

    fn add_to_queue(&mut self, task: Task, client_name: ClientName) {
        let _ = self.queue.insert(client_name, task);

        self.debug();
    }

    fn fetch_queued_task_by_client(&self, client_name: ClientName) -> Option<Task> {
        self.queue.get(&client_name).copied()
    }

    fn delete_queued_task_by_client(&mut self, client_name: ClientName) {
        let _ = self.queue.remove(&client_name);
    }

    fn fetch_results_by_client(&mut self, client_name: ClientName) -> Vec<ThreeNResult> {
        self.results
            .iter()
            .filter(|(_k, v)| v.0 == client_name)
            .map(|(_k, v)| v.1.clone())
            .collect()
    }

    fn store_results(&mut self, client_name: ClientName, task: Task, result: ThreeNResult) {
        self.results.insert(task, (client_name, result));

        self.debug();
    }
}
