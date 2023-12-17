use crate::database::models::user::User;
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

                match admin.clone().add_xp(4, 1, &mut connection) {
                    Ok(_) => println!("Added XP"),
                    Err(e) => println!("Error: {}", e),
                };

                match admin.clone().xp(1, &mut connection) {
                    Ok(xp) => println!("XP: {}", xp),
                    Err(e) => println!("Error: {}", e),
                };

                match admin.clone().skills(&mut connection) {
                    Ok(skills) => {
                        println!("Skills: {:?}", skills);
                        for skill in skills {
                            println!(
                                "Skill: {} XP: {}",
                                match skill.1 {
                                    Some(skill) => skill.name,
                                    e => format!("Error: {:?}", e),
                                },
                                skill.0.quantity
                            );
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                };
            }
        }
        Err(e) => println!("Error: {}", e),
    };
}
