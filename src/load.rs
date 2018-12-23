extern crate systemstat;
use systemstat::{System, Platform};

fn main() {
    let sys = System::new();
    match sys.load_average() {
        Ok(loadavg) => {
            let mut one: String;
            let mut five: String;
            let mut fifteen: String;
            if loadavg.one < 1.0 {
                one = format!("%{{F#BEB6AE}}{:.2}%{{F-}}", loadavg.one);
            } else if loadavg.one < 2.0 {
                one = format!("%{{F#F1F0EE}}{:.2}%{{F-}}", loadavg.one);
            } else if loadavg.one < 3.0 {
                one = format!("%{{F#FFE200}}{:.2}%{{F-}}", loadavg.one);
            } else {
                one = format!("%{{F#CA6699}}{:.2}%{{F-}}", loadavg.one);
            }
            if loadavg.five < 1.0 {
                five = format!("%{{F#BEB6AE}}{:.2}%{{F-}}", loadavg.five);
            } else if loadavg.five < 2.0 {
                five = format!("%{{F#F1F0EE}}{:.2}%{{F-}}", loadavg.five);
            } else if loadavg.five < 3.0 {
                five = format!("%{{F#FFE200}}{:.2}%{{F-}}", loadavg.five);
            } else {
                five = format!("%{{F#CA6699}}{:.2}%{{F-}}", loadavg.five);
            }
            if loadavg.fifteen < 1.0 {
                fifteen = format!("%{{F#BEB6AE}}{:.2}%{{F-}}", loadavg.fifteen);
            } else if loadavg.five < 2.0 {
                fifteen = format!("%{{F#F1F0EE}}{:.2}%{{F-}}", loadavg.fifteen);
            } else if loadavg.fifteen < 3.0 {
                fifteen = format!("%{{F#FFE200}}{:.2}%{{F-}}", loadavg.fifteen);
            } else {
                fifteen = format!("%{{F#CA6699}}{:.2}%{{F-}}", loadavg.fifteen);
            }
            println!("{} {} {} ", one, five, fifteen)
        },
        Err(x) => println!("\nLoad average: error: {}", x)
    }
}
