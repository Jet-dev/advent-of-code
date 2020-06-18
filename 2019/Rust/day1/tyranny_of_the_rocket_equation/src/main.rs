use std::fs;
use std::str::Lines;

fn main() {
    let module_masses = fs::read_to_string("src/module_masses.in").unwrap();
    let sum_of_fuel = calculate_fuel_for_all_modules(module_masses.lines());

    println!("mass for all modules {}", sum_of_fuel);
}

fn calculate_fuel_for_mass_and_fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + calculate_fuel_for_mass_and_fuel(fuel)
}

fn calculate_fuel_for_all_modules(modules_masses: Lines<'_>) -> i32 {
    modules_masses
        .map(|x| x.parse().unwrap())
        .map(|x| calculate_fuel_for_mass_and_fuel(x))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::calculate_fuel_for_mass_and_fuel;

    #[test]
    fn simple_fuel_calculation() {
        assert_eq!(2, calculate_fuel_for_mass_and_fuel(12))
    }

    #[test]
    fn simple_fuel_calculation_with_rounding() {
        assert_eq!(2, calculate_fuel_for_mass_and_fuel(14))
    }

    #[test]
    fn intermediate_fuel_calculation_with_rounding() {
        assert_eq!(966, calculate_fuel_for_mass_and_fuel(1969))
    }

    #[test]
    fn fuel_calculation_with_rounding() {
        assert_eq!(50346, calculate_fuel_for_mass_and_fuel(100756))
    }
}
