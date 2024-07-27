use std::{error::Error, io, process};

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("Error running <read_csv>: {}", err);
        process::exit(1);
    }
}
