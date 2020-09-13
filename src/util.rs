pub trait SingleUseExt<T: 'static> {
    /// Takes the value, leaving a None in its place.
    ///
    /// # Panics
    ///
    /// Panics if the value was already taken.
    fn take_unwrapped(&self) -> T;
}
impl<T: 'static> SingleUseExt<T> for druid::SingleUse<T> {
    fn take_unwrapped(&self) -> T {
        match self.take() {
            Some(value) => value,
            None => take_unwrapped_panic(),
        }
    }
}

// This is a separate function to reduce the code size of `take_panicking()` itself.
#[inline(never)]
#[cold]
fn take_unwrapped_panic() -> ! {
    panic!("cannot take SingleUse value twice")
}
