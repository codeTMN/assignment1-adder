#[link(name = "our_code")]
extern "C" {
    // The \x01 prefix prevents the compiler from mangling the name
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here() -> i64;
}

fn main() {
    let i: i64 = unsafe {
        our_code_starts_here()
    };
    println!("{i}");
}