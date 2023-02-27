# Toy Raytracer
with inspiration from the "Ray Tracing in One Weekend" book series by Peter Shirley   

This is a CPU based raytracer with some basic support for 3D models, and light based rendering methods  

This project is a rust port with additional addons.  

## Build Instructions
This will build the demo scene

Ensure you are on the nightly build, this is a prerequisite for SIMD instruction sets
```
rustup default nightly
```

Cloning, building, and running
```
git clone https://github.com/canarado/rust-rt

cd rust-rt

cargo b -r

// windows
.\target\release\raytracer.exe > image.ppm

// linux
./target/release/raytracer > image.ppm
```

## Custom Scenes
Until there is a gui to do so, or a configuration file, something alone those lines,
you can add objects with materials into the scene in the code in the appropriate section in `main.rs`

## TODO
- optimize vector math with SIMD
- add config file for custom scenes
- add rayon for multiple threads rendering