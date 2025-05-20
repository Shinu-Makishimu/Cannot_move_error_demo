//! Holds a queue of tasks and runs them all.

use crate::task::Task;

pub struct Dispatcher {
    // We'll store **trait objects** … and that’s exactly where the
    // compilation blows up↓↓↓
    tasks: Vec<Box<dyn Task<Output = ()>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn push<T>(&mut self, t: T)
    where
        T: Task<Output = ()> + 'static,
    {
        self.tasks.push(Box::new(t));
    }

    pub fn run_all(&mut self) {
        for t in self.tasks.iter_mut() {
            t.run();
        }
    }
}
