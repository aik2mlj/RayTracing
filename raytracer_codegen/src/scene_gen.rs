extern crate proc_macro;
use crate::vec3::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::cmp::Ordering;
// use syn::DeriveInput;

struct SimpleOb {
    pub bounding_box_min: Vec3,
    // pub bounding_box_max: Vec3,
    pub code: TokenStream,
}

fn box_x_compare(a: &SimpleOb, b: &SimpleOb) -> Ordering {
    a.bounding_box_min
        .x
        .partial_cmp(&b.bounding_box_min.x)
        .unwrap()
}
fn box_y_compare(a: &SimpleOb, b: &SimpleOb) -> Ordering {
    a.bounding_box_min
        .y
        .partial_cmp(&b.bounding_box_min.y)
        .unwrap()
}
fn box_z_compare(a: &SimpleOb, b: &SimpleOb) -> Ordering {
    a.bounding_box_min
        .z
        .partial_cmp(&b.bounding_box_min.z)
        .unwrap()
}

fn bvh_build(objects: &mut Vec<SimpleOb>, start: usize, end: usize) -> TokenStream {
    let axis = rand::random::<usize>() % 3;
    let comparator = match axis {
        0 => box_x_compare,
        1 => box_y_compare,
        _ => box_z_compare,
    };
    // objects.sort_by(|a, b| comparator(a, b));
    let span = end - start;
    let mut left = &objects[start];
    let mut right = &objects[start];

    if span == 1 {
        let code = objects[start].code.clone();
        quote! {
            #code
        }
    } else if span == 2 {
        if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
            right = &objects[start + 1];
        } else {
            left = &objects[start + 1];
        }
        let left = left.code.clone();
        let right = right.code.clone();
        quote! {
            Arc::new(BVHNodeStatic::construct(#left, #right, 0.0, 1.0))
        }
    } else {
        let objects_slice = &mut objects[start..end]; // mutable slice
        objects_slice.sort_by(|a, b| comparator(a, b)); // sort the slice

        let mid = (start + end) >> 1; // half divide and recurse
        let left = bvh_build(objects, start, mid);
        let right = bvh_build(objects, mid, end);
        quote! {
            Arc::new(BVHNodeStatic::construct(#left, #right, 0.0, 1.0))
        }
    }
}

pub fn build_static_scenes() -> proc_macro::TokenStream {
    let mut objects = vec![];
    let checker = quote! {
        Arc::new(CheckerTexture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        ))
    };
    let earth_texture = quote! {
        Arc::new(ImageTexture::new("input/yyu2.jpg"))
    };
    objects.push(SimpleOb {
        bounding_box_min: Vec3::new(-1000.0, -2000.0, -1000.0),
        code: quote! {
            Arc::new(Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                mat_ptr: Arc::new(Lambertian::new_from_texture(#checker)),
            })
        },
    });
    let radius: f64 = 0.2;

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            let (x, y, z) = (center.x, center.y, center.z);
            let bounding_box_min = center - Vec3::new(radius, radius, radius);

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.2 {
                    // diffuse
                    objects.push(SimpleOb {
                        bounding_box_min,
                        code: quote! {
                            Arc::new(Sphere::new(
                                Vec3::new(#x, #y, #z),
                                #radius,
                                Arc::new(Lambertian::new(Vec3::rand(0.0, 1.0).elemul(Vec3::rand(0.0, 1.0))))
                            ))
                        }
                    });
                } else if choose_mat < 0.5 {
                    // metal
                    objects.push(SimpleOb {
                        bounding_box_min,
                        code: quote! {
                            Arc::new(Sphere::new(
                                Vec3::new(#x, #y, #z),
                                #radius,
                                Arc::new(Metal::new(Vec3::rand(0.5, 1.0), random_f64(0.0, 0.5)))
                            ))
                        },
                    });
                } else if choose_mat < 0.8 {
                    // glass
                    objects.push(SimpleOb {
                        bounding_box_min,
                        code: quote! {
                            Arc::new(Sphere::new(
                                Vec3::new(#x, #y, #z),
                                #radius,
                                Arc::new(Dielectric::new(1.5))
                            ))
                        },
                    });
                } else {
                    // diffuse light
                    objects.push(SimpleOb {
                        bounding_box_min,
                        code: quote! {
                            Arc::new(Sphere::new(
                                Vec3::new(#x, #y, #z),
                                #radius,
                                Arc::new(DiffuseLight::new(Vec3::rand(0.0, 1.0), 2.0))
                            ))
                        },
                    });
                }
            }
        }
    }
    objects.push(SimpleOb {
        bounding_box_min: Vec3::new(-1.7, 0.0, -1.7),
        code: quote! {
            Arc::new(Sphere {
                center: Vec3::new(0.0, 1.7, 0.0),
                radius: 1.7,
                mat_ptr: Arc::new(DiffuseLight::new_from_texture(#earth_texture, 1.0)),
            })
        },
    });
    objects.push(SimpleOb {
        bounding_box_min: Vec3::new(-6.0, 0.0, -1.0),
        code: quote! {
            Arc::new(Sphere {
                center: Vec3::new(-5.0, 1.0, 0.0),
                radius: 1.0,
                mat_ptr: Arc::new(Dielectric::new(1.5)),
            })
        },
    });
    objects.push(SimpleOb {
        bounding_box_min: Vec3::new(4.0, 0.0, -1.0),
        code: quote! {
            Arc::new(Sphere {
                center: Vec3::new(5.0, 1.0, 0.0),
                radius: 1.0,
                mat_ptr: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
            })
        },
    });
    // for ob in &objects {
    //     println!("{}", ob.code);
    // }

    let len = objects.len();
    let bvh_code = bvh_build(&mut objects, 0, len);
    let code = quote! {
        pub fn static_scene() -> HitTableList {
            let mut objects = HitTableList::default();
            objects.add(#bvh_code);
            objects
        }
    };
    code.into()
}
