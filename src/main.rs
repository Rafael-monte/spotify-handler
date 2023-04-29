use crate::handler::args_handler;

mod handler;
mod connectors;
mod configuration;
fn main() {
    println!("Hello, world!");
    println!("{}", args_handler::identify_and_run_args().unwrap());
}
