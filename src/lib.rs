extern crate wasm_bindgen;

pub mod cluster;
pub mod command;
pub mod equalifier;
pub mod graph;

use command::Command;
use equalifier::JSEqualifier;
use graph::Graph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct GraphJS {
    g: Box<Graph>,
}

#[wasm_bindgen]
impl GraphJS {
    pub fn new() -> Self {
        GraphJS {
            g: Box::new(Graph::new()),
        }
    }

    pub fn new_with_equalifier(js_func: &js_sys::Function) -> Self {
        let equalifier = Box::new(JSEqualifier::new(js_func));
        GraphJS {
            g: Box::new(Graph::new_with_equalifier(equalifier)),
        }
    }

    pub fn execute_command(&mut self, cmd_string: &str) -> Result<JsValue, JsValue> {
        let cmd = Command::from(cmd_string);
        if cmd.is_err() {
            return Err(JsValue::from_str(&cmd.err().unwrap()));
        }
        let res = self.g.execute_command(&cmd.unwrap());
        if res.is_err() {
            let err_val = res.err().unwrap();
            return Err(JsValue::from_str(&err_val));
        }
        match JsValue::from_serde(&res.unwrap()) {
            Ok(v) => Ok(v),
            Err(v) => Err(JsValue::from_str("Error parsing command response")),
        }
    }
}
