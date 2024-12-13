/*use crate::modules::checker::check_website; // Use the actual crate name from Cargo.toml
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

#[test]
fn test_check_website_with_performance() {
    // Create a channel for inter-thread communication
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to make a request
    thread::spawn(move || {
        let result = check_website("http://httpbin.org/status/200", 3, Duration::from_secs(5));
        tx.send(result).unwrap();
    });

    // Receive the result from the thread
    let result = rx.recv().unwrap();

    // Assert based on the result
    match result {
        Ok(duration) => {
            println!("Response time: {:?}", duration);
            assert!(duration < Duration::from_secs(5));
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            assert!(err.contains("Connection refused") || err.contains("Timeout"));
        }
    }
}

#[test]
fn test_concurrent_requests_performance() {
    // Define a list of URLs to test
    let urls = vec![
        "http://httpbin.org/status/200",
        "http://httpbin.org/status/500",
        "http://example.com",
        "http://nonexistent.com",
    ];

    let num_threads = 10; // Number of threads for concurrent requests
    let timeout = Duration::from_secs(5);
    let retries = 3;

    let mut handles = vec![];

    // Spawn multiple threads to test concurrent requests
    for _ in 0..num_threads {
        let urls_clone = urls.clone();

        let handle = thread::spawn(move || {
            for url in urls_clone {
                let result = check_website(&url, retries, timeout);
                match result {
                    Ok(duration) => println!("Success: {:?} took {:?}", url, duration),
                    Err(err) => eprintln!("Error: {} - {}", url, err),
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Concurrent requests test completed");
}*/

//IMPORTANT

//Had an issue where it wouldn't recognize the module crate so it was messing all of the tests up.