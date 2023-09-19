#[macro_use]
extern crate rocket;

pub mod post;
pub mod player;
pub mod league;
pub mod season;


pub use league::Entity as League;
pub use player::Entity as Player;
pub use season::Entity as Season;