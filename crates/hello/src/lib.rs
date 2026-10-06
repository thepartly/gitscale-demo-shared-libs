//! The greeting every service gives.

/// How a greeting is said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Plain,
    Shout,
}

/// The greeting for `name`.
pub fn greet(name: &str, style: Style) -> String {
    let greeting = format!("Hello, {}!", name);
    match style {
        Style::Plain => greeting,
        Style::Shout => greeting.to_uppercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_greets_by_name() {
        assert_eq!(greet("Ada", Style::Plain), "Hello, Ada!");
    }

    #[test]
    fn shout_is_upper_case() {
        assert_eq!(greet("Ada", Style::Shout), "HELLO, ADA!");
    }
}
