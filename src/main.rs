use crate::handler::args_handler;

mod handler;
mod connectors;
mod configuration;
fn main() {
   let result = args_handler::identify_and_run_args();
   if result.is_err() {
      eprintln!("{}", result.unwrap_err().to_string())
   }
}
