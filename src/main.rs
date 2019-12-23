use structopt::StructOpt;
mod day1;
mod day2;
mod day3;

#[derive(StructOpt)]
struct Cli {
    day: u8,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => println!("Implemented day four in J!"),
        _ => println!("Unimplemented day: {}", args.day),
    }
}
