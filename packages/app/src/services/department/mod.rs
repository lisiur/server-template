use entity::departments;

use crate::impl_service;
pub mod create_department;
pub mod delete_departments;
pub mod query_departments;
pub mod update_department;

impl_service!(DepartmentService, departments::Entity);
