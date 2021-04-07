use anyhow::{anyhow, Context, Result};
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq)]
enum GaugeChip {
    BQ27621,
    BQ27z561,
}

#[derive(Debug)]
struct Gauge {
    chip: GaugeChip,
    capacity: u32,
    remaining_capacity: u32,
    full_capacity: u32,
    voltage: u32,
    current: i32,
}

impl fmt::Display for GaugeChip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BQ27621 => write!(f, "BQ27621"),
            Self::BQ27z561 => write!(f, "BQ27z561"),
        }
    }
}

impl GaugeChip {
    fn path(&self) -> &str {
        match self {
            GaugeChip::BQ27621 => "bq27621",
            GaugeChip::BQ27z561 => "bq27z561",
        }
    }
}

impl Default for Gauge {
    fn default() -> Self {
        Gauge {
            chip: GaugeChip::BQ27621,
            capacity: 0,
            remaining_capacity: 0,
            full_capacity: 0,
            voltage: 0,
            current: 0,
        }
    }
}

impl Gauge {
    fn get_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/capacity").as_str())
    }

    fn get_voltage(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/voltage").as_str())
    }

    fn get_current(&self) -> Result<i32> {
        read_i32_property(format!("{}{}", self.chip.path(), "/current_now").as_str())
    }

    fn get_full_charge_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/charge_full").as_str())
    }

    fn get_charge_now_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/charge_now").as_str())
    }

    fn get_cycle_count(&self) -> Result<u32> {
        match self.chip {
            GaugeChip::BQ27z561 => {
                read_u32_property(format!("{}{}", self.chip.path(), "/cycle_count").as_str())
            }
            _ => Err(anyhow!("Not support 'get_cycle_count' for {}", self.chip)),
        }
    }

    fn get_time_to_full(&self) -> Result<u32> {
        match self.chip {
            GaugeChip::BQ27z561 => {
                read_u32_property(format!("{}{}", self.chip.path(), "/time_to_full_now").as_str())
            }
            _ => Err(anyhow!("Not support 'get_time_to_full' for {}", self.chip)),
        }
    }

    fn get_time_to_empty(&self) -> Result<u32> {
        match self.chip {
            GaugeChip::BQ27z561 => {
                read_u32_property(format!("{}{}", self.chip.path(), "/time_to_empty_now").as_str())
            }
            _ => Err(anyhow!("Not support 'get_time_to_empty' for {}", self.chip)),
        }
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
    let gauge_path = "/home/chenhuiz/work/rust/gauge/bq27z561/";
    // --snip--
    println!("In file {}", gauge_path);

    let capacity = fs::read_to_string(format!("{}{}", gauge_path, "capacity"))
        .expect("Something went wrong reading the file");

    println!(
        "With text:{}",
        capacity.trim_end_matches('\n').parse::<u32>().unwrap()
    );

    let voltage = read_u32_property(format!("{}{}", gauge_path, "voltage_now").as_str());
    println!("Voltage: {}", voltage.unwrap());

    let g = Gauge {
        //chip: GaugeChip::BQ27z561(String::from("bq27z561")),
        chip: GaugeChip::BQ27z561,
        ..Default::default()
    };

    println!("{:?}", g);

    println!("{}", g.get_capacity().unwrap());
    //println!("{}", g.get_current().unwrap());
    //println!("{}", g.get_cycle_count().unwrap());
    match g.get_current() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    };

    if let Err(e) = g.get_voltage() {
        println!("{:?}", e);
    }

    println!(
        "{}/{}",
        g.get_charge_now_capacity().unwrap() / 1000,
        g.get_full_charge_capacity().unwrap() / 1000
    );

    if let Err(e) = g.get_cycle_count() {
        println!("{:?}", e)
    }

    if g.get_current().unwrap() > 0 {
        println!("Time left: {}", g.get_time_to_empty().unwrap());
    } else {
        println!("Time left: {}", g.get_time_to_full().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let g = Gauge{..Default::default()};

        assert_eq!(g.chip, GaugeChip::BQ27621);
        assert_eq!(g.capacity, 0);
        assert_eq!(g.current, 0);
        assert_eq!(g.voltage, 0);
        assert_eq!(g.remaining_capacity, 0);
        assert_eq!(g.full_capacity, 0);

    }

    #[test]
    fn test_capacity() {
        let g = Gauge{..Default::default()};

        assert_eq!(g.capacity, 0);

        assert_eq!(g.get_capacity().unwrap(), 89);

    }

    #[test]
    #[should_panic(expected = "Failed to read bq27621/voltage")]
    fn test_voltage() {
        let g = Gauge{..Default::default()};

        assert_eq!(g.voltage, 0);

        assert_eq!(g.get_voltage().unwrap(), 89);

    }
}
