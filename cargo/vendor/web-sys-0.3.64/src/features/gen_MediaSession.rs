#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaSession , typescript_type = "MediaSession")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSession` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaSession;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaMetadata")]
    # [wasm_bindgen (structural , method , getter , js_class = "MediaSession" , js_name = metadata)]
    #[doc = "Getter for the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/metadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadata`, `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn metadata(this: &MediaSession) -> Option<MediaMetadata>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaMetadata")]
    # [wasm_bindgen (structural , method , setter , js_class = "MediaSession" , js_name = metadata)]
    #[doc = "Setter for the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/metadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadata`, `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_metadata(this: &MediaSession, value: Option<&MediaMetadata>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionPlaybackState")]
    # [wasm_bindgen (structural , method , getter , js_class = "MediaSession" , js_name = playbackState)]
    #[doc = "Getter for the `playbackState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/playbackState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `MediaSessionPlaybackState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn playback_state(this: &MediaSession) -> MediaSessionPlaybackState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionPlaybackState")]
    # [wasm_bindgen (structural , method , setter , js_class = "MediaSession" , js_name = playbackState)]
    #[doc = "Setter for the `playbackState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/playbackState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `MediaSessionPlaybackState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_playback_state(this: &MediaSession, value: MediaSessionPlaybackState);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    # [wasm_bindgen (method , structural , js_class = "MediaSession" , js_name = setActionHandler)]
    #[doc = "The `setActionHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setActionHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `MediaSessionAction`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_action_handler(
        this: &MediaSession,
        action: MediaSessionAction,
        handler: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "MediaSession" , js_name = setCameraActive)]
    #[doc = "The `setCameraActive()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setCameraActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_camera_active(this: &MediaSession, active: bool);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "MediaSession" , js_name = setMicrophoneActive)]
    #[doc = "The `setMicrophoneActive()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setMicrophoneActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_microphone_active(this: &MediaSession, active: bool);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "MediaSession" , js_name = setPositionState)]
    #[doc = "The `setPositionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setPositionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_position_state(this: &MediaSession);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaPositionState")]
    # [wasm_bindgen (method , structural , js_class = "MediaSession" , js_name = setPositionState)]
    #[doc = "The `setPositionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setPositionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`, `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_position_state_with_state(this: &MediaSession, state: &MediaPositionState);
}
