extern crate systemstat;
use systemstat::{System, Platform, ByteSize};

fn main() {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => {
            let m = mem.total - (mem.total - mem.free);
            if m < ByteSize::mb(4000) {
                println!("%{{B#CA6699}}{:.2}%{{B-}} ", m);
            } else if m < ByteSize::mb(12000) {
                println!("%{{F#CCB647}}{:.2}%{{F-}} ", m);
            } else {
                println!("%{{F#BEB6AE}}{:.2}%{{F-}} ", m);
            }
      },
      Err(x) => println!("\nMemory: error: {}", x)
    }
}
