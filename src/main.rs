mod charger;
mod gauge;

use crate::charger::{Charger, BQ24296};

use crate::gauge::{BQ27z561, Gauge, GaugeAdv, GaugeBase, BQ27621};

fn logger<T>(b: T) -> T
where
    T: GaugeBase,
{
    println!(
        "capacity: {: >3} {: >4} {: >4} {: >4}",
        b.get_capacity().unwrap_or_default(),
        b.get_charge_now_capacity().unwrap_or_default(),
        b.get_full_charge_capacity().unwrap_or_default(),
        b.get_voltage().unwrap_or_default()
    );

    b
}

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

    logger(g);
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
