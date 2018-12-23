extern crate systemstat;
use systemstat::{System, Platform};

fn main() {
    let sys = System::new();
    match sys.socket_stats() {
        Ok(stats) => {
            println!(" {:?} {:?} {:?} ", stats.tcp_sockets_in_use, stats.udp_sockets_in_use, stats.tcp_sockets_orphaned);
        },
        Err(x) => println!("\nError: {}", x.to_string())
    }
}
