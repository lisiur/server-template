#[macro_export]
macro_rules! impl_service {
    ($struct_name:ident) => {
        pub struct $struct_name {
            app: crate::App,
            conn: sea_orm::DatabaseConnection,
        }

        impl $struct_name {
            pub fn new(app: crate::App) -> Self {
                let conn = app.conn.clone();
                Self { app, conn }
            }
        }

        impl From<crate::App> for $struct_name {
            fn from(app: crate::App) -> Self {
                Self::new(app)
            }
        }
    };

    ($struct_name:ident, $entity:ty) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            app: crate::App,
            #[allow(dead_code)]
            conn: sea_orm::DatabaseConnection,
            #[allow(dead_code)]
            pub(crate) crud: crate::services::crud::Crud<$entity>,
        }

        impl $struct_name {
            pub fn new(app: crate::App) -> Self {
                let conn = app.conn.clone();
                Self {
                    app,
                    conn: conn.clone(),
                    crud: crate::services::crud::Crud::new(conn),
                }
            }
        }

        impl From<crate::App> for $struct_name {
            fn from(app: crate::App) -> Self {
                Self::new(app)
            }
        }
    };
}
