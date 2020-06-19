pub struct ThreadPool;

pub struct PoolCreationError {
    message: String,
}

impl ThreadPool {
    // Create a new ThreadPool
    //
    // `size` is the number of threads in the pool.
    //
    // # Panics
    //
    // `new` will panic if the size is 0.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool)
        } else {
            Err(PoolCreationError { message: "size must be greater than 0".to_string() })
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
