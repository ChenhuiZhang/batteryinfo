use anyhow::{Context, Result};
use std::fs;

trait GaugeBase {
    // Static method signature; `Self` refers to the implementor type.
    fn new() -> Self;

    // Instance method signatures; these will return a string.
    fn path(&self) -> &'static str;

    fn get_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/capacity").as_str())
    }

    fn get_voltage(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/voltage").as_str())
    }

    fn get_current(&self) -> Result<i32> {
        read_i32_property(format!("{}{}", self.path(), "/current_now").as_str())
    }

    fn get_full_charge_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/charge_full").as_str())
    }

    fn get_charge_now_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/charge_now").as_str())
    }
}

struct BQ27621 {
    name: &'static str,
}

struct BQ27z561 {
    name: &'static str,
}

impl GaugeBase for BQ27621 {
    fn new() -> BQ27621 {
        BQ27621 { name: "bq27621" }
    }

    fn path(&self) -> &'static str {
        self.name
    }
}

impl GaugeBase for BQ27z561 {
    fn new() -> BQ27z561 {
        BQ27z561 { name: "bq27z561" }
    }

    fn path(&self) -> &'static str {
        self.name
    }
}

impl BQ27z561 {
    fn get_time_to_full(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/time_to_full_now").as_str())
    }

    fn get_time_to_empty(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/time_to_empty_now").as_str())
    }

    fn get_cycle_count(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/cycle_count").as_str())
    }
}

fn read_i32_property(path: &str) -> Result<i32> {
    let s = fs::read_to_string(path).context(format!("Failed to read {}", path))?;

    let value = s
        .trim_end_matches('\n')
        .parse::<i32>()
        .context(format!("Failed to parse {} into i32", s))?;

    Ok(value)
}

fn read_u32_property(path: &str) -> Result<u32> {
    let s = fs::read_to_string(path).context(format!("Failed to read {}", path))?;

    let value = s
        .trim_end_matches('\n')
        .parse::<u32>()
        .context(format!("Failed to parse {} into u32", s))?;

    Ok(value)
}

fn main() {
    let g: BQ27621 = GaugeBase::new();

    if let Ok(v) = g.get_current() {
        println!("Current is: {}", v)
    }

    let z: BQ27z561 = GaugeBase::new();

    if let Ok(v) = z.get_cycle_count() {
        println!("Cycle count is: {}", v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let g: BQ27621 = GaugeBase::new();

        assert_eq!(g.path(), "bq27621");
    }

    #[test]
    fn test_capacity() {
        let g: BQ27621 = GaugeBase::new();

        assert_eq!(g.get_capacity().unwrap(), 89);
    }

    #[test]
    #[should_panic(expected = "Failed to read bq27621/voltage")]
    fn test_voltage() {
        let g: BQ27621 = GaugeBase::new();

        assert_eq!(g.get_voltage().unwrap(), 0);
    }
}
