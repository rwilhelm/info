extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};

fn main() {
    let sys = System::new();

    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            //println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            let load = cpu.user * 100.0 + cpu.nice * 100.0 + cpu.system * 100.0 + cpu.interrupt * 100.0;
            if load < 20.0 {
                println!(" %{{F#BEB6AE}}{:.2}%%{{F-}} ", load);
            } else if load < 40.0 {
                println!(" %{{F#F1F0EE}}{:.2}%%{{F-}} ", load);
            } else if load < 60.0 {
                println!(" %{{F#A8FF00}}{:.2}%%{{F-}} ", load);
            } else if load < 80.0 {
                println!(" %{{F#FFE200}}{:.2}%%{{F-}} ", load);
            } else if load < 90.0 {
                println!(" %{{F#CA6699}}{:.2}%%{{F-}} ", load);
            } else {
                println!(" %{{F#CA6699}}{:.2}%%{{F-}} ", load);
            }
        },
        Err(x) => println!("\nCPU load: error: {}", x)
    }
}
