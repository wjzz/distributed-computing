use super::{
    current_timestamp,
    types::{ClientName, ThreeNResult, ThreeNState, Timestamp},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RawTask {
    pub from: u64,
    pub to: u64,
}

#[derive(Debug, Clone)]
pub struct StartedTask {
    pub from: u64,
    pub to: u64,
    pub started_at: Timestamp,
}

impl RawTask {
    pub fn to_started_task(self) -> StartedTask {
        let RawTask { from, to } = self;
        StartedTask {
            from,
            to,
            started_at: current_timestamp(),
        }
    }
}

impl StartedTask {
    pub fn into_raw_task(self) -> RawTask {
        let StartedTask { from, to, .. } = self;
        RawTask { from, to }
    }
}

pub trait Repository {
    // TODO: how to handle transactions?

    fn fetch_current_state(&mut self) -> ThreeNState;
    fn add_to_queue(&mut self, task: RawTask, client_name: ClientName);
    fn fetch_queued_task_by_client(&mut self, client_name: ClientName) -> Option<StartedTask>;
    fn delete_queued_task_by_client(&mut self, client_name: ClientName);
    fn update_from(&mut self, from: u64);
    fn fetch_results_by_client(&mut self, client_name: ClientName) -> Vec<ThreeNResult>;
    fn store_results(&mut self, client_name: ClientName, task: StartedTask, result: ThreeNResult);
}
