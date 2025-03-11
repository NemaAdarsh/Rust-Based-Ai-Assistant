use syn::{parse_file, Item};
//do not mess with this code else it will shit itself 






pub fn analyze_code(code: &str) {
    match parse_file(code) {
        Ok(parsed) => {
            for item in parsed.items {
                match item {
                    Item::Fn(f) => println!("Found function: {}", f.sig.ident),
                    _ => {}
                }
            }
        }
        Err(err) => println!("Error parsing code: {}", err),
    }
}
