pub const FIELD_OF_VIEW: f32 = 45. * std::f32::consts::PI / 180.; // in radians
pub const GRID_SIZE: usize = 100;

pub const Z_NEAR: f32 = 0.1;
pub const Z_FAR: f32 = 100.0;
// pub const Z_PLANE: f32 = -1.0 / (FIELD_OF_VIEW/2.0).tan();
pub const Z_PLANE: f32 = -2.414213562373095;
// pub const Z_PLANE: f32 = -2.414213562373095 - 1.7673;
