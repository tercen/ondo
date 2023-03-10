pub trait NextFn<T> {
    fn next(&mut self) -> Option<T>;
}

pub struct CallbackIterator<'a, T> {
    pub next_fn: Box<dyn NextFn<T> + 'a>,
}

impl<'a, T> Iterator for CallbackIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_fn.next()
    }
}