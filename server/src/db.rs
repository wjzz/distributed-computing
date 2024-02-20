use postgres::{Client, NoTls};

use crate::domain::{
    ClientName, RawTask, Repository, StartedTask, ThreeNResult, ThreeNState, Timestamp,
};

pub struct PgRepository {
    client: Client,
}

impl PgRepository {
    pub fn new() -> Self {
        let url = "postgresql://postgres:password@localhost:25432/postgres";
        let client = Client::connect(url, NoTls).unwrap();
        eprintln!("Database connected!");
        PgRepository { client }
    }
}

impl Repository for PgRepository {
    fn fetch_current_state(&mut self) -> ThreeNState {
        let result = self
            .client
            .query_one("SELECT from_, to_, step FROM state", &[])
            .unwrap();
        ThreeNState {
            from: result.get::<usize, i64>(0) as u64,
            to: result.get::<usize, i64>(1) as u64,
            increment: result.get::<usize, i64>(2) as u64,
        }
    }

    fn add_to_queue(&mut self, task: RawTask, client_name: ClientName) {
        self.client
            .execute(
                "INSERT INTO queue(client_name, task_from, task_to) VALUES ($1, $2, $3)",
                &[&client_name, &(task.from as i64), &(task.to as i64)],
            )
            .unwrap();
    }

    fn fetch_queued_task_by_client(&mut self, client_name: ClientName) -> Option<StartedTask> {
        let result = self
            .client
            .query(
                "SELECT task_from, task_to, queued_at FROM queue WHERE client_name = $1",
                &[&client_name],
            )
            .unwrap();
        if result.len() == 0 {
            return None;
        }

        let row = &result[0];

        let from: i64 = row.get(0);
        let to: i64 = row.get(1);
        let started_at: Timestamp = row.get(2);

        Some(StartedTask {
            from: from as u64,
            to: to as u64,
            started_at,
        })
    }

    fn delete_queued_task_by_client(&mut self, client_name: ClientName) {
        self.client
            .execute("DELETE FROM queue WHERE client_name = $1", &[&client_name])
            .unwrap();
    }

    fn update_from(&mut self, from: u64) {
        let from = from as i64;
        self.client
            .execute("UPDATE state SET from_ = $1", &[&from])
            .unwrap();
    }

    fn fetch_results_by_client(&mut self, client_name: ClientName) -> Vec<ThreeNResult> {
        let result = self
            .client
            .query(
                "SELECT result FROM results WHERE client_name = $1",
                &[&client_name],
            )
            .unwrap();
        result.iter().map(|row| row.get(0)).collect()
    }

    fn store_results(&mut self, client_name: ClientName, task: StartedTask, result: ThreeNResult) {
        self.client
            .execute(
                "INSERT INTO results(task_from, task_to, result, client_name, started_at) VALUES ($1, $2, $3, $4, $5)",
                &[&(task.from as i64), &(task.to as i64), &result, &client_name, &task.started_at],
            )
            .unwrap();
    }
}
