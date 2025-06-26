#[macro_export]
macro_rules! impl_service {
    ($struct_name:ident, $conn_type:ty) => {
        pub struct $struct_name {
            conn: $conn_type,
        }

        impl $struct_name {
            pub fn new(conn: $conn_type) -> Self {
                Self { conn }
            }
        }

        impl From<$conn_type> for $struct_name {
            fn from(conn: $conn_type) -> Self {
                Self::new(conn)
            }
        }
    };

    ($struct_name:ident, $conn_type:ty, $entity:ty) => {
        pub struct $struct_name {
            conn: $conn_type,
            #[allow(dead_code)]
            crud: crate::services::crud::Crud<$entity>,
        }

        impl $struct_name {
            pub fn new(conn: $conn_type) -> Self {
                Self {
                    conn: conn.clone(),
                    crud: crate::services::crud::Crud::new(conn),
                }
            }
        }

        impl From<$conn_type> for $struct_name {
            fn from(conn: $conn_type) -> Self {
                Self::new(conn)
            }
        }
    };
}
