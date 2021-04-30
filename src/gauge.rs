use anyhow::{Context, Result};
use std::fs;

pub trait Gauge {
    // Static method signature; `Self` refers to the implementor type.
    fn new() -> Self;

    // Instance method signatures; these will return a string.
    fn path(&self) -> &'static str;
}

pub trait GaugeBase: Gauge {
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

pub trait GaugeAdv: Gauge {
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

pub struct BQ27621;

pub struct BQ27z561;

impl Gauge for BQ27621 {
    fn new() -> BQ27621 {
        BQ27621
    }

    fn path(&self) -> &'static str {
        "bq27621"
    }
}

impl Gauge for BQ27z561 {
    fn new() -> BQ27z561 {
        BQ27z561
    }

    fn path(&self) -> &'static str {
        "bq27z561"
    }
}

impl GaugeBase for BQ27621 {}

impl GaugeBase for BQ27z561 {}
impl GaugeAdv for BQ27z561 {}

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
