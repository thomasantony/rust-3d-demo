- following https://www.youtube.com/watch?v=p7DtoeuDT5Y
- using pnpm instead of npm
- use webpack (latest version instead of v4) and then maybe switch to swc later

- webpack 5 requires extra stuff in config for wasm
    - https://github.com/rustwasm/wasm-pack/issues/835#issuecomment-772591665
    - also we end up with "pkg" instead of "dist"
- WebGL will be used (instead of WebGL2 or WebGPU)
    - learning curve
    - three.js and babylon are wrappers over webgl
    - maybe useful to transition to webgpu later

- WebGL has no knowledge of 3D
    - uses linalg to paint 3D into 2D
    - canvas is a "box", origin at center, -1 to 1 in vertical and horizontal directions
    - z-axis exists only for occulison

- Vertex shaders "squishes" triangles down to -1 to 1 range in 2D?
- Fragment shader colors the triangles. Single color or images or calculated
