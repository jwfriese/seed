pub trait Value {
    fn to_sql_element(&self) -> String;
}
