use crate::database::models::User;
use diesel::mysql::MysqlConnection;

pub fn tick(mut connection: MysqlConnection) {
    match User::star(&mut connection) {
        Ok(users) => {
            for user in users {
                println!("User #{}: {}", user.id, user.name);
            }
        }
        Err(e) => println!("Error: {}", e),
    };

    match User::admins(&mut connection) {
        Ok(admins) => {
            for admin in admins {
                println!("Admin #{}: {}", admin.id, admin.name);
            }
        }
        Err(e) => println!("Error: {}", e),
    };
}
