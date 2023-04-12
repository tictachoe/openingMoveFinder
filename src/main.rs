use std::env;
use std::fs::File;
use std::io::{BufReader, Error};
use serde::{Deserialize, Serialize}; //used to serialize the json

// A struct meant to represent an opening
// pretty much the same structure as the json
#[derive(Debug, Deserialize, Serialize)]
struct Opening {
    name: String,
    eco: String,
    fen: String,
    moves: String,
}

fn main() -> Result<(), Error> {
    // gets command line args
    let args: Vec<String> = env::args().collect();
    // if less the 2 args print usage
    if args.len() < 2 {
        println!("Usage: chess-openings [move]");
        return Ok(());
    }
    
    // joining the args to create the move input
    let move_input = &args[1..].join(" ");
    println!("Searching for openings that start with: {}", move_input);

    // opening the json file with the openings
    let file = File::open("eco.json")?;
    let reader = BufReader::new(file);

    let openings: Vec<Opening> = serde_json::from_reader(reader)?;

    // Filter openings by input provided
    let matching_openings: Vec<&Opening> = openings.iter().filter(|opening|{
        opening.moves.starts_with(move_input)
    }).collect();

    // print message if no openings found
    if matching_openings.is_empty() {
        println!("No matching openings found.");
    } else {
        // print opening name and moves if found
        // TODO: add options letting user get eco codes, fen or get openings by eco codes
        println!("Matching openings:");
        for opening in matching_openings {
            println!("Name: {}", opening.name);
            println!("Moves: {}", opening.moves);
            println!();
        }
    }

    Ok(())
}
