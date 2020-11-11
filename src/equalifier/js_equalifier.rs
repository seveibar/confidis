use crate::command::Answer;
use crate::equalifier::Equalifier;
use futures::executor;
use futures::future::Future;
use js_sys::Promise;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;

use wasm_bindgen::prelude::*;

pub struct JSEqualifier {
    js_func: Box<js_sys::Function>,
}

impl JSEqualifier {
    pub fn new(js_func: &js_sys::Function) -> Self {
        JSEqualifier {
            js_func: Box::new(js_func.clone()),
        }
    }
}

impl Equalifier for JSEqualifier {
    fn is_valid_answer(&self, _a: &Answer) -> bool {
        true
    }
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        let this = JsValue::null();
        let js_func_res = self.js_func.call2(
            &this,
            &JsValue::from_str(&a.content),
            &JsValue::from_str(&b.content),
        );
        let js_func_ret_val = js_func_res.unwrap();
        let as_f64 = js_func_ret_val.as_f64();
        return as_f64.unwrap();
    }
}
