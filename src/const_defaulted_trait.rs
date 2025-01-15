#[const_trait]
pub trait Defaulted {
    fn defaulted() -> Self;
}

impl const Defaulted for String {
    fn defaulted() -> Self {
        String::new()
    }
}

struct ActiveUser(Option<String>);

impl Defaulted for ActiveUser {
    fn defaulted() -> Self {
        match std::env::var("ACTIVE_USER") {
            Ok(v) => ActiveUser(Some(v)),
            Err(_) => ActiveUser(None),
        }
    }
}

impl std::fmt::Display for ActiveUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(v) => write!(f, "{v}"),
            None => write!(f, "(no active user)"),
        }
    }
}