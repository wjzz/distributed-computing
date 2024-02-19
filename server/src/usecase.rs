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
    if let Some(task) = repo.fetch_queued_task_by_client(client_name.clone()) {
        return task;
    }

    let task = allocate_new_task(repo);
    repo.add_to_queue(task, client_name);

    task
}

pub fn handle_solved<R: Repository>(
    repo: &mut R,
    client_name: ClientName,
    task: Task,
    result: ThreeNResult,
) -> () {
    if repo
        .fetch_queued_task_by_client(client_name.clone())
        .is_none()
    {
        eprintln!("Unknown problem");
        return;
    }

    repo.delete_queued_task_by_client(client_name.clone());
    repo.store_results(client_name, task, result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inmemory_repository::*;

    fn client_name() -> ClientName {
        "client1".to_string()
    }

    fn result() -> ThreeNResult {
        "example_result".to_string()
    }

    #[test]
    fn after_ready_gets_queued() {
        let mut repo = InMemoryRepository::new();
        let task = handle_ready(&mut repo, client_name());

        assert_eq!(
            task,
            Task {
                from: 1,
                to: 1_000_000
            }
        );
    }

    #[test]
    fn double_ready_is_idempotent() {
        // we wait until the client solves the original problem
        let mut repo = InMemoryRepository::new();
        let task1 = handle_ready(&mut repo, client_name());
        let task2 = handle_ready(&mut repo, client_name());

        assert_eq!(task1, task2);
    }

    #[test]
    fn solving_deletes_the_queue() {
        let mut repo = InMemoryRepository::new();
        let task = handle_ready(&mut repo, client_name());
        handle_solved(&mut repo, client_name(), task, result());

        let task_queued_for_client = repo.fetch_queued_task_by_client(client_name());

        assert!(task_queued_for_client.is_none());
    }

    #[test]
    fn solving_appends_results() {
        let mut repo = InMemoryRepository::new();
        let task = handle_ready(&mut repo, client_name());
        handle_solved(&mut repo, client_name(), task, result());

        let solved = repo.fetch_results_by_client(client_name());

        assert_eq!(solved, vec![result()]);
    }

    #[test]
    fn after_ready_after_solve_works() {
        let mut repo = InMemoryRepository::new();
        let task = handle_ready(&mut repo, client_name());
        handle_solved(&mut repo, client_name(), task, result());

        let task2 = handle_ready(&mut repo, client_name());

        assert_eq!(
            task2,
            Task {
                from: 1_000_001,
                to: 2_000_000
            }
        );
    }
}
