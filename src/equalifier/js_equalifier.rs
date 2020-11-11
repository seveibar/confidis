use crate::command::Answer;
use crate::equalifier::Equalifier;

use wasm_bindgen::prelude::*;

pub struct JSEqualifier {
    js_func: js_sys::Function,
}

impl JSEqualifier {
    pub fn new(js_func: &js_sys::Function) -> Self {
        JSEqualifier {
            js_func: js_func.clone(),
        }
    }
}

impl Equalifier for JSEqualifier {
    fn is_valid_answer(&self, _a: &Answer) -> bool {
        true
    }
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        let this = JsValue::null();
        return self
            .js_func
            .call2(
                &this,
                &JsValue::from_str(&a.content),
                &JsValue::from_str(&b.content),
            )
            .unwrap()
            .as_f64()
            .unwrap();
    }
}

// #[test]
// fn test_js_equalifier_1() {
//     let this = JsValue::null();
//     let f = js_sys::Function::new_no_args("return 0.5;".into());
//     assert_eq!(f.call0(&this).unwrap().as_f64().unwrap(), 0.6);
//     // JSEqualifier::new()
// }
