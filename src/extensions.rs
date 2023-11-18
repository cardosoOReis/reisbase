pub trait OptionFromPredicate<T> {
    fn from_predicate<F>(predicate: bool, f: F) -> Option<T>
    where
        F: FnOnce(bool) -> Option<T>;
}

impl<T> OptionFromPredicate<T> for Option<T> {
    fn from_predicate<F>(predicate: bool, f: F) -> Option<T>
    where
        F: FnOnce(bool) -> Option<T>,
    {
        if predicate {
            f(true)
        } else {
            None
        }
    }
}
