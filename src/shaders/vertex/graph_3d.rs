pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute float aY;

    uniform mat4 uProjection;
    varying lowp vec4 vColor;
    
    void main() {
        // 4-dimenional helps with projection
        gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);
        if (aY > 0.0) {
            vColor = vec4(0.0, 5.0*aY, 0.0, 1.0);
        }else {
            vColor = vec4(-5.0*aY, 0.0, 0.0, 1.0);
        }
        
    }
"#;
