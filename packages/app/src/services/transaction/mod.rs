use entity::transactions;

use crate::impl_service;

impl_service!(TransactionService, transactions::Entity);
