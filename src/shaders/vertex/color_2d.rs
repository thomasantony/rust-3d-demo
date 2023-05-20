// Shaders are programs that are defined as strings
pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    uniform mat4 uTransform;
    
    void main() {
        gl_Position = uTransform * aPosition;
    }
"#;
