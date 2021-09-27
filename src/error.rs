pub fn error(line: u32, column: u32, message: &str) {
    report(line, column, "", message);
}

fn report(line: u32, column: u32, location: &str, message: &str) {
    eprintln!(
        "[line {} column {}] Error {}: {}",
        line, column, location, message
    );
}
