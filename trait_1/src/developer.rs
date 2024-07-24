use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone)]
pub(crate) struct Developer {
    pub(crate) name: String,
    pub(crate) age: u8,
    pub(crate) language: Language,
}

#[warn(dead_code)]
#[derive(Clone, Debug)]
pub(crate) enum Language {
    Rust,
    Java,
    Python,
    Php,
    TypeScript,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl Display for Developer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} is {} years old, and he is a {:?} developer",
            self.name, self.age, self.language
        )
    }
}
