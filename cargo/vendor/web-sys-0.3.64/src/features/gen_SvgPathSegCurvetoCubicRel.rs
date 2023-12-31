#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoCubicRel , typescript_type = "SVGPathSegCurvetoCubicRel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoCubicRel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub type SvgPathSegCurvetoCubicRel;
    # [wasm_bindgen (structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x)]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn x(this: &SvgPathSegCurvetoCubicRel) -> f32;
    # [wasm_bindgen (structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x)]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn set_x(this: &SvgPathSegCurvetoCubicRel, value: f32);
    # [wasm_bindgen (structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y)]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn y(this: &SvgPathSegCurvetoCubicRel) -> f32;
    # [wasm_bindgen (structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y)]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn set_y(this: &SvgPathSegCurvetoCubicRel, value: f32);
    # [wasm_bindgen (structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x1)]
    #[doc = "Getter for the `x1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn x1(this: &SvgPathSegCurvetoCubicRel) -> f32;
    # [wasm_bindgen (structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x1)]
    #[doc = "Setter for the `x1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn set_x1(this: &SvgPathSegCurvetoCubicRel, value: f32);
    # [wasm_bindgen (structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y1)]
    #[doc = "Getter for the `y1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn y1(this: &SvgPathSegCurvetoCubicRel) -> f32;
    # [wasm_bindgen (structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y1)]
    #[doc = "Setter for the `y1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn set_y1(this: &SvgPathSegCurvetoCubicRel, value: f32);
    # [wasm_bindgen (structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x2)]
    #[doc = "Getter for the `x2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn x2(this: &SvgPathSegCurvetoCubicRel) -> f32;
    # [wasm_bindgen (structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x2)]
    #[doc = "Setter for the `x2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn set_x2(this: &SvgPathSegCurvetoCubicRel, value: f32);
    # [wasm_bindgen (structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y2)]
    #[doc = "Getter for the `y2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn y2(this: &SvgPathSegCurvetoCubicRel) -> f32;
    # [wasm_bindgen (structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y2)]
    #[doc = "Setter for the `y2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*"]
    pub fn set_y2(this: &SvgPathSegCurvetoCubicRel, value: f32);
}
