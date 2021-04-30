mod charger;
mod gauge;

use crate::charger::Charger;
use crate::charger::BQ24296;

use crate::gauge::BQ27z561;
use crate::gauge::Gauge;
use crate::gauge::GaugeAdv;
use crate::gauge::GaugeBase;
use crate::gauge::BQ27621;

fn main() {
    let g: BQ27621 = Gauge::new();

    if let Ok(v) = g.get_current() {
        println!("Current is: {}", v)
    }

    let z: BQ27z561 = Gauge::new();

    if let Ok(v) = z.get_cycle_count() {
        println!("Cycle count is: {}", v)
    }

    let c = BQ24296::new();

    if let Err(e) = c.set_current(1280) {
        println!("Error: {:?}", e);
    }

    c.enable_charger(false).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let g: BQ27621 = Gauge::new();

        assert_eq!(g.path(), "bq27621");
    }

    #[test]
    fn test_capacity() {
        let g: BQ27621 = Gauge::new();

        assert_eq!(g.get_capacity().unwrap(), 89);
    }

    #[test]
    #[should_panic(expected = "Failed to read bq27621/voltage")]
    fn test_voltage() {
        let g: BQ27621 = Gauge::new();

        assert_eq!(g.get_voltage().unwrap(), 0);
    }
}
