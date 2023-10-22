const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() {
    println!("{}", VERSION);
}
