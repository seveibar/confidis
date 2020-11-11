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
        if as_f64.is_some() {
            return as_f64.unwrap();
        } else {
            // Ok it's probably a promise?
            // let val_promise = Promise::from(js_func_ret_val);
            // let val_future: JsFuture = JsFuture::from(val_promise);

            // while val_future.poll(self: Pin<&mut Self>, cx: &mut Context)

            // return executor::block_on(val_future).unwrap().as_f64().unwrap();
            return 0.0;
        }
    }
}

// #[test]
// fn test_js_equalifier_1() {
//     let this = JsValue::null();
//     let f = js_sys::Function::new_no_args("return 0.5;".into());
//     assert_eq!(f.call0(&this).unwrap().as_f64().unwrap(), 0.6);
//     // JSEqualifier::new()
// }
