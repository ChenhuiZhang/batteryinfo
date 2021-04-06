use std::fmt;
use std::fs;

type GaugeResult<T> = Result<T, GError>;

#[derive(Debug)]
enum GError {
    IOErr(std::io::Error),
    ParseErr(std::num::ParseIntError),
    NotSupport,
}
#[derive(Debug)]
enum GaugeChip {
    BQ27621(String),
    BQ27z561(String),
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

impl fmt::Display for GError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GError::NotSupport => {
                write!(fmt, "batteryinfo: Not support")
            }
            GError::IOErr(e) => {
                write!(fmt, "batteryinfo: {}", e)
            }
            GError::ParseErr(e) => {
                write!(fmt, "batteryinfo: {}", e)
            }
        }
    }
}

impl From<std::num::ParseIntError> for GError {
    fn from(e: std::num::ParseIntError) -> Self {
        GError::ParseErr(e)
    }
}

impl From<std::io::Error> for GError {
    fn from(e: std::io::Error) -> Self {
        GError::IOErr(e)
    }
}

impl GaugeChip {
    fn path(&self) -> &str {
        match self {
            GaugeChip::BQ27621(_) => "bq27621",
            GaugeChip::BQ27z561(_) => "bq27z561",
        }
    }
}

impl Default for Gauge {
    fn default() -> Self {
        Gauge {
            chip: GaugeChip::BQ27621(String::from("bq27621")),
            capacity: 0,
            remaining_capacity: 0,
            full_capacity: 0,
            voltage: 0,
            current: 0,
        }
    }
}

impl Gauge {
    fn get_capacity(&self) -> GaugeResult<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/capacity").as_str())
    }

    fn get_voltage(&self) -> GaugeResult<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/voltage").as_str())
    }

    fn get_current(&self) -> GaugeResult<i32> {
        read_i32_property(format!("{}{}", self.chip.path(), "/current_now").as_str())
    }

    fn get_full_charge_capacity(&self) -> GaugeResult<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/charge_full").as_str())
    }

    fn get_charge_now_capacity(&self) -> GaugeResult<u32> {
        read_u32_property(format!("{}{}", self.chip.path(), "/charge_now").as_str())
    }

    fn get_cycle_count(&self) -> GaugeResult<u32> {
        match self.chip {
            GaugeChip::BQ27z561(_) => {
                read_u32_property(format!("{}{}", self.chip.path(), "/cycle_count").as_str())
            }
            _ => Err(GError::NotSupport),
        }
    }

    fn get_time_to_full(&self) -> GaugeResult<u32> {
        match self.chip {
            GaugeChip::BQ27z561(_) => {
                read_u32_property(format!("{}{}", self.chip.path(), "/time_to_full_now").as_str())
            }
            _ => Err(GError::NotSupport),
        }
    }

    fn get_time_to_empty(&self) -> GaugeResult<u32> {
        match self.chip {
            GaugeChip::BQ27z561(_) => {
                read_u32_property(format!("{}{}", self.chip.path(), "/time_to_empty_now").as_str())
            }
            _ => Err(GError::NotSupport),
        }
    }
}

fn read_i32_property(path: &str) -> GaugeResult<i32> {
    println!("Read path {}", path);

    let value = fs::read_to_string(path)?
        .trim_end_matches('\n')
        .parse::<i32>()?;

    Ok(value)
}

fn read_u32_property(path: &str) -> GaugeResult<u32> {
    let value = fs::read_to_string(path)?
        .trim_end_matches('\n')
        .parse::<u32>()?;

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
        chip: GaugeChip::BQ27z561(String::from("bq27z561")),
        ..Default::default()
    };

    println!("{:?}", g);

    println!("{}", g.get_capacity().unwrap());
    //println!("{}", g.get_current().unwrap());
    println!("{}", g.get_cycle_count().unwrap());
    match g.get_current() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    };

    if let Err(e) = g.get_voltage() {
        println!("{}", e);
    }

    println!(
        "{}/{}",
        g.get_charge_now_capacity().unwrap() / 1000,
        g.get_full_charge_capacity().unwrap() / 1000
    );

    if g.get_current().unwrap() > 0 {
        println!("Time left: {}", g.get_time_to_empty().unwrap());
    } else {
        println!("Time left: {}", g.get_time_to_full().unwrap());
    }
}
