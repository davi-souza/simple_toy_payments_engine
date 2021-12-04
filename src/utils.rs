use crate::entities::Account;

pub fn print_result(accounts: Vec<Account>) {
    println!("id,available,held,total,locked");
    for account in accounts {
        println!("{}", account);
    }
}
