pub trait Functional: Sized {
    fn also<F: FnOnce(&Self)>(self, f: F) -> Self {
        f(&self);
        self
    }

    fn also_mut<F: FnMut(&mut Self)>(mut self, mut f: F) -> Self {
        f(&mut self);
        self
    }

    fn map_to<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
        return f(self);
    }
}

impl<T: Sized> Functional for T {}

pub struct Publisher<T: Send>(Vec<Box<dyn Fn(&T) + 'static + Send>>);

impl<T: Send> Publisher<T> {
    pub fn new() -> Self {
        Publisher(Vec::new())
    }

    pub fn subscribe<F: Fn(&T) + 'static + Send>(&mut self, ff: F) {
        self.0.push(Box::new(ff));
    }

    pub fn notify(&self, t: &T) {
        self.0.iter().for_each(|f| f(t))
    }
}
