use std::{f64, rc::Rc};

use clap::{Parser, Subcommand};

// Constants
const PI: f64 = 3.14159;
const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11; // N*m^2*kg^-2
const SEPARATOR: &str = &"------------------------------------------------------";

fn calculate_orbital_period(mass: f64, semi_major_axis: f64) -> f64 {
    // mass: kg
    // semi_major_axis: m
    2.0 * PI * ((semi_major_axis.powi(3) / (GRAVITATIONAL_CONSTANT * mass)).sqrt())
}

fn calculate_circular_orbital_velocity(mass: f64, semi_major_axis: f64) -> f64 {
    // mass: kg
    // semi_major_axis: m
    ((GRAVITATIONAL_CONSTANT * mass) / semi_major_axis).sqrt() // m / s
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Celestial body to calculate orbits for
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate common satellite orbits for Earth
    Earth { altitude: Option<f64> },
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
    body: Rc<Body>,
}

impl Orbit {
    fn get_period_string(&self) -> String {
        match &self.altitude {
            Altitude::Single { value } => {
                let axis = self.body.radius + value * 1000.0;
                let period_in_seconds = calculate_orbital_period(self.body.mass, axis).ceil();
                let period_in_minutes = &period_in_seconds / 60.0;
                let period_in_days = &period_in_minutes / (60.0 * 24.0);

                format!(
                    "{period_in_seconds} seconds
{period_in_minutes:.2} minutes
{period_in_days:.2} days"
                )
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

                format!(
                    "{min_period_in_seconds}-{max_period_in_seconds} seconds
{min_period_in_minutes:.2}-{max_period_in_minutes:.2} minutes 
{min_period_in_days:.2}-{max_period_in_days:.2} days"
                )
            }
        }
    }

    fn get_velocity_string(&self) -> String {
        match &self.altitude {
            Altitude::Single { value } => {
                let axis = self.body.radius + value * 1000.0;

                let velocity =
                    (calculate_circular_orbital_velocity(self.body.mass, axis) * 60.0 * 60.0)
                        / 1000.0;
                format!("{velocity:.2} km/hr")
            }
            Altitude::Range { max, min } => {
                let min_axis = self.body.radius + min * 1000.0;
                let max_axis = self.body.radius + max * 1000.0;

                let min_velocity =
                    calculate_circular_orbital_velocity(self.body.mass, min_axis) * 60.0 * 60.0
                        / 1000.0;
                let max_velocity =
                    calculate_circular_orbital_velocity(self.body.mass, max_axis) * 60.0 * 60.0
                        / 1000.0;

                format!("{min_velocity:.2}-{max_velocity:.2} km/s")
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Earth { altitude }) => earth(altitude.clone()),
        _ => {}
    }
}

fn earth(altitude: Option<f64>) {
    let earth = Rc::new(Body {
        mass: 5.9722e24,
        radius: 6.3781e6,
    });

    let mut orbits = vec![];

    match &altitude {
        Some(value) => orbits.push(Orbit {
            name: "User Defined".to_string(),
            altitude: Altitude::Single {
                value: value.clone(),
            },
            body: earth.clone(),
        }),
        None => {
            orbits.push(Orbit {
                name: "VLEO".to_string(),
                altitude: Altitude::Range {
                    max: 450.0,
                    min: 100.0,
                },
                body: earth.clone(),
            });
            orbits.push(Orbit {
                name: "LEO".to_string(),
                altitude: Altitude::Range {
                    max: 2000.0,
                    min: 450.0,
                },
                body: earth.clone(),
            });
            orbits.push(Orbit {
                name: "MEO".to_string(),
                altitude: Altitude::Range {
                    min: 2000.0,
                    max: 36000.0,
                },
                body: earth.clone(),
            });
            orbits.push(Orbit {
                name: "GEO".to_string(),
                altitude: Altitude::Single { value: 35786.0 },
                body: earth.clone(),
            });
        }
    }

    println!();
    println!("Constants");
    println!("{SEPARATOR}");
    println!("Pi: {PI}");
    println!("Gravitational Constant: {GRAVITATIONAL_CONSTANT:+e} N*m^2*kg^-2");
    println!("Earth Mass: {:+e} kg", earth.mass);
    println!("Earth Radius: {:+e} m", earth.radius);
    println!("{SEPARATOR}");
    println!();
    println!();
    println!("Orbital Periods");
    println!("{SEPARATOR}");
    println!();

    orbits.iter().for_each(|o| match &o.altitude {
        Altitude::Single { value } => {
            println!("{} ({value} km) \n{}\n", o.name, o.get_period_string());
        }
        Altitude::Range { max, min } => {
            println!("{} ({min}-{max} km) \n{}\n", o.name, o.get_period_string());
        }
    });

    println!("{SEPARATOR}");
    println!();
    println!();
    println!("Orbital Velocities");
    println!("{SEPARATOR}");
    println!();

    orbits.iter().for_each(|o| match &o.altitude {
        Altitude::Single { value } => {
            println!("{} ({value} km) \n{}\n", o.name, o.get_velocity_string());
        }
        Altitude::Range { max, min } => {
            println!(
                "{} ({min}-{max} km) \n{}\n",
                o.name,
                o.get_velocity_string()
            );
        }
    });

    println!("{SEPARATOR}");
    println!()
}
