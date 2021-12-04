use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Transaction {
    pub r#type: TransactionType,
    #[serde(rename = "client")]
    pub account_id: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    amount: Option<f32>,
}

impl Transaction {
    pub fn amount(&self) -> f32 {
        self.amount.unwrap_or(0.0)
    }

    pub fn get_dispute_delta(&self) -> f32 {
        match self.r#type {
            TransactionType::Withdrawal => -self.amount(),
            _ => self.amount(),
        }
    }
}
