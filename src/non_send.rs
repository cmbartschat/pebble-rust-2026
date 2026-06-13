pub enum NonSend<T> {
    Good(T),
    Poisoned,
}

unsafe impl<T> core::marker::Sync for NonSend<T> {}

unsafe impl<T> core::marker::Send for NonSend<T> {}

impl<T> NonSend<T> {
    pub fn new(v: T) -> Self {
        Self::Good(v)
    }
}
