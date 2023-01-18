# rust-raytracer
Raytracer written in rust (WIP)

# 18/01/2023

Raytracer written in rust.
Renders arbitrary width, height scenes.
Renders ppm format image.
Camera defined using 

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
