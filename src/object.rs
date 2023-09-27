use std::any::Any;

pub enum ObjectType {
    INTEGER_OBJ,
    BOOLEAN_OBJ,
    FLOAT_OBJ,
    NULL_OBJ,
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

pub trait Object {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn get_type(&self) -> ObjectType {
        return ObjectType::INTEGER_OBJ;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        ObjectType::BOOLEAN_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Float {
    pub value: f64,
}

impl Object for Float {
    fn get_type(&self) -> ObjectType {
        ObjectType::FLOAT_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Null {}

impl Object for Null {
    fn get_type(&self) -> ObjectType {
        ObjectType::NULL_OBJ
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
