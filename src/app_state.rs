use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new (AppState {
        time,
        canvas_height,
        canvas_width,
        ..*data.clone()
    });
}

pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub time: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0.,
            canvas_width: 0.,
            time: 0.,
        }
    }
}
