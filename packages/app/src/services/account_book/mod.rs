use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_account_book;
pub mod delete_account_books;
pub mod query_account_books;
pub mod update_account_book;

impl_service!(AccountBookService, DatabaseConnection);
