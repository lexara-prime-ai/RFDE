use clap::Parser;

use crate::{
    game_logic::{Cli, Commands},
    Call, RespondToCall,
};

#[derive(Debug, Clone)]
pub struct Cmd;

impl Cmd {
    pub fn new() {
        let args = Cli::parse();
        match args.command {
            Some(Commands::Play { name }) => {
                println!("{}", Call::respond(&name.as_str()));
            }
            None => println!("No command given!"),
        }
    }
}
