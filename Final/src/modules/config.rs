use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub num_threads: usize,
    pub request_timeout: Duration,
    pub max_retries: usize, // Maximum retries per website
}

impl Config {
    pub fn load() -> Self {
        Config {
            num_threads: 50,              
            request_timeout: Duration::from_secs(5), 
            max_retries: 3,               
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; 
    use std::time::Duration;

    #[test]
    fn test_load_config() {
        
        let config = Config::load();

        
        assert_eq!(config.num_threads, 50); 
        assert_eq!(config.request_timeout, Duration::from_secs(5));
        assert_eq!(config.max_retries, 3);
    }
}
