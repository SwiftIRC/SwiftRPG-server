use crate::database::*;

pub fn tick(pool: &mysql::Pool) {
    match execute_query(&pool, "SELECT * FROM users LIMIT 1") {
        Ok(()) => (),
        Err(e) => {
            println!("Error executing MySQL query: {}", e);
            return;
        }
    };
}
