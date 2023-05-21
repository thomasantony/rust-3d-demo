use nalgebra::{Perspective3, Matrix4};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create program object"))?;

    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        vert_source
    ).unwrap();

    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        frag_source
    ).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}


fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    
    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    [
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., 0.,
        tx, ty, tz, 1.,
    ]
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16]
{
    [
        sx, 0., 0., 0.,
        0., sy, 0., 0.,
        0., 0., sz, 0.,
        0., 0., 0., 1.,
    ]
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16]
{
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return_var
}

pub fn get_position_grid_n_by_n(n: usize) -> (Vec<f32>, Vec<u16>)
{
    let n_plus_one = n + 1;
    // X, Y, Z
    // Make it more efficient by storing just x and z and computing y on the fly
    let mut positions: Vec<f32> = vec![0.; n_plus_one * n_plus_one * 3];
    let mut indices: Vec<u16> = vec![0; n * n * 6]; // 3 vertices per triangle, 2 triangles per square = 6 indices per square
 
    // WebGL display goes from -1 to 1, so the "width" is equal to 2
    let graph_layout_width: f32 = 2.;

    let square_size: f32 = graph_layout_width / (n as f32);

    for i in 0..n_plus_one {
        for j in 0..n_plus_one {
            // Convert from 2D to 1D array, * 3 to account for X, Y, Z
            // i,j coordinates are grid-cell positions. We store x,y,z coordinates for each vertex in the grid
            let start_pos = (i * n_plus_one + j) * 3; 
            positions[start_pos] = -1. + (j as f32) * square_size;
            positions[start_pos + 1] = 0.;
            positions[start_pos + 2] = -1. + (i as f32) * square_size; 

            // Don't add indices for the last row or column
            if i < n && j < n
            {
                let top_left = (i * n_plus_one + j) as u16;
                let bottom_left = top_left + n_plus_one as u16;

                let top_right = top_left + 1; // they are stored in a row-major order
                let bottom_right = bottom_left + 1;

                // Define counter-clockwise winding order vertices
                // for the two triangles that make up a square
                indices[(i * n + j) * 6] = top_left;
                indices[(i * n + j) * 6 + 1] = bottom_left;
                indices[(i * n + j) * 6 + 2] = top_right;

                indices[(i * n + j) * 6 + 3] = top_right;
                indices[(i * n + j) * 6 + 4] = bottom_left;
                indices[(i * n + j) * 6 + 5] = bottom_right;
            }
        }
    }
    (positions, indices)
}


