use std::thread::{available_parallelism, scope};

/// Distributes `joblist` round-robin over the available cores, runs `f` once per thread on its
/// share of the jobs, and returns each thread's result (in no particular order). Threads that
/// would get no jobs are not spawned.
pub fn spawn_jobs<F, T, U>(joblist: Vec<U>, f: F) -> Vec<T>
where
    F: Fn(Vec<U>) -> T + Sync,
    T: Send,
    U: Send,
{
    let capacity = available_parallelism().map_or(1, |n| n.get());
    let mut job_groups: Vec<Vec<U>> = (0..capacity).map(|_| Vec::new()).collect();
    for (i, job) in joblist.into_iter().enumerate() {
        job_groups[i % capacity].push(job);
    }

    let f = &f;

    scope(|s| {
        let handles: Vec<_> = job_groups
            .into_iter()
            .filter(|group| !group.is_empty())
            .map(|group| s.spawn(move || f(group)))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("worker thread panicked"))
            .collect()
    })
}
