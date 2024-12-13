mod modules;

use modules::checker::{check_website, validate_http_headers, validate_ssl_certificate, validate_response_body, current_utc_timestamp, WebsiteStatus};
use modules::config::Config;
use modules::stats::Stats;
use std::{thread, sync::{mpsc, Arc, Mutex}, time::Duration};

fn main() {
    let config = Config::load();
    let urls = vec![
        "http://www.wikipedia.org".to_string(),
        "http://httpbin.org/status/500".to_string(),
        "http://example.com".to_string(),
        "http://nonexistent.com".to_string(),
    ];

    let (tx, _rx) = mpsc::channel();
    let urls = Arc::new(Mutex::new(urls)); 
    let num_threads = config.num_threads;

    let statistics = Arc::new(Mutex::new(Stats::new())); 
    let mut handles = vec![];


    let retries = 3; 
    let timeout = Duration::from_secs(5); 

    for _ in 0..num_threads {
        let tx_clone = tx.clone();
        let urls_clone = Arc::clone(&urls);
        let statistics_clone = Arc::clone(&statistics);

        let handle = thread::spawn(move || {
            while let Some(url) = {
                let mut urls_guard = urls_clone.lock().unwrap();
                urls_guard.pop()
            } {
                let start = std::time::Instant::now();

             
                match check_website(&url, retries, timeout) {
                    Ok(response_time) => {
                        let mut stats = statistics_clone.lock().unwrap();
                        stats.record_check(response_time.as_millis() as u32, true);
                        println!("Website check successful for {}!", url);
                    }
                    Err(e) => {
                        println!("Failed to check website: {}", e);
                        let mut stats = statistics_clone.lock().unwrap();
                        stats.record_check(0, false);
                    }
                }

          
                if let Err(e) = validate_http_headers(&url) {
                    println!("HTTP Header Validation failed: {}", e);
                }

               
                if let Err(e) = validate_ssl_certificate(&url) {
                    println!("SSL Certificate Validation failed: {}", e);
                }

             
                if let Err(e) = validate_response_body(&url, "Welcome") {
                    println!("Response Body Validation failed: {}", e);
                }

                
                let duration = start.elapsed();
                let status = WebsiteStatus {
                    url,
                    status: "checked".to_string(),
                    response_time: duration,
                    timestamp: current_utc_timestamp(),
                };

                tx_clone.send(status).unwrap();
            }
        });

        handles.push(handle);
    }

    
    for handle in handles {
        handle.join().unwrap();
    }


    {
        let stats = statistics.lock().unwrap();
        println!("Uptime: {}%", stats.calculate_uptime());
        println!("Average response time: {} ms", stats.calculate_average_response_time());
    }
}
