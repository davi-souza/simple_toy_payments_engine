use crate::entities::{Account, Transaction, TransactionType};
use crate::repositories::{AccountRepository, TransactionRepository};

#[derive(Debug)]
pub struct State {
    account_repository: AccountRepository,
    transaction_repository: TransactionRepository,
}

impl State {
    pub fn new() -> Self {
        Self {
            account_repository: AccountRepository::new(),
            transaction_repository: TransactionRepository::new(),
        }
    }

    pub fn get_all_accounts(&self) -> Vec<Account> {
        self.account_repository.get_all()
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        let mut account = self
            .account_repository
            .find_by_id(transaction.account_id)
            .unwrap_or_else(|| Account::new(transaction.account_id));
        if account.locked {
            return;
        }
        self.update_account(&mut account, transaction);
        self.account_repository.upsert(account);
        self.transaction_repository.insert(transaction);
    }

    fn update_account(&mut self, account: &mut Account, transaction: Transaction) {
        match transaction.r#type {
            TransactionType::Deposit => {
                account.add_available_by(transaction.amount());
                account.add_total_by(transaction.amount());
            }
            TransactionType::Withdrawal if transaction.amount() <= account.available => {
                account.add_available_by(-transaction.amount());
                account.add_total_by(-transaction.amount());
            }
            t @ TransactionType::Dispute
            | t @ TransactionType::Resolve
            | t @ TransactionType::Chargeback => {
                if !self
                    .transaction_repository
                    .is_transaction_of_account_id(transaction.id, account.id)
                {
                    return;
                }
                let transaction = self
                    .transaction_repository
                    .find_by_id(transaction.id)
                    .unwrap();
                match t {
                    TransactionType::Dispute
                        if self
                            .transaction_repository
                            .is_transaction_disputable(transaction.id) =>
                    {
                        self.update_account_from_dispute_transaction(account, transaction)
                    }
                    TransactionType::Resolve
                        if self
                            .transaction_repository
                            .is_transaction_dispute_endable(transaction.id) =>
                    {
                        self.update_account_from_resolve_transaction(account, transaction)
                    }
                    TransactionType::Chargeback
                        if self
                            .transaction_repository
                            .is_transaction_dispute_endable(transaction.id) =>
                    {
                        self.update_account_from_chargeback_transaction(account, transaction)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn update_account_from_dispute_transaction(
        &mut self,
        account: &mut Account,
        transaction: Transaction,
    ) {
        self.transaction_repository
            .insert_disputed_transaction(transaction);
        account.add_available_by(-transaction.get_dispute_delta());
        account.add_held_by(transaction.get_dispute_delta());
    }

    fn update_account_from_resolve_transaction(
        &mut self,
        account: &mut Account,
        transaction: Transaction,
    ) {
        self.transaction_repository
            .end_disputed_transaction(transaction);
        account.add_available_by(transaction.get_dispute_delta());
        account.add_held_by(-transaction.get_dispute_delta());
    }

    fn update_account_from_chargeback_transaction(
        &mut self,
        account: &mut Account,
        transaction: Transaction,
    ) {
        self.transaction_repository
            .end_disputed_transaction(transaction);
        account.add_held_by(-transaction.get_dispute_delta());
        account.add_total_by(-transaction.get_dispute_delta());
        account.set_locked(true);
    }
}
