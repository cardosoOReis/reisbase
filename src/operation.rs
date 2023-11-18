pub struct Operation {
    pub action: String,
    pub key: Option<String>,
    pub value: Option<String>,
    pub arguments: Vec<String>,
}

impl Operation {
    pub fn new(
        action: Option<String>,
        key: Option<String>,
        value: Option<String>,
        arguments: Vec<String>,
    ) -> Option<Operation> {
        action.map(|action| Operation {
            action,
            key,
            value,
            arguments,
        })
    }

    pub fn put(key: &str, value: &str) -> Operation {
        Operation {
            action: String::from("put"),
            key: Some(key.to_owned()),
            value: Some(value.to_owned()),
            arguments: Vec::new(),
        }
    }

    pub fn clear() -> Operation {
        Operation {
            action: String::from("c"),
            key: None,
            value: None,
            arguments: vec![String::from("-f")],
        }
    }
}
