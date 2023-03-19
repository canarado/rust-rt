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
.\target\release\raytracer.exe

// linux
./target/release/raytracer
```

## Custom Scenes
Until there is a gui to do so, or a configuration file, something alone those lines,
you can add objects with materials into the scene in the code in the appropriate section in `main.rs`

## TODO
- [x] optimize vector math with SIMD
- [ ] add config file for custom scenes
- [x] add rayon for multiple threads rendering
- [x] output to png instead of ppm
- [ ] add program flags
    - [ ] output file name
    - [ ] file size
    - [ ] core count
    - [ ] sample rate
- [ ] GPU based rendering
- [ ] GUI preview + object config
- [ ] update functions to use one thread_rng per thread instead of random ones thrown about
- [ ] modularize code and move vital functions out of main