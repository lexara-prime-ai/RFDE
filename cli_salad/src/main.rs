use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Irfan M. Ghat <irfanghat@gmail.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts = Opts::parse();
    let number_of_fruits = opts.number;

    println!("\n___WISE CHOICE___\n \n-> {} fruits.", number_of_fruits);

    let result = create_fruit_salad(number_of_fruits);

    println!(
        "\nCreating Fruit Salad with {} fruits\n -> {:?}\n",
        number_of_fruits, result
    );
}
