use std::{time::{Duration, Instant, UNIX_EPOCH}, thread};
use ureq;



pub fn check_website(url: &str, retries: usize, timeout: Duration) -> Result<Duration, String> {
    let mut attempt = 0;
    let start = Instant::now();

    while attempt < retries {
        match ureq::get(url).timeout(timeout).call() {
            Ok(_) => {
                let duration = start.elapsed();
                return Ok(duration); // Return the response time if successful
            }
            Err(e) => {
                attempt += 1;
                if attempt < retries {
                    println!("Attempt {} failed for {}. Retrying... ({})", attempt, url, e);
                    thread::sleep(Duration::from_secs(1)); // Wait before retrying
                } else {
                    return Err(format!("Failed to fetch {} after {} retries: {}", url, retries, e));
                }
            }
        }
    }

    Err("All retries exhausted".to_string()) // If all retries fail
}

// Example placeholder for header validation (can be expanded)
pub fn validate_http_headers(url: &str) -> Result<(), String> {
    // Simulate HTTP Header validation logic
    println!("Validating HTTP headers for {}", url);
    Ok(())
}

// Example placeholder for SSL validation (can be expanded)
pub fn validate_ssl_certificate(url: &str) -> Result<(), String> {
    // Simulate SSL certificate validation logic
    if url.starts_with("http://") {
        Err(format!("SSL Certificate Validation failed: The URL does not use HTTPS."))
    } else {
        Ok(())
    }
}

// Example placeholder for body validation (can be expanded)
pub fn validate_response_body(url: &str, expected_content: &str) -> Result<(), String> {
    // Simulate body validation logic
    println!("Validating response body for {}", url);
    if !url.contains(expected_content) {
        Err(format!("Response Body Validation failed: Response body does not contain expected content: {}", expected_content))
    } else {
        Ok(())
    }
}

pub fn current_utc_timestamp() -> String {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");  // Expect the duration_since call to succeed
    let timestamp = since_the_epoch.as_secs();
    format!("{} UTC", timestamp)
}

// Struct to represent the website status
pub struct WebsiteStatus {
    pub url: String,
    pub status: String,
    pub response_time: Duration,
    pub timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from the parent module
    use std::time::Duration;

    #[test]
    fn test_check_website_success() {
        // Simulating a successful HTTP request to a real URL (httpbin is used for testing)
        let url = "http://httpbin.org/status/200"; // This URL should return status 200
        let timeout = Duration::from_secs(5);
        let max_retries = 1;

        // Call the function
        let result = check_website(url, max_retries, timeout);  // Correct argument order

        // Assert that the result is Ok, meaning the status was 200
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_website_failure() {
        // Simulate a request that will fail due to DNS issues
        let url = "http://nonexistent.invalid"; // This URL will not resolve (DNS error)
        let timeout = Duration::from_secs(1);
        let max_retries = 1;

        // Call the function
        let result = check_website(url, max_retries, timeout);  // Correct argument order

        // Assert that the result is Err, meaning there was an error
        assert!(result.is_err());
    }

    #[test]
    fn test_check_website_http_error() {
       
        let url = "http://httpbin.org/status/500"; 
        let timeout = Duration::from_secs(5);
        let max_retries = 1;

       
        let result = check_website(url, max_retries, timeout);  

        
        assert!(result.is_err());
    }

    #[test]
    fn test_current_utc_timestamp() {
        // Get the current UTC timestamp
        let timestamp = current_utc_timestamp();

        // Check if the timestamp string contains "UTC"
        assert!(timestamp.ends_with(" UTC"));
    }
}
