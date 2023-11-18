pub trait OptionIfPresent<T> {
    fn if_present<F>(self, f: F)
    where
        F: FnOnce(T);
}

impl<T> OptionIfPresent<T> for Option<T> {
    fn if_present<F>(self, f: F)
    where
        F: FnOnce(T),
    {
        if let Some(value) = self {
            f(value);
        }
    }
}
