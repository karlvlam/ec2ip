extern crate rusoto;

use std::env;
use std::process;
mod ops;

fn main() {
    let args: Vec<String> = env::args().collect();
    if  args.len() < 2 {
        println!("Usage: {} EC2_NAME_PATTERN", args[0]);
        process::exit(1);
    }

    match ops::get_ec2_ips(&args[1]) {
        Ok(v) => {
            for ip in v {
                println!("{}", ip);
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

}

