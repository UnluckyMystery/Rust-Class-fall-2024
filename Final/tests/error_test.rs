use std::time::Duration;
use ureq;

#[test]
fn test_check_website_timeout() {
    
    let result = ureq::get("http://localhost:8000")
        .timeout(Duration::from_secs(1))  
        .call();

    match result {
        Ok(response) => {

            assert_eq!(response.status(), 200);
        },
        Err(err) => {

            assert!(err.to_string().contains("Connection refused"));
        },
    }
}

#[test]
fn test_check_website_with_http_error() {

    let result = ureq::get("http://localhost:8000/nonexistent")
        .timeout(Duration::from_secs(2))  
        .call();

    match result {
        Ok(response) => {
           
            assert_eq!(response.status(), 404);  
        },
        Err(err) => {
        
            assert!(err.to_string().contains("Connection refused"));
        },
    }
}
