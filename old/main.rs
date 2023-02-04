pub mod cli;
fn main() {
    let mut ui = cli::CLI::new();
    ui.start();
}
