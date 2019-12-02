extern crate reqwest;
use std::io;

fn main() {
let mut res = reqwest::get("https://api.challonge.com/v1/tournaments/tournament.json?api_key=THQwE1NobDxeWTRbAb8ACEtrUV4jDse7C6N7PwvU").unwrap();
res.copy_to(&mut io::stdout()).unwrap();
}
