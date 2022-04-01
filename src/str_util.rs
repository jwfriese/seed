pub fn pluralize(count: u16, singular: &str, pluralized: &str) -> String {
    if count == 1 {
        return String::from(singular);
    }
    String::from(pluralized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_pluralized_version_when_given_number_greater_than_one_or_zero() {
        assert_eq!(pluralize(0, "turtle", "turtles"), String::from("turtles"));
        assert_eq!(pluralize(2, "turtle", "turtles"), String::from("turtles"));
    }

    #[test]
    fn it_returns_non_pluralized_version_when_given_number_equal_to_one() {
        assert_eq!(pluralize(1, "turtle", "turtles"), String::from("turtle"));
    }
}
