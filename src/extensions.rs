pub trait OptionFromPredicate<T> {
    fn from_predicate<F>(predicate: bool, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>;
}

impl<T> OptionFromPredicate<T> for Option<T> {
    fn from_predicate<F>(predicate: bool, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        if predicate {
            f()
        } else {
            None
        }
    }
}
