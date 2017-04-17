extern crate getopts;

mod bmpcode;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("o", "", "output file name", "NAME");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let output = match matches.opt_str("o") {
        Some(file) => file,
        None => {
            print_usage(&program, opts);
            return;
        }
    };
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    println!("input: {}, output: {}", input, output);
}
