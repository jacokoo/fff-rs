pub trait Functional: Sized {
    fn also<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }

    fn apply<F: FnOnce(&mut Self)>(&mut self, f: F) {
        f(self);
    }

    fn map_to<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
        return f(self);
    }
}

impl<T: Sized> Functional for T {}

pub struct Publisher<T: Send + Sync>(Vec<Box<dyn Fn(&T) + 'static + Send + Sync>>);

impl<T: Send + Sync> Publisher<T> {
    pub fn new() -> Self {
        Publisher(Vec::new())
    }

    pub fn subscribe<F: Fn(&T) + 'static + Send + Sync>(&mut self, ff: F) {
        self.0.push(Box::new(ff));
    }

    pub fn notify(&self, t: &T) {
        self.0.iter().for_each(|f| f(t))
    }
}
