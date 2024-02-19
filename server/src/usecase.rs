use crate::domain::{ClientName, FromTo, Repository, ThreeNResponse, ThreeNResult};

pub fn handle_ready<R: Repository>(repo: &mut R, client_name: String) -> ThreeNResponse {
    let state = repo.fetch_current_state();

    let from = state.from;
    let to = state.from + state.increment - 1;

    repo.update_from(state.from + state.increment);
    repo.add_to_queue(FromTo { from, to }, client_name);

    ThreeNResponse::Solve { from, to }
}

pub fn handle_solved<R: Repository>(
    repo: &mut R,
    client_name: ClientName,
    from: String,
    to: String,
    result: ThreeNResult,
) -> ThreeNResponse {
    repo.store_results(
        client_name,
        from.parse().unwrap(),
        to.parse().unwrap(),
        result,
    );
    ThreeNResponse::Ok
}
