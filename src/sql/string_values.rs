use crate::sql::value::Value;

pub struct StringValue {
    val: String,
}

impl StringValue {
    pub fn new(val: String) -> StringValue {
        StringValue { val }
    }
}

impl Value for StringValue {
    fn to_sql_element(&self) -> String {
        format!("\'{}\'", self.val)
    }
}