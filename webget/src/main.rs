use std::{env, error::Error, process};
use webget::get_url;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} HOST PATH\n", args[0]);
        eprintln!("\tExample: {} stanford.edu /class/cs144\n", args[0]);
        process::exit(1)
    }
    let url = &args[1];
    let path = &args[2];
    get_url(url, path)?;
    Ok(())
}
