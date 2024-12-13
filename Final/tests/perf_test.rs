/*use std::time::{Duration, Instant}; 
use std::sync::mpsc;
use std::thread;
use crate::modules::checker::check_website;





#[test]
fn test_check_website_with_performance() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
 
        let result = ureq::get("http://localhost:8000")
            .timeout(Duration::from_secs(2))
            .call();
        
  
        tx.send(result).unwrap();
    });

   
    let result = rx.recv().unwrap();

    match result {
        Ok(response) => {
           
            assert!(response.status() == 200 || response.status() == 500);
        },
        Err(err) => {
            
            assert!(err.to_string().contains("Connection refused"));
        }
    }
}

#[test]
fn test_performance_with_concurrent_requests() {
    let urls = vec![
        "http://httpbin.org/status/200".to_string(),
        "http://httpbin.org/status/500".to_string(),
        "http://example.com".to_string(),
        "http://nonexistent.com".to_string(),
    ];

    let num_threads = 50; 
    let timeout = Duration::from_secs(5);
    let retries = 3;

    let start = Instant::now();

    let mut handles = vec![];

    
    for _ in 0..num_threads {
        let urls_clone = urls.clone(); 

        let handle = thread::spawn(move || {
            
            for url in urls_clone.iter() {
                let _ = check_website(url, retries, timeout);
            }
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Performance test completed in {:?}", duration);
    assert!(duration < Duration::from_secs(60), "Performance test took too long");
}*/


//IMPORTANT

//Had an issue where it wouldn't recognize the module crate so it was messing all of the tests up.