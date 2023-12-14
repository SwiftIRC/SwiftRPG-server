use mysql::{prelude::Queryable, Opts, Pool, Row, TxOpts};

pub fn establish_connection_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    // Define your database configuration
    let config = DbConfig {
        host: "localhost".to_string(),
        port: 3306,
        user: "swiftrpg".to_string(),
        password: "swiftrpgpassword".to_string(),
        database: "swiftrpg".to_string(),
    };

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    );

    let options = Opts::from_url(&url)?;

    let pool = match Pool::new(options) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Error establishing MySQL connection: {}", e);
            return Err(Box::new(e));
        }
    };

    Ok(pool)
}

pub fn execute_query(pool: &Pool, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Acquire a connection from the pool
    let mut conn = pool.get_conn()?;

    let mut transaction = match conn.start_transaction(TxOpts::default()) {
        Ok(transaction) => transaction,
        Err(e) => {
            println!("Error: {}", e);
            return Err(Box::new(e));
        }
    };

    // Execute a simple query
    let query = format!("{query} FOR UPDATE SKIP LOCKED");
    let rows: Vec<Row> = match transaction.query(query) {
        Ok(rows) => rows,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    // Process the query result
    for row in rows {
        let column1: u32 = match get_row(&row, "id") {
            Ok(column) => column,
            Err(()) => continue,
        };
        let column2: String = match get_row(&row, "name") {
            Ok(column) => column,
            Err(()) => continue,
        };

        // Process other columns as needed
        println!("ID: {column1}, Username: {column2}");
    }

    match transaction.commit() {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Error: {}", e);
            return Err(Box::new(e));
        }
    };
}

pub fn get_row<T>(row: &Row, index: &str) -> Result<T, ()>
where
    T: mysql::prelude::FromValue,
{
    let column = match row.get(index) {
        Some(column) => match column {
            Some(column) => column,
            None => {
                println!("Error converting column `{}` to String [E50]", index);
                return Err(());
            }
        },
        None => {
            println!("Error extracting column `{}` to String [E51]", index);
            return Err(());
        }
    };

    Ok(column)
}

#[derive(Debug)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}
