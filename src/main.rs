use afmt::args::{get_args, Args};
use afmt::format;
use anyhow::Result;

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let results = format(args);
    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                println!("Result {}: Ok\n{}", index, value);
                println!("{}", value.replace('\n', "\\n"));
            }
            Err(e) => println!("Result {}: Err\n{}", index, e),
        }
    }
    Ok(())
}
