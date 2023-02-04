mod controller;

fn main() {
    let mut controller = controller::Controller::new();
    match controller.acquire() {
        Ok(()) => (),
        Err(e) => panic!("{}", e)
    }
}
