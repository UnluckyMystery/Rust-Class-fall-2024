

pub struct Stats {
    total_checks: u32,
    successful_checks: u32,
    failed_checks: u32,
    total_response_time: u32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total_checks: 0,
            successful_checks: 0,
            failed_checks: 0,
            total_response_time: 0,
        }
    }


    pub fn record_check(&mut self, response_time: u32, success: bool) {
        self.total_checks += 1;
        self.total_response_time += response_time;
        if success {
            self.successful_checks += 1;
        } else {
            self.failed_checks += 1;
        }
    }

    pub fn calculate_uptime(&self) -> f32 {
        if self.total_checks == 0 {
            0.0
        } else {
            (self.successful_checks as f32 / self.total_checks as f32) * 100.0
        }
    }

    pub fn calculate_average_response_time(&self) -> f32 {
        if self.total_checks == 0 {
            0.0
        } else {
            self.total_response_time as f32 / self.total_checks as f32
        }
    }


    pub fn display_stats(&self) {
        println!("Total checks: {}", self.total_checks);
        println!("Successful checks: {}", self.successful_checks);
        println!("Failed checks: {}", self.failed_checks);
        println!("Average response time: {:.2} ms", self.calculate_average_response_time());
        println!("Uptime percentage: {:.2}%", self.calculate_uptime());
    }
}
