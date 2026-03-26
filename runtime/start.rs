use std::env;

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    if errcode == 1 {
        eprintln!("invalid argument");
    } else if errcode == 2 {
        eprintln!("overflow");
    } else {
        eprintln!("an error occurred: {}", errcode);
    }
    std::process::exit(1);
}

fn print_value(val: i64) {
    if val == 3 {
        println!("true");
    } else if val == 1 {
        println!("false");
    } else if val & 1 == 0 {
        println!("{}", val >> 1); // Shift right to untag number
    } else {
        println!("Unknown value: {}", val);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        if args[1] == "true" {
            3 // 0b11
        } else if args[1] == "false" {
            1 // 0b01
        } else {
            let parsed: i64 = args[1].parse().expect("Invalid input");
            parsed << 1 // Shift left for numbers
        }
    } else {
        1 // Default to false if no input
    };

    let result = unsafe { our_code_starts_here(input) };
    print_value(result);
}

#[link(name = "our_code")]
extern "C" {
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64) -> i64;
}