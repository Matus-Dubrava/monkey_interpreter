use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Float,
    Null,
    ReturnValue,
    Error,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::Float => write!(f, "FLOAT"),
            ObjectType::Null => write!(f, "NULL"),
            ObjectType::ReturnValue => write!(f, "RETURN_VALUE"),
            ObjectType::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Integer(val) => format!("{}", val),
            Object::Boolean(val) => format!("{}", val),
            Object::Float(val) => format!("{}", val),
            Object::Null => "null".into(),
            Object::ReturnValue(val) => format!("{}", val.to_string()),
            Object::Error(val) => format!("{}", val),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> ObjectType {
        match self {
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Integer(_) => ObjectType::Integer,
            Object::Float(_) => ObjectType::Float,
            Object::Null => ObjectType::Null,
            Object::ReturnValue(_) => ObjectType::ReturnValue,
            Object::Error(_) => ObjectType::Error,
        }
    }
}
