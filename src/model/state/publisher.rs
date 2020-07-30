pub struct Publisher<T>(Vec<Box<dyn Fn(&T) + 'static>>);

impl<T> Publisher<T> {
    pub fn new() -> Self {
        Publisher(Vec::new())
    }

    pub fn subscribe<F: Fn(&T) + 'static>(&mut self, ff: F) {
        self.0.push(Box::new(ff));
    }

    pub fn notify(&self, t: &T) {
        self.0.iter().for_each(|f| f(t))
    }
}
