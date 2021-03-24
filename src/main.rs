use std::fmt;
use std::fs;
use std::error::Error;

#[derive(Debug)]
struct GError {
    details: String
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

impl GError {
    fn new(msg: &str) -> Self {
        GError{details: msg.to_string()}
    }
}

impl fmt::Display for GError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "libbattery: {}", self.details)
    }
}

impl Error for GError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::num::ParseIntError> for GError {
    fn from(e: std::num::ParseIntError) -> Self {
        //println!("{}", e);
        GError::new(&e.to_string())
    }
}

impl From<std::io::Error> for GError {
    fn from(e: std::io::Error) -> Self {
        GError::new(&e.to_string())
    }
}

impl GaugeChip {
    fn path(&self) -> &str {
        match self {
            GaugeChip::BQ27621(_) => { "bq27621" },
            GaugeChip::BQ27z561(_) => { "bq27z561" },
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
    fn get_capacity(&self) -> Result<u32, GError> {
        read_u32_property(format!("{}{}", self.chip.path(), "/capacity").as_str())
    }

    fn get_current(&self) -> Result<i32, GError> {
        read_i32_property(format!("{}{}", self.chip.path(), "/current_now").as_str())
    }

    fn get_cycle_count(&self) -> Result<u32, GError> {
        match self.chip {
            GaugeChip::BQ27z561(_) => {
                read_u32_property(format!("{}{}", self.chip.path(), "/cycle_count").as_str())
            },
            _ => Err(GError::new("Not supported")),
        }
    }
}

fn read_i32_property(path: &str) -> Result<i32, GError> {

    println!("Read path {}", path);

    let value= fs::read_to_string(path)?.trim_end_matches('\n').parse::<i32>()?;

    Ok(value)
}

fn read_u32_property(path: &str) -> Result<u32, GError> {
    let value= fs::read_to_string(path)?.trim_end_matches('\n').parse::<u32>()?;

    Ok(value)
}

fn main() {
    let gauge_path = "/home/chenhuiz/work/rust/gauge/bq27z561/";
    // --snip--
    println!("In file {}", gauge_path);

    let capacity = fs::read_to_string(format!("{}{}", gauge_path, "capacity"))
        .expect("Something went wrong reading the file");

    println!("With text:{}", capacity.trim_end_matches('\n').parse::<u32>().unwrap());

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
}
