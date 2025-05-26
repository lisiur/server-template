pub fn create_auto_updated_at_trigger(table_name: &str) -> String {
    println!("{}", table_name);
    format!(
        r#"
        CREATE OR REPLACE FUNCTION {table_name}_auto_updated_at()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;

        CREATE TRIGGER {table_name}_auto_updated_at
        BEFORE UPDATE ON {table_name}
        FOR EACH ROW
        EXECUTE FUNCTION {table_name}_auto_updated_at();
        "#,
    )
}

pub fn drop_auto_updated_at_trigger(table_name: &str) -> String {
    format!(
        r#"
        DROP TRIGGER IF EXISTS {table_name}_auto_updated_at ON {table_name};
        DROP FUNCTION IF EXISTS {table_name}_auto_updated_at();
        "#,
    )
}