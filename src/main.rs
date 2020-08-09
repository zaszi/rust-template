/// Main binary function, responsible for:
/// * Calling command line parsing logic
/// * Setting up configuration
/// * Calling the run function in lib.rs
/// * Handling the error if the above returns an error
pub fn main() {
    rust_template::run();
}
