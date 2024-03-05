use serde::Serialize;
use std::collections::HashMap;

use crate::{bank::Transaction, domain::card_number::CardNumber};

#[derive(Serialize)]
pub struct AddAccountResponse {
    pub card_number: CardNumber,
}

#[derive(Debug, Serialize)]
pub struct Account {
    pub card_number: CardNumber,
    pub balance: i64,
    pub transactions: Vec<Transaction>,
    pub exists: bool,
    pub tokens: Vec<String>,
    pub username: String,
}

#[derive(Serialize)]
pub struct ListAccountsResponse {
    pub accounts: Vec<Account>,
}

#[derive(Serialize)]
pub struct ListCardTokensResponse {
    pub list: HashMap<String, CardNumber>,
}
