pub trait OptionFromPredicate<T> {
    fn from_predicate<F>(predicate: bool, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>;
}

pub trait PeekOption<T> {
    fn peek<F>(self, f: F) -> Self
    where
        F: FnOnce(&T);
}

impl<T> PeekOption<T> for Option<T> {
    fn peek<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Some(ref value) = self {
            f(value)
        }

        self
    }
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

pub trait ResultFromPredicate<T, E> {
    fn from_predicate<OnTrue, OnFalse>(
        predicate: bool,
        on_true: OnTrue,
        on_false: OnFalse,
    ) -> Result<T, E>
    where
        OnTrue: FnOnce() -> T,
        OnFalse: FnOnce() -> E;
}

impl<T, E> ResultFromPredicate<T, E> for Result<T, E> {
    fn from_predicate<OnTrue, OnFalse>(
        predicate: bool,
        on_true: OnTrue,
        on_false: OnFalse,
    ) -> std::result::Result<T, E>
    where
        OnTrue: FnOnce() -> T,
        OnFalse: FnOnce() -> E,
    {
        if predicate {
            Ok(on_true())
        } else {
            Err(on_false())
        }
    }
}
