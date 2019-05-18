const SEPARATOR: char = '=';

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
        None => SEPARATOR,
        Some(c) => c,
    };

    let split: Vec<&str> = line.split(sep).collect();
    assert_eq!(split.len(), 2, "Invalid property line. Expected format: \"key{}value\"", sep);
    property.key = split.first().unwrap().trim();
    property.value = split.last().unwrap().trim();
    return property;
}

fn check_line(line: &str, separator: char) -> bool {
    let mut i = 0;
    line.chars().for_each(|c| {
        if c == separator {
            i += 1;
        }
    });
    if i != 1 {
        return false;
    }
    return true;
}

/// Attempt to parse a line, returns None if the following are true:
/// * line is empty
/// * comment is not None and line begins with comment
/// * separator is not None and line contains more than one separator
pub fn try_split<'a>(
    line: &'a str,
    separator: Option<char>,
    comment: Option<&'a str>,
) -> Option<Property<'a>> {
    if line.is_empty() {
        return None;
    }

    match comment {
        None => {}
        Some(c) => {
            if line.starts_with(c) {
                return None;
            }
        }
    }

    if separator == None && !check_line(line, SEPARATOR) {
        return None;
    } else {
        if !check_line(line, separator.unwrap()) {
            return None;
        }
    }

    return Some(split(line, separator));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let property: Property = Property::new();
        assert_eq!(property.key(), String::new());
        assert_eq!(property.value(), String::new());
    }

    #[test]
    fn test_init() {
        let property: Property = Property::init("foo", "bar");
        assert_eq!(property.key(), String::from("foo"));
        assert_eq!(property.value(), String::from("bar"));
    }

    #[test]
    fn test_split() {
        let expected: Property = Property::init("foo", "bar");
        assert_eq!(split("foo: bar", Some(':')), expected);
    }

    #[test]
    fn test_try_split() {
        assert_eq!(try_split("", None, None), None);
        assert_eq!(try_split("foo:bar:baz", Some(':'), None), None);
        assert_eq!(try_split("//foo:bar", Some(':'), Some("//")), None);

        let expected: Property = Property::init("foo", "bar");
        assert_eq!(try_split("foo:bar", Some(':'), Some("//")), Some(expected));
    }
}
