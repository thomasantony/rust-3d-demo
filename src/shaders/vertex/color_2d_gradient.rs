// Shaders are programs that are defined as strings
pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec4 aColor;
    uniform mat4 uTransform;

    // This is used to share stuff to fragment shader
    // The variable with the same name in the fragment shader will have the same value
    varying lowp vec4 vColor;
    
    void main() {
        vColor = aColor;
        gl_Position = uTransform * aPosition;
    }
"#;
