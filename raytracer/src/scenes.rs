use crate::bvh::*;
use crate::hittable::*;
use crate::material::*;
use crate::shared_tools::*;
use crate::texture::*;
use crate::Vec3;
use raytracer_codegen::*;
use std::sync::Arc;

bvhnode_impl! {}

pub fn big_random_scene() -> HitTableList {
    let mut world = HitTableList::default();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let earth_texture = Arc::new(ImageTexture::new("input/yyu2.jpg"));
    let checker_material = Arc::new(Lambertian::from(checker as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: checker_material,
    }));

    let radius: f64 = 0.2;

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.2 {
                    // diffuse
                    let albedo = Vec3::rand(0.0, 1.0).elemul(Vec3::rand(0.0, 1.0));
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere {
                        center,
                        radius,
                        mat_ptr: sphere_material,
                    }));
                } else if choose_mat < 0.5 {
                    // metal
                    let albedo = Vec3::rand(0.5, 1.0);
                    let fuzz = random_f64(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere {
                        center,
                        radius,
                        mat_ptr: sphere_material,
                    }));
                } else if choose_mat < 0.8 {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere {
                        center,
                        radius,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    // diffuse light
                    let albedo = Vec3::rand(0.0, 1.0);
                    let sphere_material = Arc::new(DiffuseLight::new(albedo, 2.0));
                    world.add(Arc::new(Sphere {
                        center,
                        radius,
                        mat_ptr: sphere_material,
                    }))
                }
            }
        }
    }

    let material1 = Arc::new(DiffuseLight::new_from_texture(earth_texture, 1.0));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.7, 0.0),
        radius: 1.7,
        mat_ptr: material1,
    }));
    let material2 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-5.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material2,
    }));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere {
        center: Vec3::new(5.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material3,
    }));

    world
}

pub fn two_spheres() -> HitTableList {
    let mut world = HitTableList::default();

    // let checker = Arc::new(CheckerTexture::new(
    //     Vec3::new(0.2, 0.3, 0.1),
    //     Vec3::new(0.9, 0.9, 0.9),
    // ));
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let checker_material = Arc::new(Lambertian::from(pertext as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat_ptr: checker_material.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat_ptr: checker_material,
    }));
    world
}

pub fn one_ball() -> HitTableList {
    let mut world = HitTableList::default();

    // let checker = Arc::new(CheckerTexture::new(
    //     Vec3::new(0.2, 0.3, 0.1),
    //     Vec3::new(0.9, 0.9, 0.9),
    // ));
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let checker_material = Arc::new(Lambertian::from(pertext as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: checker_material.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: checker_material,
    }));
    // let mut tmp = make_spheres();
    // let tmp = make_BVHNode!(100);
    world
}

pub fn earth() -> HitTableList {
    let mut world = HitTableList::default();
    let earth_texture = Arc::new(ImageTexture::new("input/yyu2.jpg"));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        mat_ptr: earth_surface,
    }));
    world
}

pub fn former_three_ball_scene() -> HitTableList {
    let mut world = HitTableList::default();
    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: material_ground,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_center,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_left.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        mat_ptr: material_left,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_right,
    }));
    world
}

pub fn simple_light() -> HitTableList {
    let mut world = HitTableList::default();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let checker_material = Arc::new(Lambertian::from(checker as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: checker_material.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: checker_material,
    }));

    let difflight = Arc::new(DiffuseLight::new(Vec3::new(0.7, 0.8, 1.0), 4.0));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    world
}

pub fn book2_final_scene() -> HitTableList {
    let mut boxes1 = HitTableList::default();
    let ground = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Box::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HitTableList::default();
    objects.add(Arc::new(BVHNode::new(&mut boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0), 7.0));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    // let center1 = Vec3::new(400.0, 400.0, 200.0);
    // let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    objects.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0)),
    )));

    let glass = Arc::new(Dielectric::new(1.5));
    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        glass.clone(),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Arc::new(Isotropic::new_from_color(Vec3::new(0.2, 0.4, 0.9))),
    )));
    let boundary = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, glass));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Arc::new(Isotropic::new_from_color(Vec3::new(1.0, 1.0, 1.0))),
    )));

    let emat = Arc::new(Lambertian::new_from_texture(Arc::new(ImageTexture::new(
        "input/earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_from_texture(pertext)),
    )));

    let mut boxes2 = HitTableList::default();
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::rand(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::new(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

pub fn cornell_box() -> HitTableList {
    let mut world = HitTableList::default();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0), 15.0));

    // the walls
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    // boxes

    let aluminum = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
    // let checker = Arc::new(CheckerTexture::new(
    //     Vec3::new(0.2, 0.3, 0.1),
    //     Vec3::new(0.9, 0.9, 0.9),
    // ));

    let box1 = Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        aluminum,
    ));
    // let box1 = Arc::new(RotateZ::new(box1, 38.0));
    let box1 = Arc::new(RotateY::new(box1, 38.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    // let box1 = Arc::new(ConstantMedium::new(
    //     box1,
    //     0.01,
    //     Arc::new(Isotropic::new_from_texture(checker)),
    // ));
    world.add(box1);

    // let box2 = Arc::new(Box::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(165.0, 165.0, 165.0),
    //     white.clone(),
    // ));
    // // let box2 = Arc::new(RotateZ::new(box2, -30.0));
    // let box2 = Arc::new(RotateY::new(box2, -30.0));
    // let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    // // let box2 = Arc::new(ConstantMedium::new(
    // //     box2,
    // //     0.01,
    // //     Arc::new(Isotropic::new_from_color(Vec3::ones())),
    // // ));
    // world.add(box2);
    let glass = Arc::new(Dielectric::new(1.5));
    let ball = Arc::new(Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, glass));
    world.add(ball);

    world
}
