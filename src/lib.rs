#[derive(Debug, Copy, Clone)]
pub struct Property<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Property<'a> {
    pub fn new() -> Self {
        Property { key: "", value: "" }
    }
    pub fn init(key: &'a str, value: &'a str) -> Self {
        Property { key, value }
    }
    pub fn key(self) -> String {
        self.key.to_string()
    }
    pub fn value(self) -> String {
        self.value.to_string()
    }
}

impl<'a> PartialEq for Property<'a> {
    fn eq(&self, other: &Property) -> bool {
        self.key == other.key && self.value == other.value
    }
    fn ne(&self, other: &Property) -> bool {
        self.key != other.key || self.value != other.value
    }
}

/// Split a line based and return a property
///
/// line expects there to be only one separator
///
/// Default separator is '='
pub fn split(line: &str, separator: Option<char>) -> Property {
    let mut property: Property = Property::new();
    let sep: char = match separator {
        None => '=',
        Some(c) => c,
    };

    let split: Vec<&str> = line.split(sep).collect();
    assert_eq!(split.len(), 2, "Invalid property line. Expected format: \"key{}value\"", sep);
    property.key = split.first().unwrap().trim();
    property.value = split.last().unwrap().trim();
    return property;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let property: Property = Property::new();
        assert_eq!(property.key(), String::new());
        assert_eq!(property.value(), String::new());
    }

    #[test]
    fn init() {
        let property: Property = Property::init("foo", "bar");
        assert_eq!(property.key(), String::from("foo"));
        assert_eq!(property.value(), String::from("bar"));
    }

    #[test]
    fn empty() {
        let expected: Property = Property::init("foo", "bar");
        assert_eq!(split("foo: bar", Some(':')), expected);
    }
}
