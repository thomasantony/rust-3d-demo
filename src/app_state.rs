use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let min_height_width = canvas_height.min(canvas_width);
    let display_size = 0.9 * min_height_width;
    let half_display_size = display_size / 2.;
    let half_canvas_height = canvas_height / 2.;
    let half_canvas_width = canvas_width / 2.;  
    
    // Center the display area in canvas
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new (AppState {
        time,
        canvas_height,
        canvas_width,
        control_bottom: half_canvas_height - half_display_size,
        control_top: half_canvas_height + half_display_size,
        control_left: half_canvas_width - half_display_size,
        control_right: half_canvas_width + half_display_size,
        ..*data.clone()
    });
}

pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub control_top: f32,
    pub control_bottom: f32,
    pub control_left: f32,
    pub control_right: f32,
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rotation_angle_x_axis: f32,
    pub rotation_angle_y_axis: f32,
    pub time: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0.,
            canvas_width: 0.,
            control_top: 0.,
            control_bottom: 0.,
            control_left: 0.,
            control_right: 0.,
            mouse_down: false,
            mouse_x: 0.,
            mouse_y: 0.,
            rotation_angle_x_axis: -0.5,
            rotation_angle_y_axis: 0.5,
            time: 0.,
        }
    }
}

pub fn update_mouse_down(x:f32, y: f32, mouse_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new (AppState {
        mouse_down,
        mouse_x: x,
        // Flip y-axis to match WebGL coordinates
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    });
}

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();

    // Flip y-axis to match WebGL coordinates
    let mouse_y =  data.canvas_height - y;
    let delta_x = x - data.mouse_x;
    let delta_y = mouse_y - data.mouse_y;

    // Motion in x-axis is rotation around y-axis
    let rotation_x_delta = if data.mouse_down {
        std::f32::consts::PI * delta_y / data.canvas_width 
    } else {
        0.
    };

    // Motion in y-axis is rotation around x-axis
    let rotation_y_delta = if data.mouse_down {
        - std::f32::consts::PI * delta_x / data.canvas_width 
    } else {
        0.
    };
    *data = Arc::new (AppState {
        mouse_x: x,
        mouse_y,
        rotation_angle_x_axis: data.rotation_angle_x_axis + rotation_x_delta,
        rotation_angle_y_axis: data.rotation_angle_y_axis + rotation_y_delta,
        ..*data.clone()
    });
}