pub struct Matrices3D {
    pub normals_rotation: [f32; 16],
    pub projection: [f32; 16],
}
pub fn get_3d_matrices(bottom: f32,
    top:f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
    rotation_angle_x_axis: f32, rotation_angle_y_axis: f32)
 -> Matrices3D
 {
    use crate::constants as c;
    let mut return_var: Matrices3D = Matrices3D {
        normals_rotation: [0.; 16],
        projection: [0.; 16],
    };

    let rotate_x_mat : [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, rotation_angle_x_axis.cos(), -rotation_angle_x_axis.sin(), 0.0,
        0.0, rotation_angle_x_axis.sin(), rotation_angle_x_axis.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let rotate_y_mat: [f32; 16] = [
        rotation_angle_y_axis.cos(), 0.0, rotation_angle_y_axis.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -rotation_angle_y_axis.sin(), 0.0, rotation_angle_y_axis.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let rotation_matrix = mult_matrix_4(rotate_x_mat, rotate_y_mat);

    let aspect_ratio = canvas_width / canvas_height;
    let x_range = right - left;
    let y_range = top - bottom;
    let scale_x = x_range / canvas_width;
    let scale_y = y_range / canvas_height;
    let scale = scale_y;

    let translate_mat = translation_matrix(
        -1. + scale_x + 2. * left / canvas_width,
        -1. + scale_y + 2. * bottom / canvas_height,
        c::Z_PLANE,
    );

    let scale_mat = scaling_matrix(
        scale,
        scale,
        0.,
    );
    let rotation_scale = mult_matrix_4(rotation_matrix, scale_mat);
    let combined_transform = mult_matrix_4(rotation_scale, translate_mat);
    let perspective_mat_tmp: Perspective3<f32> = Perspective3::new(
        aspect_ratio,
        c::FIELD_OF_VIEW,
        c::Z_NEAR,
        c::Z_FAR,
    );
    let mut perspective: [f32; 16] = [0.; 16];
    perspective.copy_from_slice(perspective_mat_tmp.as_matrix().as_slice());
    
    return_var.projection =  mult_matrix_4(combined_transform, perspective);

// Assign rotation_matrix to normals_rotation
    let normal_matrix = Matrix4::new(
        rotation_matrix[0], rotation_matrix[1], rotation_matrix[2], rotation_matrix[3],
        rotation_matrix[4], rotation_matrix[5], rotation_matrix[6], rotation_matrix[7],
        rotation_matrix[8], rotation_matrix[9], rotation_matrix[10], rotation_matrix[11],
        rotation_matrix[12], rotation_matrix[13], rotation_matrix[14], rotation_matrix[15],
    );
    match normal_matrix.try_inverse() {
        Some(inv) => {
            return_var.normals_rotation.copy_from_slice(inv.as_slice());
        }, 
        None => {}
    };
    return_var
}


pub fn get_updated_y_values(curr_time: f32) -> Vec<f32>
{
    use crate::constants as c;
    let points_per_row = c::GRID_SIZE + 1;
    let mut y_vals: Vec<f32> = vec![0.; points_per_row * points_per_row];
    
    let half_width: f32 = points_per_row as f32 / 2.0;
    let frequency_scale = 2. * std::f32::consts::PI;
    let y_scale = 0.15;

    let sin_offset = curr_time * 0.001;
 
    for z in 0..points_per_row
    {
        for x in 0..points_per_row
        {
            let index = z * points_per_row + x;

            let scaled_x =  frequency_scale * (x as f32 - half_width)/half_width;
            let scaled_z =  frequency_scale * (z as f32 - half_width)/half_width;
            y_vals[index] = y_scale * ((scaled_x*scaled_x + scaled_z*scaled_z).sqrt() + sin_offset).sin();
        }
    }
    y_vals
}

pub fn get_grid_normals(grid_size: usize, y_vals: &[f32]) -> Vec<f32>
{
    let points_per_row = grid_size + 1;
    let graph_layout_width: f32 = 2.;
    let square_size = graph_layout_width / (grid_size as f32);
    let mut normals: Vec<f32> = vec![0.; points_per_row * points_per_row * 3];

    for i in 0..points_per_row
    {
        for j in 0..points_per_row
        {
            let y_index_a = i * points_per_row + j;
            let return_var_start_pos = y_index_a * 3;

            if i == grid_size || j == grid_size
            {
                // "up" is the default at the edge ([0, 1, 0])
                normals[return_var_start_pos + 1] = 1.0;
            }else{
                // Get normal vector of triangle defined by three points
                let y_index_b = y_index_a + points_per_row;  // down
                let y_index_c = y_index_a + 1;               // right

                let x_val_1 = j as f32 * square_size;
                let x_val_2 = x_val_1 + square_size;

                let z_val_1 = i as f32 * square_size;
                let z_val_2 = z_val_1 + square_size;
                
                let normal_vec = get_normal_vec(
                    x_val_1, y_vals[y_index_a], z_val_1,
                    x_val_1, y_vals[y_index_b], z_val_2,
                    x_val_2, y_vals[y_index_c], z_val_1,
                ); 
                normals[return_var_start_pos] = normal_vec.0;
                normals[return_var_start_pos + 1] = normal_vec.1;
                normals[return_var_start_pos + 2] = normal_vec.2;
            }
        }
    }
    normals
}
pub fn get_normal_vec(point_a_x: f32, point_a_y: f32, point_a_z: f32,
    point_b_x: f32, point_b_y: f32, point_b_z: f32,
    point_c_x: f32, point_c_y: f32, point_c_z: f32) -> (f32, f32, f32)
{
    // Compute normal vector of triangle defined by three points
    // https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal
    let u_x = point_b_x - point_a_x;
    let u_y = point_b_y - point_a_y;
    let u_z = point_b_z - point_a_z;
    
    let v_x = point_c_x - point_a_x;
    let v_y = point_c_y - point_a_y;
    let v_z = point_c_z - point_a_z;

    // Cross product
    let normal_x = u_y * v_z - u_z * v_y;
    let normal_y = u_z * v_x - u_x * v_z;
    let normal_z = u_x * v_y - u_y * v_x;

    // Normalize
    let normal_length = (normal_x * normal_x + normal_y * normal_y + normal_z * normal_z).sqrt();
    (normal_x / normal_length, normal_y / normal_length, normal_z / normal_length)
}
