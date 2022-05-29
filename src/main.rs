#[path = "./bank.rs"]
mod bank;
use bank::Account;
use text_io::read;

fn main() {
    println!("Welcome to rustBank'! What is your name?");
    let name: String = read!();
    println!("How much money would you like to start with?");
    let money: f32 = read!();
    let mut account = Account::new(name, money);
    loop {
        println!("What would you like to do? Type help to see a list of commands");
        let choice: String = read!();
        match choice.as_str() {
            "1" => {
                println!("How much would you like to deposit?");
                let deposit: f32 = read!();
                account.deposit(deposit);
            }
            "2" => {
                println!("How much would you like to withdraw?");
                let withdraw: f32 = read!();
                account.withdraw(withdraw);
            }
            "3" => {
                println!("Your balance is {}", account.get_money_as_string());
            }
            "4" => {
                account.print_transaction();
            }
            "5" => {
                println!("Thank you for using bankUncodeable!");
                println!("{}'s final balance: {}", account.name, account.get_money_as_string());
                break;
            }
            _ => {
                println!("1: Deposit\n2: Withdraw\n3: Check Balance\n4: View Transactions\n5: Exit");
            }
        }
    }
}
