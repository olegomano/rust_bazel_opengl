#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AssignedNodesOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AssignedNodesOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub type AssignedNodesOptions;
}
impl AssignedNodesOptions {
    #[doc = "Construct a new `AssignedNodesOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn flatten(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("flatten"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for AssignedNodesOptions {
    fn default() -> Self {
        Self::new()
    }
}
