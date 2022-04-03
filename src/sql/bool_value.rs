use crate::sql::value::Value;

pub struct BoolValue {
    val: bool,
}

impl BoolValue {
    pub fn new(val: bool) -> BoolValue {
        BoolValue { val }
    }
}

impl Value for BoolValue {
    fn to_sql_element(&self) -> String {
        if self.val {
            String::from("TRUE")
        } else {
            String::from("FALSE")
        }
    }
}
