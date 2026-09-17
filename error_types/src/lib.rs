pub use chrono::{NaiveDate, Utc};

pub const USERNAME_EMPTY: &str = "Username is empty";
pub const PASSWORD_TOO_SHORT: &str = "Password should be at least 8 characters long";
pub const PASSWORD_INVALID_CHARS: &str =
    "Password should be a combination of ASCII numbers, letters and symbols";

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new<T: Into<String>>(
        field_name: &'static str,
        field_value: T,
        err: &'static str,
    ) -> FormError {
        FormError {
            form_values: (field_name, field_value.into()),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Form {
        Form { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), USERNAME_EMPTY));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                PASSWORD_TOO_SHORT,
            ));
        }

        let has_letters = self.password.chars().any(|c| c.is_alphabetic());
        let has_numbers = self.password.chars().any(|c| c.is_numeric());
        let has_symbols = self.password.chars().any(|c| !c.is_alphanumeric());

        if !(has_letters && has_numbers && has_symbols) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                PASSWORD_INVALID_CHARS,
            ));
        }

        Ok(())
    }
}
