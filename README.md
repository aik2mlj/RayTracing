# PPCA raytracer

This is a ray tracing project for *Principle and Practice of Computer Algorithms* (CS1952) course.
We followed the tutorial series [Ray Tracing in One Weekend](https://raytracing.github.io/), and writed the code in Rust. To acquire extra points, I implemented several optimizations:
- [x] Reduce Contention: only clone `Arc` when creating a thread; change `Arc` into reference in other places.
- [x] Static Dispatch: use generics to reduce extra expenses.
- [x] Code Generation: use procedural macros in Rust to generate the scene code statically.
- [x] Advanced Features: add support for `Tranform`
