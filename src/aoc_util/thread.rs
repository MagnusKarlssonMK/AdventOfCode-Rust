use std::thread::*;

pub fn spawn_jobs<F, T, U>(mut joblist: Vec<U>, f: F)
where
    F: FnOnce(Vec<U>) -> T + Copy + Send,
    T: Send,
    U: Send,
{
    let capacity = available_parallelism().unwrap().get();
    let mut job_groups: Vec<_> = (0..capacity).map(|_| Vec::new()).collect();

    let mut i = 0;
    while let Some(job) = joblist.pop() {
        job_groups[i % capacity].push(job);
        i += 1;
    }

    scope(|s| {
        for group in job_groups {
            s.spawn(move || f(group));
        }
    });
}
