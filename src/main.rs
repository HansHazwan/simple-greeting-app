use std::env;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Hello, World!");
    } else {
        args.remove(0);

        let message = args.join(" ");

        println!("Hello {}!", message);
    }

    Ok(())
}
