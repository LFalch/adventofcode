use std::io::{Result, BufRead, BufReader};
use std::fs::File;

fn main() -> Result<()> {
    let lines = BufReader::new(File::open("day1.input")?);
    let i: u32 = lines
        .lines()
        .map(|l| l.ok().and_then(|l| l.parse().ok()))
        .filter_map(|x| x.and_then(get_required_fuel)).sum();
    println!("{}", i);

    let lines = BufReader::new(File::open("day1.input")?);
    let i: u32 = lines
        .lines()
        .map(|l| l.ok().and_then(|l| l.parse().ok()))
        .filter_map(|x| x.and_then(get_total_fuel_for_module)).sum();
    println!("{}", i);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn twelve() {
        assert_eq!(get_required_fuel(12), Some(2));
    }
    #[test]
    fn fourteen() {
        assert_eq!(get_required_fuel(14), Some(2));
    }
    #[test]
    fn test_1969() {
        assert_eq!(get_required_fuel(1969), Some(654));
    }
    #[test]
    fn test_100756() {
        assert_eq!(get_required_fuel(100756), Some(33583));
    }

    #[test]
    fn module_14() {
        assert_eq!(get_total_fuel_for_module(14), Some(2))
    }
    #[test]
    fn module_1969() {
        assert_eq!(get_total_fuel_for_module(1969), Some(966))
    }
    #[test]
    fn module_100756() {
        assert_eq!(get_total_fuel_for_module(100756), Some(50346))
    }
}

#[inline]
fn get_total_fuel_for_module(module_mass: u32) -> Option<u32> {
    let fuel = get_required_fuel(module_mass)?;

    Some(fuel + get_total_fuel_for_module(fuel).unwrap_or(0))
}

#[inline]
pub fn get_required_fuel(mass: u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}
