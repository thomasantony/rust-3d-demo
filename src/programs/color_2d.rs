use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::common_funcs as cf;
use crate::log;

pub struct Color2D{
    program: WebGlProgram,
    rect_vertices_len: usize,
    rect_vertices_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D{
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(gl, crate::shaders::vertex::color_2d::SHADER, crate::shaders::fragment::color_2d::SHADER).unwrap();

        let vertices_rect: [f32; 12] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
            1., 1., // x, y
            0., 0., // x, y
            1., 0., // x, y
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location, 
            vertices_location + vertices_rect.len() as u32
        );

        let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        Self {
            // These are all variables that are used in the shaders
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            rect_vertices_len: vertices_rect.len(),
            rect_vertices_buffer: buffer_rect,
            program,
        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, 
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32
    ) {
        gl.use_program(Some(&self.program));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertices_buffer));
        // How many values per vertex (would be 3 for 3D)
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        // Set the array to the "position" attribute in the vertex shader
        // Note the difference between "attrib" and "uniform"
        gl.enable_vertex_attrib_array(0);
        
        // Assign values to the variables in the shaders
        gl.uniform4f(Some(&self.u_color), 
            0.0, 
            0.5, 
            0.4, 
            1.0
        );
        gl.uniform1f(Some(&self.u_opacity), 1.0);

        log(&format!("{} {} {} {}", (right - left), (top - bottom), canvas_width, canvas_height));
        let translation_mat = cf::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_mat = cf::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );
        // Order of multiplication is important
        let transform_mat = cf::mult_matrix_4(scale_mat, translation_mat);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);
        gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertices_len / 2) as i32);
    }
}
