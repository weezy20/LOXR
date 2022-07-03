use std::borrow::Cow;

#[derive(Debug, Default, PartialEq)]
pub enum Value {
    Double(f64),
    Bool(bool),
    String(String),
    #[default]
    Nil,
}
impl Value {
    pub fn is_numeric(&self) -> Option<f64> {
        match self {
            Value::Double(d) => Some(*d),
            _ => None,
        }
    }
    pub fn is_string<'a>(&'a self) -> Option<Cow<'a, str>> {
        match self {
            Value::String(s) => Some(Cow::Borrowed(s)),
            _ => None,
        }
    }
    pub fn is_equal(&self, other: &Value) -> bool {
        self == other
    }
    /// Only false, and nil are falsey, rest everything else is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}
impl From<String> for Value {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Double(f)
    }
}
