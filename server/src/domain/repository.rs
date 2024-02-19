use super::types::{ClientName, ThreeNResult, ThreeNState};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FromTo {
    pub from: u64,
    pub to: u64,
}

pub trait Repository {
    // TODO: how to handle transactions?

    fn fetch_current_state(&self) -> ThreeNState;
    fn update_from(&mut self, from: u64);
    fn add_to_queue(&mut self, fromto: FromTo, client_name: ClientName);
    fn store_results(&mut self, client_name: ClientName, from: u64, to: u64, result: ThreeNResult);
}
