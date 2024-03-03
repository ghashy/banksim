use secrecy::Secret;
use serde::Deserialize;

use crate::domain::card_number::CardNumber;

#[derive(Deserialize)]
pub struct AddAccountRequest {
    pub password: Secret<String>,
}

#[derive(Deserialize)]
pub struct DeleteAccountRequest {
    pub card_number: CardNumber,
}

#[derive(Deserialize)]
pub struct OpenCreditRequest {
    pub card_number: CardNumber,
    pub amount: i64,
}

#[derive(Deserialize)]
pub struct NewTransactionRequest {
    pub from: CardNumber,
    pub to: CardNumber,
    pub amount: i64,
}
