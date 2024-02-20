# Distributed computing

The idea is to try to search the problem space of mathematical problems. We used the famous `3n+1` using multiple clients, but the server is almost problem-agnostic, we only need to tweak the `state` table.

## How it works

A task is a `from`, `to` pair. The problem is implicit right now, but it could be extended in the future.

### Clients

Clients work like:

```
loop {
    fetch_problem
    solve_problem
    report_result
}
```

### Server

The `state` table defines three fields: `from`, `to` and `step`.

The `queue` table tells us which tasks are currently solved and by whom.

The `results` table stores the results of the solved tasks.

Each client can be assigned only one task at a time (we check the queue for that).

## What has been done so far

- We store the results of each interaction in the database
- We have a simple server and a few clients (python and js, the Rust client is not done yet)
- We have a simple CI/CD with github actions

## What could be done next

- Implement a frontend and an API to visualize the progress of computation
- Benchmark the performance of each client, so we can adjust the size of the task and make it finish in, say, 10 minuts each.
