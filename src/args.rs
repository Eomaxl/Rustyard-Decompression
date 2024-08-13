pub fn parse_args() -> Result<String, ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Err(());
    }

    Ok(args[1].clone())
}
