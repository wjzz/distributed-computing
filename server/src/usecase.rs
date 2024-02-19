use crate::domain::{ClientName, Repository, Task, ThreeNResult};

fn allocate_new_task<R: Repository>(repo: &mut R) -> Task {
    let state = repo.fetch_current_state();
    let from = state.from;
    let to = state.from + state.increment - 1;

    repo.update_from(state.from + state.increment);
    Task { from, to }
}

pub fn handle_ready<R: Repository>(repo: &mut R, client_name: String) -> Task {
    // check if this client has an existing tasks, if so return it again
    // otherwise, allocate new task

    let task = allocate_new_task(repo);
    repo.add_to_queue(task, client_name);

    task
}

pub fn handle_solved<R: Repository>(
    repo: &mut R,
    client_name: ClientName,
    from: String,
    to: String,
    result: ThreeNResult,
) -> () {
    repo.store_results(
        client_name,
        Task {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        },
        result,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inmemory_repository::*;

    fn client_name() -> ClientName {
        "client1".to_string()
    }

    #[test]
    fn after_ready_gets_queued() {
        let mut repo = InMemoryRepository::new();
        let from_to = handle_ready(&mut repo, client_name());

        assert_eq!(from_to.from, 1);
        assert_eq!(from_to.to, 1_000_000);
    }

    #[test]
    fn double_ready_is_idempotent() {
        // we wait until the client solves the original problem
        let mut repo = InMemoryRepository::new();
        let from_to1 = handle_ready(&mut repo, client_name());
        let from_to2 = handle_ready(&mut repo, client_name());

        assert_eq!(from_to1, from_to2);
    }
}
