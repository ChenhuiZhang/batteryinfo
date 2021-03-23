use std::{convert::TryInto, fs};

use std::fmt;

static BASE_PATH: &str = "/home/chenhuiz/work/rust/gauge/";

enum GError {
    NoSuppot,
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
        write!(fmt, "An Error Occurred, Please Try Again!") // user-facing output
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
        OK(read_property(format!("{}{}", self.chip.path(), "/capacity").as_str()).try_into().unwrap())
    }
}
fn read_property(path: &str) -> i32 {

    println!("Read path {}", path);

    let value = fs::read_to_string(path)
        .expect("Read property failed")
        .trim_end_matches('\n')
        .parse::<i32>()
        .unwrap();

    return value;
}

fn main() {
    let gauge_path = "/home/chenhuiz/work/rust/gauge/bq27z561/";
    // --snip--
    println!("In file {}", gauge_path);

    let capacity = fs::read_to_string(format!("{}{}", gauge_path, "capacity"))
        .expect("Something went wrong reading the file");

    println!("With text:{}", capacity.trim_end_matches('\n').parse::<u32>().unwrap());

    let voltage = read_property(format!("{}{}", gauge_path, "voltage_now").as_str());
    println!("Voltage: {}", voltage);

    let g = Gauge {
        chip: GaugeChip::BQ27z561(String::from("bq27z561")),
        ..Default::default()
    };

    println!("{:?}", g);

    println!("{}", g.get_capacity());
}
