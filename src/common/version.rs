
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");


pub fn show_version(){
    println!("{}", VERSION);
}