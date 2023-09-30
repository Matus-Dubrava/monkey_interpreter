use std::any::Any;

pub enum ObjectType {
    INTEGER_OBJ,
    BOOLEAN_OBJ,
    FLOAT_OBJ,
    NULL_OBJ,
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
            Self::INTEGER_OBJ => "INTEGER".to_string(),
            Self::BOOLEAN_OBJ => "BOOLEAN".to_string(),
            Self::FLOAT_OBJ => "FLOAT".to_string(),
            Self::NULL_OBJ => "NULL".to_string(),
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
