mod args;
mod extractor;

use std::process;

fn main() {
    process::exit(real_main());
}

fn real_main() -> i32 {
    let args = args::parse_args();
    if args.is_err() {
        return 1;
    }

    let filename = args.unwrap();
    let result = extractor::extract_files(&filename);

    if result.is_err() {
        return 1;
    }

    0
}
