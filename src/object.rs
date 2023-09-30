use std::any::Any;

pub enum ObjectType {
    IntegerObj,
    BooleanObj,
    FloatObj,
    NullOb,
}

pub enum Object {
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Null,
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            Self::IntegerObj => "INTEGER".to_string(),
            Self::BooleanObj => "BOOLEAN".to_string(),
            Self::FloatObj => "FLOAT".to_string(),
            Self::NullOb => "NULL".to_string(),
        }
    }
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Integer(val) => format!("{}", val),
            Object::Boolean(val) => format!("{}", val),
            Object::Float(val) => format!("{}", val),
            Object::Null => "null".into(),
        }
    }
}
