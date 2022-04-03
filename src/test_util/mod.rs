use regex::{Captures, Regex};

pub fn extract_value_from_sql_insert(query: &str) -> Option<String> {
    let regex = Regex::new(r"\(([^)]+)\)").unwrap();
    let matches = regex.captures_iter(query).collect::<Vec<Captures>>();
    if matches.is_empty() {
        return None;
    }

    match matches.get(matches.len() - 1) {
        None => None,
        Some(captures) => Some(String::from(&captures[1])),
    }
}

#[cfg(test)]
mod test {
    use crate::test_util::extract_value_from_sql_insert;

    #[test]
    fn it_can_extract_an_insert_value_from_an_insert_sql_query() {
        match extract_value_from_sql_insert("INSERT INTO bananas (blah) VALUES (10)") {
            None => {
                assert!(
                    false,
                    "Expected to extract a value from a SQL statement, but found nothing"
                );
            }
            Some(v) => {
                assert_eq!(v, String::from("10"));
            }
        }

        match extract_value_from_sql_insert("INSERT INTO bananas (blah) VALUES ('thing')") {
            None => {
                assert!(
                    false,
                    "Expected to extract a value from a SQL statement, but found nothing"
                );
            }
            Some(v) => {
                assert_eq!(v, String::from("'thing'"));
            }
        }

        match extract_value_from_sql_insert("INSERT INTO bananas (blah) VALUES (TRUE)") {
            None => {
                assert!(
                    false,
                    "Expected to extract a value from a SQL statement, but found nothing"
                );
            }
            Some(v) => {
                assert_eq!(v, String::from("TRUE"));
            }
        }
    }
}
