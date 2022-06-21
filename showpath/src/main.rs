use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let paths: String = env::var("PATH").expect("failed to find PATH variable");

    let mut line_numbers: bool = false;

    if args.len() != 0 {
        for arg in args.iter() {
            match &arg[..] {
                "-l" | "--lines" => line_numbers = true,
                _ => {
                    help();
                    return;
                },
            }
        }
    }

    for (line, path) in paths.split(':').enumerate() {
        if line_numbers {
            let width = paths.len() as f32;
            let width = width.log10() as usize;

            let line = line.to_string();
            let line = line + ":";

            print!("{line:<width$} ", line=line, width=width + 1);
        }

        println!("{}", path);
    }
}

fn help() {
    println!("showpath [OPTIONS]");
    println!();
    println!("  -h --help   print this help menu");
    println!("  -l --lines  show line numbers");
}
