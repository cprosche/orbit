use clap::{Parser, Subcommand};

// Constants
const PI: f64 = 3.14159;
const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11; // N*m^2*kg^-2
const SEPARATOR: &str = &"------------------------------------------------------";

fn calculate_orbital_period(mass: f64, semi_major_axis: f64) -> f64 {
    2.0 * PI * ((semi_major_axis.powi(3) / (GRAVITATIONAL_CONSTANT * mass)).sqrt())
}

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

#[derive(Debug, Clone)]
struct Body {
    mass: f64,   // kg
    radius: f64, // m
}

enum Altitude {
    // always in km
    Single { value: f64 },
    Range { max: f64, min: f64 },
}

struct Orbit {
    name: String,
    altitude: Altitude,
    body: Body,
}

impl Orbit {
    fn get_period_string(&self) -> String {
        match &self.altitude {
            Altitude::Single { value } => {
                let axis = self.body.radius + value * 1000.0;
                let period_in_seconds = calculate_orbital_period(self.body.mass, axis).ceil();
                let period_in_minutes = &period_in_seconds / 60.0;
                let period_in_days = &period_in_minutes / (60.0 * 24.0);
                format!("{period_in_seconds}s {period_in_minutes:.2}m {period_in_days:.2}d")
            }
            Altitude::Range { max, min } => {
                let max_axis = self.body.radius + max * 1000.0;
                let min_axis = self.body.radius + min * 1000.0;

                let max_period_in_seconds =
                    calculate_orbital_period(self.body.mass, max_axis).ceil();
                let max_period_in_minutes = &max_period_in_seconds / 60.0;
                let max_period_in_days = &max_period_in_minutes / (60.0 * 24.0);

                let min_period_in_seconds =
                    calculate_orbital_period(self.body.mass, min_axis).ceil();
                let min_period_in_minutes = &min_period_in_seconds / 60.0;
                let min_period_in_days = &min_period_in_minutes / (60.0 * 24.0);

                format!("{min_period_in_seconds}-{max_period_in_seconds}s {min_period_in_minutes:.2}-{max_period_in_minutes:.2}m {min_period_in_days:.2}-{max_period_in_days:.2}d")
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Earth) => earth(),
        None => println!("No commands provided"),
    }
}

fn earth() {
    let earth = Body {
        mass: 5.9722e24,
        radius: 6.3781e6,
    };
    let orbits = vec![
        Orbit {
            name: "VLEO".to_string(),
            altitude: Altitude::Range {
                max: 450.0,
                min: 100.0,
            },
            body: earth.clone(),
        },
        Orbit {
            name: "LEO".to_string(),
            altitude: Altitude::Range {
                max: 2000.0,
                min: 450.0,
            },
            body: earth.clone(),
        },
        Orbit {
            name: "MEO".to_string(),
            altitude: Altitude::Range {
                min: 2000.0,
                max: 36000.0,
            },
            body: earth.clone(),
        },
        Orbit {
            name: "GEO".to_string(),
            altitude: Altitude::Single { value: 35786.0 },
            body: earth.clone(),
        },
    ];

    println!("Orbital Periods");
    println!("{SEPARATOR}");

    orbits
        .iter()
        .for_each(|o| println!("{}: {}", o.name, o.get_period_string()));

    println!("{SEPARATOR}")
}
