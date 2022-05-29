pub struct Account {
    pub name: String,
    pub money: f32,
    transactions: Vec<Transaction>,
}

struct Transaction {
    pub transaction_type: String,
    pub amount: f32,
}

impl Account {
    pub fn new(name: String, money: f32) -> Account {
        let account = Account {
            name,
            money,
            transactions: vec![],
        };
        println!(
            "Hello {}, you have {}",
            account.name,
            account.get_money_as_string()
        );
        return account;
    }

    fn modify(&mut self, amount: f32) {
        self.money += amount;
    }


    pub fn deposit(&mut self, amount: f32) {
        self.modify(amount);
        self.transactions.push(Transaction {
            transaction_type: "Deposited".to_string(),
            amount,
        });
        println!(
            "You have deposited {}. Your balance is now {}",
            amount,
            self.get_money_as_string()
        );
    }

    pub fn withdraw(&mut self, amount: f32) {
        self.modify(amount * -1f32);
        if self.money < amount {
            println!("You cannot withdraw more than you have!");
            self.add_transaction(Transaction {transaction_type: "Withdraw (failed) for".to_string(), amount: amount * -1f32 });
        } else {
            self.add_transaction(Transaction { transaction_type: "Withdrew".to_string(), amount: amount * -1f32 });
            println!(
                "You have withdrawn {}. Your balance is now {}",
                amount,
                self.get_money_as_string()
            );
        }
    }
    pub fn get_money_as_string(&self) -> String {
        format!("${:.2}", self.money)
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn print_transaction(&self) {
        for transaction in self.transactions.iter() {
            println!("{} {}", transaction.transaction_type, format!("${:.2}", transaction.amount));
        }
    }
}

