use crate::sql::value::Value;

pub struct IntValue {
    val: u32,
}

impl IntValue {
    pub fn new(val: u32) -> IntValue {
        IntValue { val }
    }
}

impl Value for IntValue {
    fn to_sql_element(&self) -> String {
        format!("{}", self.val)
    }
}