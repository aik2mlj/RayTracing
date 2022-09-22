# PPCA raytracer

This is a ray tracing project for *Principle and Practice of Computer Algorithms* (CS1952) course.
We followed the tutorial series [Ray Tracing in One Weekend](https://raytracing.github.io/), and writed the code in Rust. To acquire extra points, I implemented several optimizations:
- [x] Reduce Contention: only clone `Arc` when creating a thread; change `Arc` into reference in other places.
- [x] Static Dispatch: use generics to reduce extra expenses.
- [x] Code Generation: use procedural macros in Rust to generate the scene code statically.
- [x] Advanced Features: add support for `Tranform`

Here are some photos rendered:
![earth_and_balls_2](https://user-images.githubusercontent.com/53085155/191780402-1157cdcc-9083-4802-abb1-ede07c3a6dca.png)
![book2_scene](https://user-images.githubusercontent.com/53085155/191780569-2292f20e-5f02-4097-a37e-63b65f8083b3.png)
![add_all_pdf](https://user-images.githubusercontent.com/53085155/191780630-10f122ac-b8b6-4762-839c-050bb5e142ee.png)
