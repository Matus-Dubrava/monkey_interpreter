enum ObjectType {
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

trait Object {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

struct Integer {
    value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn get_type(&self) -> ObjectType {
        return ObjectType::INTEGER_OBJ;
    }
}

struct Boolean {
    value: bool,
}

impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        ObjectType::BOOLEAN_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

struct Float {
    value: f64,
}

impl Object for Float {
    fn get_type(&self) -> ObjectType {
        ObjectType::FLOAT_OBJ
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

struct Null {}

impl Object for Null {
    fn get_type(&self) -> ObjectType {
        ObjectType::NULL_OBJ
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}
