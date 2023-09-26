enum ObjectType {
    INTEGER_OBJ,
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            Self::INTEGER_OBJ => "INTEGER".to_string(),
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
