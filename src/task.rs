//! Task abstraction plus one convenience combinator (`map`) that causes trouble.

/// A unit of work that returns some value.
pub trait Task: Send {
    type Output: Send;

    /// Run the task once.
    fn run(&mut self) -> Self::Output;

    // -----------------------------------------------------------
    // ⚠️  The generic combinator below makes the trait **non-object-safe**
    // -----------------------------------------------------------
    fn map<R, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,                 // needed to take `self` by value
        F: FnOnce(Self::Output) -> R + Send,
        R: Send,
    {
        Map { task: self, f }
    }
}

/// The result of calling `task.map(f)`.
pub struct Map<T, F> {
    task: T,
    f: F,
}

impl<T, F, R> Task for Map<T, F>
where
    T: Task,
    F: FnOnce(T::Output) -> R + Send,
    R: Send,
{
    type Output = R;

    fn run(&mut self) -> Self::Output {
        // NOTE: this *also* consumes `f`, but that’s not the compilation
        // error you’re after – it’s just there so everything type-checks
        // once the object-safety issue is solved.
        let output = self.task.run();
        (self.f)(output)
    }
}

// Blanket impl so plain closures can act as Tasks.
impl<F, O> Task for F
where
    F: FnMut() -> O + Send,
    O: Send,
{
    type Output = O;
    fn run(&mut self) -> Self::Output {
        (self)()
    }
}
