use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Celestial body to calculate orbits for
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Earth,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Earth) => earth(),
        None => println!("No commands provided"),
    }
}

fn earth() {
    println!("hello earth");
}
