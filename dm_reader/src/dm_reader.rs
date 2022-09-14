use crate::dm_error::*;
use crate::scanner::*;

pub fn read_file(path: String) -> Result<(), DMError> {
    let buf = std::fs::read_to_string(path).expect("Failed to read input file.");
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report();
            return Err(m);
        }
    }
    Ok(())
}

fn run(source: String) -> Result<(), DMError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
