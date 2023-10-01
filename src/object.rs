pub enum Object {
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Null,
    ReturnValue(Box<Object>),
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Integer(val) => format!("{}", val),
            Object::Boolean(val) => format!("{}", val),
            Object::Float(val) => format!("{}", val),
            Object::Null => "null".into(),
            Object::ReturnValue(val) => format!("{}", val.to_string()),
        }
    }
}
