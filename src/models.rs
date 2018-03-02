use schema::blackjack;
use schema::rpgplayer;

#[derive(Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "blackjack"]
pub struct BJSession {
    pub id: i64,
    // None means it was claimed
    pub bet: Option<i64>,
    // None - In Progress
    // true - Player Won
    // false - Player Lost
    pub status: Option<bool>,
    // Empty when game ends
    pub deck: Vec<String>,
    // Empty when game ends
    pub player_hand: Vec<String>,
    // Empty when game ends
    pub dealer_hand: Vec<String>,
    // False by default
    pub player_stay: bool,
    // False by default
    pub dealer_stay: bool,
    // True by default
    pub first_turn: bool,
}

#[derive(Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "rpgplayer"]
pub struct RPGSession {
    pub id: i64,
    pub exp: i64,
    pub damage_recieved: i64,
    pub gear: Vec<String>,
}
