extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use confidis::Graph;
use confidis::command::{Command}

let mut Graph g = Graph::new();

// #[wasm_bindgen]
// fn 