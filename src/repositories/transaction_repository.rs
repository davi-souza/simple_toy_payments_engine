use crate::entities::Transaction;
use std::collections::HashMap;

#[derive(Debug)]
struct DisputedTransaction {
    transaction: Transaction,
    open: bool,
}

#[derive(Debug)]
pub struct TransactionRepository {
    transactions: Vec<Transaction>,
    disputed_transaction: HashMap<u32, DisputedTransaction>,
}

impl TransactionRepository {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
            disputed_transaction: HashMap::new(),
        }
    }
    pub fn find_by_id(&self, transaction_id: u32) -> Option<Transaction> {
        self.transactions
            .iter()
            .find(|tx| tx.id == transaction_id)
            .copied()
    }

    pub fn is_transaction_of_account_id(&self, transaction_id: u32, account_id: u16) -> bool {
        match self.find_by_id(transaction_id) {
            None => false,
            Some(tx) => tx.account_id == account_id,
        }
    }

    pub fn is_transaction_disputable(&self, transaction_id: u32) -> bool {
        self.disputed_transaction.get(&transaction_id).is_none()
    }

    pub fn is_transaction_dispute_endable(&self, transaction_id: u32) -> bool {
        self.disputed_transaction
            .get(&transaction_id)
            .map(|disputed_transaction| disputed_transaction.open)
            .unwrap_or(false)
    }

    pub fn insert(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn insert_disputed_transaction(&mut self, transaction: Transaction) {
        self.disputed_transaction.insert(
            transaction.id,
            DisputedTransaction {
                transaction,
                open: true,
            },
        );
    }

    pub fn end_disputed_transaction(&mut self, transaction: Transaction) {
        if let Some(disputed_transaction) = self.disputed_transaction.get_mut(&transaction.id) {
            (*disputed_transaction).open = false;
        }
    }
}
