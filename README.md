# rust-raytracer
Raytracer written in rust (WIP)

# 21/01/23

Correct mistake in direct illumination.
Use Area of light / (light_distance * light_distance) instead of Area of light / (Area of Hemisphere with radius of light_distance).
This results in a brighter image.  

Corrected render (256 samples/pixel, 10 depth)
![diffuse and reflective sphere in a cornell box](https://github.com/LanderDebreyne/rust-raytracer/blob/main/21_1_23.png?raw=true)

Incorrect render  
![diffuse and reflective sphere in a cornell box](https://github.com/LanderDebreyne/rust-raytracer/blob/main/20_1_23_render.png?raw=true)

# 20/01/23

Split rendering into direct and indirect illumination.  
Direct illumination samples random position on light source and checks visibility.  

Lights, direct and indirect illumination, rendered using 256 samples per pixel path tracing, max_depth of 10 in 800*800px.
![diffuse and reflective sphere in a cornell box](https://github.com/LanderDebreyne/rust-raytracer/blob/main/20_1_23_combined.png?raw=true)

Only light, rendered using 256 samples per pixel path tracing, max_depth of 1 in 800*800px.  
![diffuse and reflective sphere in a cornell box, only light rendered](https://github.com/LanderDebreyne/rust-raytracer/blob/main/20_1_23_light.png?raw=true)

Only direct illumination, rendered using 256 samples per pixel path tracing, max_depth of 1 in 800*800px.
![diffuse and reflective sphere in a cornell box, only direct illumination rendered](https://github.com/LanderDebreyne/rust-raytracer/blob/main/20_1_23_direct.png?raw=true)

Only indirect illumination, rendered using 256 samples per pixel path tracing, max_depth of 10 in 800*800px.
![diffuse and reflective sphere in a cornell box, only indirect illumination rendered](https://github.com/LanderDebreyne/rust-raytracer/blob/main/20_1_23_indirect.png?raw=true)

# 19/01/2023

Added rectangle primitive.  
Added area lights.  
Rendering loop now writes to png (in parallel). (using image library, can output to other encoders).  

Sample image of diffuse and reflective sphere in a cornell box.  
Rendered using 1000 samples per pixel path tracing, max_depth of 50 in 800*800px.    
![diffuse and reflective sphere in a cornell box](https://github.com/LanderDebreyne/rust-raytracer/blob/main/19_1_23.png?raw=true)
![diffuse and reflective sphere in a cornell box](https://github.com/LanderDebreyne/rust-raytracer/blob/main/19_1_23_2.png?raw=true)

# 18/01/2023

Raytracer written in rust.
Renders arbitrary width, height scenes.
Renders ppm format image.  

Sample image of diffuse sphere and reflective triangle and sphere on white plane, left green plane and white environment.  
Rendered using 100 samples per pixel, max_depth of 50 in 1920*1080 px.
![ diffuse sphere and reflective triangle and sphere on white plane, left green plane and white environment](https://github.com/LanderDebreyne/rust-raytracer/blob/main/18_1_23.png?raw=true)

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
