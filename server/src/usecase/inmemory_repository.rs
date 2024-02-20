use std::collections::HashMap;

use crate::domain::{
    ClientName, RawTask, Repository, StartedTask, ThreeNResult, ThreeNState, Timestamp,
};

pub struct InMemoryRepository {
    from: u64,
    to: u64,
    increment: u64,
    queue: HashMap<ClientName, StartedTask>,
    results: HashMap<RawTask, (ClientName, Timestamp, ThreeNResult)>,
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
}

impl Repository for InMemoryRepository {
    fn fetch_current_state(&mut self) -> ThreeNState {
        ThreeNState {
            from: self.from,
            to: self.to,
            increment: self.increment,
        }
    }

    fn update_from(&mut self, from: u64) {
        self.from = from;
    }

    fn add_to_queue(&mut self, task: RawTask, client_name: ClientName) {
        let _ = self.queue.insert(client_name, task.to_started_task());
    }

    fn fetch_queued_task_by_client(&mut self, client_name: ClientName) -> Option<StartedTask> {
        self.queue.get(&client_name).cloned()
    }

    fn delete_queued_task_by_client(&mut self, client_name: ClientName) {
        let _ = self.queue.remove(&client_name);
    }

    fn fetch_results_by_client(&mut self, client_name: ClientName) -> Vec<ThreeNResult> {
        self.results
            .iter()
            .filter(|(_k, v)| v.0 == client_name)
            .map(|(_k, v)| v.2.clone())
            .collect()
    }

    fn store_results(&mut self, client_name: ClientName, task: StartedTask, result: ThreeNResult) {
        let started_at = task.clone().started_at;
        self.results
            .insert(task.to_raw_task(), (client_name, started_at, result));
    }
}
