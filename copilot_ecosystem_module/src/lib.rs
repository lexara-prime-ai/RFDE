/*
    A [Marco Polo] game.

    The program will respond with 'Polo'
    when the name 'Marco' is given.

*/

pub mod cmd;
pub mod game_logic;

#[derive(Debug)]
pub struct Call;

pub trait RespondToCall {
    fn respond(name: &str) -> String;
}

impl RespondToCall for Call {
    fn respond(name: &str) -> String {
        match name {
            name if name == "Marco" => "[Answer] -> Polo!".to_owned(),
            _ => "What's your name?".to_owned(),
        }
    }
}
