use std::{
    sync::Mutex,
    thread::{self},
};

pub enum MaybeAsyncExecution<'scope, T> {
    Async(thread::ScopedJoinHandle<'scope, T>),
    Sync(T),
}

impl<'scope, T> MaybeAsyncExecution<'scope, T> {
    pub fn into_inner(self) -> T {
        match self {
            MaybeAsyncExecution::Async(join_handle) => join_handle.join().unwrap(),
            MaybeAsyncExecution::Sync(v) => v,
        }
    }
}

// Limits the amount of concurrent spawned threads to a given value, not including the main thread.
//
// Threads spawned using the scope directly are not tracked for the purposes of concurrency
// limiting.
pub struct ConcurrencyLimiter {
    available_concurrency: Mutex<usize>,
}

impl ConcurrencyLimiter {
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            available_concurrency: Mutex::new(max_concurrency),
        }
    }

    // If there is available concurrency, runs the job, returning Ok(join handle).
    //
    // Otherwise runs the function directly, returning Err(function result).
    pub fn run_scoped<'env, 'scope, F, T>(
        &'env self,
        scope: &'scope thread::Scope<'scope, 'env>,
        f: F,
    ) -> MaybeAsyncExecution<'scope, T>
    where
        F: FnOnce() -> T + Send + 'scope,
        T: Send + 'scope,
    {
        let do_async = {
            let mut lock = self.available_concurrency.lock().unwrap();
            if *lock == 0 {
                false
            } else {
                *lock -= 1;
                true
            }
        };

        if do_async {
            let f = || {
                let ret = f();
                let mut lock = self.available_concurrency.lock().unwrap();
                *lock += 1;
                ret
            };
            MaybeAsyncExecution::Async(scope.spawn(f))
        } else {
            MaybeAsyncExecution::Sync(f())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct ConcurrencyCounter {
        concurrency: Mutex<usize>,
        max: usize,
    }

    impl ConcurrencyCounter {
        pub fn new(max: usize) -> Self {
            Self {
                concurrency: Mutex::new(0),
                max,
            }
        }

        pub fn inc(&self) -> bool {
            let mut lock = self.concurrency.lock().unwrap();
            if *lock == self.max {
                false
            } else {
                *lock += 1;
                true
            }
        }

        pub fn dec(&self) {
            let mut lock = self.concurrency.lock().unwrap();
            assert!(*lock > 0);
            *lock -= 1;
        }
    }

    #[test]
    fn test_concurrency_limiter() {
        const MAX_SPAWNED: usize = 3;
        const NUM_THREADS: usize = 1000;
        // Add 1 for the current thread.
        let counter = ConcurrencyCounter::new(MAX_SPAWNED + 1);
        let number = AtomicUsize::new(0);
        let concurrency_limiter = ConcurrencyLimiter::new(MAX_SPAWNED);

        thread::scope(|s| {
            for _ in 0..NUM_THREADS {
                concurrency_limiter.run_scoped(s, || {
                    assert!(counter.inc());
                    number.fetch_add(1, Ordering::Relaxed);
                    while number.load(Ordering::Relaxed) < MAX_SPAWNED + 1 {}
                    counter.dec();
                });
            }
        });

        assert_eq!(number.load(Ordering::SeqCst), NUM_THREADS);
    }

    #[test]
    fn test_concurrency_limiter_runs_sync_with_zero_concurrency() {
        let concurrency_limiter = ConcurrencyLimiter::new(0);

        thread::scope(|s| {
            match concurrency_limiter.run_scoped(s, || {}) {
                MaybeAsyncExecution::Async(_) => panic!("Should not have run async"),
                MaybeAsyncExecution::Sync(_) => {}
            };
        });
    }
}
