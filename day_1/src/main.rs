use std::io;
use std::io::{BufReader, BufRead};

fn main() {
    let input = io::stdin();
    let buffer = BufReader::new(input);

    let mut total_required_fuel: i64 = 0;
    for line in buffer.lines() {

        let line = line.expect("Failed to read line");

        // Fuel is (floor(mass / 3) - 2)
        let module_mass: i64 = line.parse().unwrap();
        let module_required_fuel = get_fuel_requirement(module_mass);

        total_required_fuel += module_required_fuel;
    }
    println!("Requires {} units of fuel", total_required_fuel);
}

fn get_fuel_requirement(mass: i64) -> i64 {
    let fuel_requirement: i64 = ((mass as f64 / 3.0).floor() as i64) - 2;
    if fuel_requirement <= 0 {
        return 0;
    }
    return fuel_requirement + get_fuel_requirement(fuel_requirement);
}
