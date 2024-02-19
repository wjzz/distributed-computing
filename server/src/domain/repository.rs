use super::types::{ClientName, ThreeNResult, ThreeNState};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Task {
    pub from: u64,
    pub to: u64,
}

pub trait Repository {
    // TODO: how to handle transactions?

    fn fetch_current_state(&self) -> ThreeNState;
    fn add_to_queue(&mut self, task: Task, client_name: ClientName);
    fn fetch_queued_task_by_client(&self, client_name: ClientName) -> Option<Task>;
    fn delete_queued_task_by_client(&mut self, client_name: ClientName);
    fn update_from(&mut self, from: u64);
    fn fetch_results_by_client(&mut self, client_name: ClientName) -> Vec<ThreeNResult>;
    fn store_results(&mut self, client_name: ClientName, task: Task, result: ThreeNResult);
}
