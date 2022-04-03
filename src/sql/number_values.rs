use crate::sql::value::Value;

pub struct IntValue {
    val: i64,
}

impl IntValue {
    pub fn new(val: i64) -> IntValue {
        IntValue { val }
    }
}

impl Value for IntValue {
    fn to_sql_element(&self) -> String {
        format!("{}", self.val)
    }
}
