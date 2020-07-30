use crate::hittable::*;
use crate::material::*;
use crate::shared_tools::*;
use crate::texture::*;
use crate::Vec3;
use std::sync::Arc;

pub fn big_random_scene() -> HitTableList {
    let mut world = HitTableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let earth_texture = Arc::new(ImageTexture::new("input/yyu2.jpg"));
    let checker_material = Arc::new(Lambertian::from(checker.clone() as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: checker_material.clone(),
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
    let mut world = HitTableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let checker_material = Arc::new(Lambertian::from(checker.clone() as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat_ptr: checker_material.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat_ptr: checker_material.clone(),
    }));
    world
}

pub fn one_ball() -> HitTableList {
    let mut world = HitTableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let checker_material = Arc::new(Lambertian::from(checker.clone() as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: checker_material.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: checker_material.clone(),
    }));
    world
}

pub fn earth() -> HitTableList {
    let mut world = HitTableList::new();
    let earth_texture = Arc::new(ImageTexture::new("input/yyu2.jpg"));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        mat_ptr: earth_surface.clone(),
    }));
    world
}

pub fn former_three_ball_scene() -> HitTableList {
    let mut world = HitTableList::new();
    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: material_ground.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_center.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_left.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        mat_ptr: material_left.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_right.clone(),
    }));
    world
}

pub fn simple_light() -> HitTableList {
    let mut world = HitTableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let checker_material = Arc::new(Lambertian::from(checker.clone() as Arc<dyn Texture>));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: checker_material.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: checker_material.clone(),
    }));

    let difflight = Arc::new(DiffuseLight::new(Vec3::new(0.7, 0.8, 1.0), 4.0));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    world
}

pub fn cornell_box() -> HitTableList {
    let mut world = HitTableList::new();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(1.0, 1.0, 1.0), 15.0));

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
    world.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.add(Arc::new(Box::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 15.0, 330.0),
        white.clone(),
    )));
    world.add(Arc::new(Box::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white.clone(),
    )));

    world
}
