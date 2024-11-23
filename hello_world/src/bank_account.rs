#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
        // Implement this method
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(30.0);
        assert_eq!(account.balance(), 70.0);
    }

    // Add more tests here

    fn test_depo_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);  // Should not change the balance
        assert_eq!(account.balance(), 100.0);
    }

    fn test_with_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-30.0);  // Should not change the balance
        assert_eq!(account.balance(), 100.0);
    }


    fn test_edge_floating_point() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(99.999999999);  // Near the edge of precision
        assert!((account.balance() - 0.000000001).abs() < 1e-10);  // Assert with epsilon for floating point
    }
}
