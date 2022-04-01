use std::collections::HashMap;

use crate::database::Pool;
use crate::generation::error::GenerationTableError;
use crate::generation::TableGenerator;

pub fn execute(
    db: Pool,
    generators: HashMap<String, TableGenerator>,
    table: &str,
    count: u8,
) -> Option<GenerationTableError> {
    let get_generator = generators.get(table);
    match get_generator {
        None => {
            Some(
                GenerationTableError::new(
                    table,
                    format!("No generator exists for table '{}'", table).as_str(),
                )
            )
        }
        Some(generator) => {
            let mut num_inserted_records = 0;
            let conn = db.get().unwrap();
            for _i in 0..count {
                let generated_statement = generator.generate_statement();
                match generated_statement {
                    Ok(sql_statement) => {
                        let result = sql_statement.run(&conn);
                        match result {
                            None => {
                                num_inserted_records += 1
                            }
                            Some(err) => {
                                return Some(GenerationTableError::new(
                                    table,
                                    format!(
                                        "Error generating a row for table {}; had completed {} insertions; {}",
                                        table,
                                        num_inserted_records,
                                        err.as_str(),
                                    ).as_str(),
                                ));
                            }
                        }
                    }
                    Err(err) => {
                        return Some(
                            GenerationTableError::new(
                                table,
                                format!("Error generating insert statement for table '{}': {}", table, err).as_str(),
                            )
                        );
                    }
                }
            }
            None
        }
    }
}