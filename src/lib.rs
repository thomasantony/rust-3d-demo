use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod shaders;
mod programs;
mod common_funcs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub struct Client {
    gl: GL,

}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Client {
            gl
        }
    }

    pub fn update(&self, _time: f32, _height:f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        Ok(())
    }
}
