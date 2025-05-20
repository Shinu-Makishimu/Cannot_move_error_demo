mod dispatcher;
mod task;

use dispatcher::Dispatcher;

fn main() {
    let mut d = Dispatcher::new();

    // push three trivial tasks
    d.push(|| println!("alpha"));
    d.push(|| println!("beta"));
    d.push(|| println!("gamma"));

    d.run_all();
}
