use crate::impl_service;
pub mod assign_permissions;
pub mod login;
pub mod logout;
pub mod query_permissions;
pub mod register;

impl_service!(AuthService);
