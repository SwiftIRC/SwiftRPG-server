use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Player {
    pub name: String,
    pub position: Coordinate,
    pub inventory: Inventory,
}

impl Player {
    pub fn process(&self) {
        println!("Processing player {}", self.name);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Inventory {
    pub rocks: i32,
    pub trees: i32,
    pub herbs: i32,
    pub fruits: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub fn get_players() -> Vec<Player> {
    vec![
        Player {
            name: "player1".to_string(),
            position: Coordinate { x: 0, y: 0 },
            inventory: Inventory {
                rocks: 0,
                trees: 0,
                herbs: 0,
                fruits: 0,
            },
        },
        Player {
            name: "player2".to_string(),
            position: Coordinate { x: 0, y: 0 },
            inventory: Inventory {
                rocks: 0,
                trees: 0,
                herbs: 0,
                fruits: 0,
            },
        },
    ]
}
