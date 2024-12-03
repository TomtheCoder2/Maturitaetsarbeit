use rayon::prelude::*;
use std::ptr;
use std::sync::{Arc, Mutex};

// Define a new type that wraps the raw pointer
struct SafePtr(*mut u32);

// Implement Send and Sync for SafePtr
unsafe impl Send for SafePtr {}
unsafe impl Sync for SafePtr {}

fn main() {
    let mut list = vec![0u32; 100];

    // Get a mutable raw pointer to the list
    let ptr = list.as_mut_ptr();
    let safe_ptr = SafePtr(ptr);
    let safe_ptr = Arc::new(Mutex::new(safe_ptr));

    // Parallel iteration
    (0..100u32).into_par_iter().for_each(|x| {
        let safe_ptr = Arc::clone(&safe_ptr);
        unsafe {
            // Lock the mutex to get access to the raw pointer
            let safe_ptr = safe_ptr.lock().unwrap();
            // Use the raw pointer to write directly to the list at index x
            safe_ptr.0.add(x as usize).write(x);
        }
    });

    // Print the list (this is safe after parallel writes are complete)
    println!("{:?}", list);
}
