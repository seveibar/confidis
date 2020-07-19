extern crate wasm_bindgen;

pub mod cluster;
pub mod command;
pub mod equalifier;
pub mod graph;

use command::{Command, CommandResponse};
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
    pub fn execute_command(&mut self, cmd_string: &str) -> JsValue {
        let cmd = Command::from(cmd_string).unwrap();
        let res = self.g.execute_command(&cmd).unwrap();
        JsValue::from_serde(&res).unwrap()
    }
}
