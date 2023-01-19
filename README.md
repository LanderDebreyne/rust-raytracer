# rust-raytracer
Raytracer written in rust (WIP)

# 19/01/2023

Added rectangle primitive.  
Added area lights.  
Rendering loop now writes to png (in parallel). (using image library, can output to other encoders).  

Sample image of diffuse and reflective sphere in a cornell box.  
Rendered using 1000 samples per pixel path tracing, max_depth of 10 in 800*800px.    
![diffuse and reflective sphere in a cornell box](https://github.com/LanderDebreyne/rust-raytracer/blob/main/19_01_23.png?raw=true)

# 18/01/2023

Raytracer written in rust.
Renders arbitrary width, height scenes.
Renders ppm format image.  

Sample image of diffuse sphere and reflective triangle and sphere on white plane, left green plane and white environment.  
Rendered using 100 samples per pixel, max_depth of 50 in 1920*1080 px.
![ diffuse sphere and reflective triangle and sphere on white plane, left green plane and white environment](https://github.com/LanderDebreyne/rust-raytracer/blob/main/18_01_23.png?raw=true)

Jittered sampling.  
No lights, no shadowrays yet.  
Sphere, triangle, plane primitives.
Lambertian and metal (reflective) materials.
Only supports pinhole camera.
Data-parallellism on the main renderloop using rayon.

run using command:

```
cargo run --release
```

Supports compiling to wasm by uncommenting the getrandom dependency in cargo.toml using

```
cargo build --target wasm32-unknown-unknown --release
```

Did not test running this...
