use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::common_funcs as cf;
use crate::log;

pub struct Color2DGradient{
    program: WebGlProgram,
    index_count: i32,
    rect_vertices_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2DGradient{
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(gl, 
            crate::shaders::vertex::color_2d_gradient::SHADER, 
            crate::shaders::fragment::color_2d_gradient::SHADER
        ).unwrap();

        // Be more efficient by storing just the indices of the vertices
        let vertices_rect: [f32; 8] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
            1., 0., // x, y
        ];
        // This *must* be marked as u16 or it will not work
        let indices_rect: [u16; 6] = [
            0, 1, 2,
            2, 1, 3,
        ];

        // Required to store data on the GPU
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


        // Required to store data on the GPU
        let indices_memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_location = indices_rect.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory).subarray(
            indices_location,
            indices_location + indices_rect.len() as u32
        );
        let buffer_indices_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        // Note how we now say ELEMENT_ARRAY_BUFFER instead of ARRAY_BUFFER
        // This is how WebGL knows to use the indices
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices_rect));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);

        Self {
            // These are all variables that are used in the shaders
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            index_count: indices_rect.len() as i32,
            // Dynamically filled in every render instead of being hardcoded
            color_buffer: gl.create_buffer().ok_or("failed to create buffer").unwrap(),
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
        // Set the array to the "position" attribute in the vertex shader
        // Note the difference between "attrib" and "uniform"
        gl.enable_vertex_attrib_array(0);
        // How many values per vertex (would be 3 for 3D)
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        // Attribute position 1
        gl.enable_vertex_attrib_array(1);
        // Four values per vertex (RGBA)
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        
        // The color for each vertex (intermediate pixels are interpolated)
        let colors: [f32; 16] = [
            1., 0., 0., 1., // red
            0., 1., 0., 1., // green
            0., 0., 1., 1., // blue
            1., 1., 1., 1., // white
        ];
        // Pass the color data to the GPU
        let colors_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let colors_location = colors.as_ptr() as u32 / 4;
        let colors_array = js_sys::Float32Array::new(&colors_memory_buffer).subarray(
            colors_location,
            colors_location + colors.len() as u32
        );
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors_array, GL::DYNAMIC_DRAW);
        
        gl.uniform1f(Some(&self.u_opacity), 1.0);

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
        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}
