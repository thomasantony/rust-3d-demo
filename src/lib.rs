use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod shaders;
mod programs;
mod common_funcs;
mod app_state;
mod constants;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}


#[wasm_bindgen]
pub struct Client {
    gl: GL,
    _program_color_2d: programs::Color2D,
    _program_color_2d_gradient: programs::Color2DGradient,
    program_graph_3d: programs::Graph3D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Client {
            _program_color_2d: programs::Color2D::new(&gl),
            _program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            program_graph_3d: programs::Graph3D::new(&gl),
            gl,
        }
    }

    pub fn update(&self, time: f32, height:f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let curr_state = app_state::get_curr_state();
        // self._program_color_2d.render(
        //     &self.gl,
        //     curr_state.control_bottom,
        //     curr_state.control_top,
        //     curr_state.control_left,
        //     curr_state.control_right,
        //     curr_state.canvas_height,
        //     curr_state.canvas_width,
        // );
        // self.program_color_2d_gradient.render(
        //     &self.gl,
        //     curr_state.control_bottom + 20.,
        //     curr_state.control_top - 20.,
        //     curr_state.control_left + 20.,
        //     curr_state.control_right - 20.,
        //     curr_state.canvas_height,
        //     curr_state.canvas_width,
        // );
        self.program_graph_3d.render(
            &self.gl,
            curr_state.control_bottom + 20.,
            curr_state.control_top - 20.,
            curr_state.control_left + 20.,
            curr_state.control_right - 20.,
            curr_state.canvas_height,
            curr_state.canvas_width,
            curr_state.rotation_angle_x_axis,
            curr_state.rotation_angle_y_axis,
            &common_funcs::get_updated_y_values(curr_state.time),
        );
        Ok(())
    }
}
