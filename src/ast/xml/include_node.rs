use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;

#[derive(Clone)]
pub struct IncludeNode {
    pub refid: String,
    pub childs: Vec<NodeType>,
}

impl Ast for IncludeNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder,arg_array:&mut Vec<Value>) -> Result<String, String> {
        return do_child_nodes(&self.childs, env,holder,arg_array);
    }
}

impl SqlNodePrint for IncludeNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<include " + "refid=\"" + self.refid.as_str() + "\"" + " >";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</include>";
        return result;
    }
}