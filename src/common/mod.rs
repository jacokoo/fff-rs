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
