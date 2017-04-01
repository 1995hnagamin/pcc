extern crate getopts;

use getopts::Options;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    println!("Hello, world!");
}
