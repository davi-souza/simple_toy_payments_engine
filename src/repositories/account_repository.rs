use crate::entities::Account;

#[derive(Debug, Clone)]
pub struct AccountRepository {
    accounts: Vec<Account>,
}

impl AccountRepository {
    pub fn new() -> Self {
        Self { accounts: vec![] }
    }

    pub fn get_all(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    pub fn find_by_id(&self, id: u16) -> Option<Account> {
        self.accounts.iter().find(|acc| acc.id == id).copied()
    }

    pub fn upsert(&mut self, account: Account) {
        match self.accounts.iter().position(|acc| acc.id == account.id) {
            Some(index) => {
                self.update(index, account);
            }
            None => {
                self.insert(account);
            }
        };
    }

    pub fn update(&mut self, index: usize, account: Account) {
        self.accounts.splice(index..index + 1, [account]);
    }

    pub fn insert(&mut self, account: Account) {
        self.accounts.push(account);
    }
}
