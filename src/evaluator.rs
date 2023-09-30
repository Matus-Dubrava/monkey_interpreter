use crate::ast::Node;
use crate::object::Object;

pub fn eval(node: Box<dyn Node>) -> Object {
    let res = node.eval();

    return res;
}
