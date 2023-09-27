#[cfg(test)]
mod evaluator_test {
    use monkey_interpreter::object::*;

    fn get_and_assert_integer_object(obj: Box<dyn Object>, value: i64) {
        if let Some(int) = obj.as_any().downcast_ref::<Integer>() {
            assert_eq!(
                int.value, value,
                "expected ingeger value `{}`, got=`{}`",
                value, int.value
            )
        } else {
            panic!("Object is not Ingeger");
        }
    }

    fn get_and_assert_float_object(obj: Box<dyn Object>, value: f64) {
        if let Some(float) = obj.as_any().downcast_ref::<Float>() {
            assert_eq!(
                float.value, value,
                "expected float value `{}`, got=`{}`",
                value, float.value
            )
        } else {
            panic!("Object is not Float");
        }
    }

    fn get_and_assert_boolean_object(obj: Box<dyn Object>, value: bool) {
        if let Some(boolean) = obj.as_any().downcast_ref::<Boolean>() {
            assert_eq!(
                boolean.value, value,
                "expected boolean value `{}`, got=`{}`",
                value, boolean.value
            )
        } else {
            panic!("Object is not Boolean");
        }
    }

    fn get_and_assert_null_object(obj: Box<dyn Object>) {
        let null = obj.as_any().downcast_ref::<Null>();
        assert!(null.is_some(), "Object is not Nll");
    }
}
