const rust = import ('./pkg');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then(m => {
    if (!gl) {
        alert('Failed to initialize WebGL');
        return;
    }
    const rustClient = new m.Client();
    const initialTime = Date.now();

    const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
    let lastDrawTime = -1; // milliseconds

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;

            if (window.innerHeight != canvas.height || window.innerWidth != canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }
            let elapsedTime = currTime - initialTime;
            rustClient.update(elapsedTime, window.innerWidth, window.innerHeight);
            rustClient.render();
        }
        
    }
    requestAnimationFrame(render);
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;
    canvas.width = width;
    canvas.height = height;
    gl.viewport(0, 0, width, height);
});
