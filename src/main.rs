use std::env;
use huff::menu;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "-h" {
        menu()
    } else if args.len() == 4 && args[1] == "-c" && args[2] == "-f" {

    } else if args.len() == 4 && args[1] == "-d" && args[2] == "-f" {

    } else if args.len() == 4 && args[1] == "-s" && args[2] == "-f" {

    }
    Ok(())
}
