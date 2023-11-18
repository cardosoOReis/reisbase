#[derive(Debug, PartialEq, Eq)]
pub enum ReisbaseActionsArguments {
    Force,
    Help,
    Clipboard,
    Description,
}

impl ReisbaseActionsArguments {
    pub fn new(action: &str) -> Option<ReisbaseActionsArguments> {
        match action {
            "-f" => Some(ReisbaseActionsArguments::Force),
            "-h" => Some(ReisbaseActionsArguments::Help),
            "-c" => Some(ReisbaseActionsArguments::Clipboard),
            "-d" => Some(ReisbaseActionsArguments::Description),
            _ => None,
        }
    }
}

impl std::fmt::Display for ReisbaseActionsArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReisbaseActionsArguments::Force => write!(f, "-f (Force)"),
            ReisbaseActionsArguments::Help => write!(f, "-h (Help)"),
            ReisbaseActionsArguments::Clipboard => write!(f, "-c (Copy to Clipboard)"),
            ReisbaseActionsArguments::Description => write!(f, "-d (Description)"),
        }
    }
}
