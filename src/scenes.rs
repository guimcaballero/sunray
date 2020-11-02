#[allow(unused_imports)]
use crate::{
    bvh::*,
    camera::*,
    hittable::{
        cube::*, cylinder::*, flip_face::*, medium::*, moving_sphere::*, pyramid::*, rectangle::*,
        rotate::*, sdf::*, sphere::*, translate::*, triangle::*, *,
    },
    hittable_list::*,
    material::*,
    perlin::*,
    texture,
    vec3::*,
};
use rand::Rng;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub enum Scene {
    CornellBox,
    CornellSmokes,
    SpaceDonut,
    Imagine,
    MengerSponge,
    MandelBulb,
    MandelBox,
    Knot,
    CornellMandelBox,
}

#[cfg(target_arch = "wasm32")]
pub fn generate_world(scene: Scene) -> World {
    match scene {
        Scene::CornellBox => cornell_box(),
        Scene::CornellSmokes => cornell_smokes(),
        Scene::SpaceDonut => space_dount(),
        Scene::MengerSponge => menger_sponge(),
        Scene::Imagine => imagine(),
        Scene::MandelBulb => mandelbulb(),
        Scene::MandelBox => mandelbox(),
        Scene::Knot => knot(),
        Scene::CornellMandelBox => cornell_mandelbox(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub enum Scene {
    Test,
    ManySpheres,
    TwoPerlinSpheres,
    Earth,
    LightRectangle,
    CornellBox,
    CornellSmokes,
    FinalScene,
    CustomScene,
    SpaceDonut,
    Imagine,
    MengerSponge,
    MandelBulb,
    MandelBox,
    Knot,
    CornellMandelBox,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_world(scene: Scene) -> World {
    match scene {
        Scene::Test => test(),
        Scene::ManySpheres => many_spheres(),
        Scene::TwoPerlinSpheres => two_perlin_spheres(),
        Scene::Earth => earth(),
        Scene::LightRectangle => light_rectangle(),
        Scene::CornellBox => cornell_box(),
        Scene::CornellSmokes => cornell_smokes(),
        Scene::FinalScene => final_scene(),
        Scene::CustomScene => custom_scene(),
        Scene::SpaceDonut => space_dount(),
        Scene::MengerSponge => menger_sponge(),
        Scene::Imagine => imagine(),
        Scene::MandelBulb => mandelbulb(),
        Scene::MandelBox => mandelbox(),
        Scene::Knot => knot(),
        Scene::CornellMandelBox => cornell_mandelbox(),
    }
}

pub struct World {
    pub hittables: HittableList,
    pub lights: HittableList,
    pub camera: Camera,

    pub background_color_top: Color,
    pub background_color_bottom: Color,

    // Image
    pub samples_per_pixel: u16,
    pub aspect_ratio: f32,
    pub image_width: u16,
    pub max_depth: u16,
}

impl Default for World {
    fn default() -> Self {
        let aspect_ratio = 3.0 / 2.0;

        let camera = Camera::new(
            Point::new(13., 2., 3.),
            Point::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            20.,
            aspect_ratio,
            0.,
            10.,
            0.0,
            1.0,
        );

        Self {
            hittables: HittableList::new(),
            lights: HittableList::new(),
            camera,

            background_color_top: Color::zeros(),
            background_color_bottom: Color::zeros(),

            samples_per_pixel: 100,
            aspect_ratio,
            image_width: 800,
            max_depth: 50,
        }
    }
}

fn test() -> World {
    let mut hittables = HittableList::new();

    hittables.add(box TracedSDF {
        sdf: box SDFRepetition {
            a: box SDFSphere {
                radius: 0.5,
                center: Point::from(0.),
            },
            repetition: Vec3::from(10.),
        },
        material: Material::DiffuseLight(Color::from(0.7)),
    });

    World {
        hittables,
        samples_per_pixel: 100,
        ..World::default()
    }
}

fn many_spheres() -> World {
    let mut hittables = HittableList::new();

    // Ground
    let checker = texture::checker(
        texture::solid_color(Color::new(0.2, 0.3, 0.1)),
        texture::solid_color(Color::new(0.9, 0.9, 0.9)),
    );
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::LambertianTexture(checker),
    }));

    // Spheres
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::LambertianTexture(texture::noise(Perlin::new(), 4.0)),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0),
    }));

    // Illuminated sphere
    hittables.add(Box::new(Sphere {
        center: Point::new(-4.0, 0.5, 2.0),
        radius: 0.5,
        material: Material::Lambertian(Color::new(2.0, 2.0, 1.0)),
    }));

    for a in -5..5 {
        for b in -5..5 {
            let choose_mat = rand::thread_rng().gen::<f32>();
            let center = Point {
                x: a as f32 + 0.9 * rand::thread_rng().gen::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * rand::thread_rng().gen::<f32>(),
            };

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    let albedo = Color::random() * Color::random();
                    let center1 =
                        center + Vec3::new(0.0, rand::thread_rng().gen_range(0.0, 0.3), 0.0);
                    hittables.add(Box::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Material::Lambertian(albedo),
                    }));
                } else if choose_mat < 0.8 {
                    let texture = texture::image("earthmap.jpg");
                    hittables.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::LambertianTexture(texture),
                    }));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    hittables.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(albedo, fuzz),
                    }));
                } else {
                    hittables.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    }));
                }
            }
        }
    }

    World {
        hittables,
        background_color_top: Color::new(0.7, 0.8, 1.0),
        background_color_bottom: Color::new(0.7, 0.8, 1.0),
        ..World::default()
    }
}

fn two_perlin_spheres() -> World {
    let mut hittables = HittableList::new();

    let texture = texture::marble(Perlin::new(), 4.0);
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(1.0, 1.0, 1.0), 0.0),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::LambertianTexture(texture.clone()),
    }));

    World {
        hittables,
        background_color_top: Color::new(0.7, 0.8, 1.0),
        background_color_bottom: Color::new(0.7, 0.8, 1.0),
        ..World::default()
    }
}

fn earth() -> World {
    let mut hittables = HittableList::new();

    let texture = texture::image("earthmap.jpg");
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 0.0, 0.0),
        radius: 2.0,
        material: Material::LambertianTexture(texture),
    }));

    World {
        hittables,
        background_color_top: Color::new(0.7, 0.8, 1.0),
        background_color_bottom: Color::new(0.7, 0.8, 1.0),
        ..World::default()
    }
}

fn light_rectangle() -> World {
    let mut hittables = HittableList::new();

    // Two marble spheres
    let texture = texture::marble(Perlin::new(), 4.0);
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Material::Metal(Color::new(1.0, 1.0, 1.0), 0.0),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::LambertianTexture(texture.clone()),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 5.0, 0.0),
        radius: 1.0,
        material: Material::DiffuseLight(Color::new(4.0, 3.0, 1.0)),
    }));

    hittables.add(Box::new(Rect {
        a0: 3.0,
        a1: 5.0,
        b0: 1.0,
        b1: 3.0,
        k: -2.0,
        material: Material::DiffuseLight(Color::new(4.0, 4.0, 4.0)),
        plane: Plane::XY,
    }));

    let camera = Camera::new(
        Point::new(26.0, 3.0, 6.0),
        Point::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.,
        3. / 2.,
        0.,
        10.,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        ..World::default()
    }
}

fn cornell_box() -> World {
    let mut hittables = HittableList::new();

    // Walls
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.12, 0.45, 0.15)),
        plane: Plane::YZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::new(0.65, 0.05, 0.05)),
        plane: Plane::YZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XY,
    }));

    let tall_cube = {
        let cube = Box::new(Cube::new(
            Point::zeros(),
            Point::new(165.0, 330.0, 165.0),
            Material::Metal(Color::new(0.8, 0.85, 0.88), 0.),
        ));
        let rot = Box::new(RotateY::new(cube, 15.0));
        Box::new(Translate::new(rot, Vec3::new(265.0, 0.0, 295.0)))
    };
    hittables.add(tall_cube);
    // let short_cube = {
    //     let cube = Box::new(Cube::new(
    //         Point::zeros(),
    //         Point::from(165.0),
    //         Material::Lambertian(Color::from(0.73)),
    //     ));
    //     let rot = Box::new(RotateY::new(cube, -18.0));
    //     Box::new(Translate::new(rot, Vec3::new(130.0, 0.0, 65.0)))
    // };
    // hittables.add(short_cube);
    hittables.add(box Sphere {
        center: Point::new(190., 90., 190.),
        radius: 90.,
        material: Material::Dielectric(1.5),
    });

    // Light
    hittables.add(Box::new(FlipFace {
        hittable: Box::new(Rect {
            a0: 213.0,
            a1: 343.0,
            b0: 227.0,
            b1: 332.0,
            k: 550.0,
            material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
            plane: Plane::XZ,
        }),
    }));
    let mut lights = HittableList::new();
    lights.add(box Rect {
        a0: 213.0,
        a1: 343.0,
        b0: 227.0,
        b1: 332.0,
        k: 550.0,
        material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
        plane: Plane::XZ,
    });
    lights.add(box Sphere {
        center: Point::new(190., 90., 190.),
        radius: 90.,
        material: Material::Dielectric(1.5),
    });

    let lookfrom = Point::new(278.0, 278.0, -800.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        lights,
        samples_per_pixel: 100,
        ..World::default()
    }
}

fn cornell_smokes() -> World {
    let mut hittables = HittableList::new();

    // Walls
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.12, 0.45, 0.15)),
        plane: Plane::YZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::new(0.65, 0.05, 0.05)),
        plane: Plane::YZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XZ,
    }));
    hittables.add(Box::new(Rect {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XY,
    }));

    let tall_cube = {
        let cube = Box::new(Cube::new(
            Point::zeros(),
            Point::new(165.0, 330.0, 165.0),
            Material::Lambertian(Color::from(0.73)),
        ));
        let rot = Box::new(RotateY::new(cube, 15.0));
        let translate = Box::new(Translate::new(rot, Vec3::new(265.0, 0.0, 295.0)));
        Box::new(ConstantMedium::new(translate, 0.01, Color::zeros()))
    };
    hittables.add(tall_cube);
    let short_cube = {
        let cube = Box::new(Cube::new(
            Point::zeros(),
            Point::from(165.0),
            Material::Lambertian(Color::from(0.73)),
        ));
        let rot = Box::new(RotateY::new(cube, -18.0));
        let translate = Box::new(Translate::new(rot, Vec3::new(130.0, 0.0, 65.0)));
        Box::new(ConstantMedium::new(translate, 0.01, Color::ones()))
    };
    hittables.add(short_cube);

    // Light
    hittables.add(Box::new(Rect {
        a0: 213.0,
        a1: 343.0,
        b0: 227.0,
        b1: 332.0,
        k: 550.0,
        material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
        plane: Plane::XZ,
    }));

    let lookfrom = Point::new(278.0, 278.0, -800.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        ..World::default()
    }
}

fn final_scene() -> World {
    let mut boxes: Vec<Box<dyn Hittable>> = Vec::new();

    // Floor
    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rand::thread_rng().gen::<f32>() + 0.01);
            let z1 = z0 + w;
            boxes.push(Box::new(Cube::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                Material::Lambertian(Color::new(0.48, 0.83, 0.53)),
            )));
        }
    }
    let bvh = BVHNode::new(boxes, 0.0, 1.0);

    let mut hittables = HittableList::new();
    hittables.add(Box::new(bvh));

    // Ceiling light
    hittables.add(Box::new(Rect {
        a0: 123.0,
        a1: 423.0,
        b0: 147.0,
        b1: 412.0,
        k: 554.0,
        material: Material::DiffuseLight(Color::new(7.0, 7.0, 7.0)),
        plane: Plane::XZ,
    }));

    // Moving sphere
    hittables.add(Box::new(MovingSphere {
        center0: Point::new(400.0, 400.0, 200.0),
        center1: Point::new(430.0, 400.0, 200.0),
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        material: Material::Lambertian(Color::new(0.7, 0.3, 0.1)),
    }));

    // Spheres
    hittables.add(Box::new(Sphere {
        center: Point::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Material::Dielectric(1.5),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Material::Metal(Color::new(0.8, 0.8, 0.9), 10.0),
    }));

    // Volumes
    let boundary = Sphere {
        center: Point::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Material::Dielectric(1.5),
    };
    hittables.add(Box::new(boundary.clone()));
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Sphere {
        center: Point::zeros(),
        radius: 5000.0,
        material: Material::Dielectric(1.5),
    };
    hittables.add(Box::new(boundary.clone()));
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(boundary),
        0.0001,
        Color::ones(),
    )));

    hittables.add(Box::new(Sphere {
        center: Point::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: Material::LambertianTexture(texture::image("earthmap.jpg")),
    }));
    let perlin = Perlin::new();
    hittables.add(Box::new(Sphere {
        center: Point::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Material::LambertianTexture(texture::noise(perlin, 0.1)),
    }));

    let mut spheres: Vec<Box<dyn Hittable>> = Vec::new();

    for _ in 0..1000 {
        spheres.push(Box::new(Sphere {
            center: Vec3::random_range(0.0, 165.0),
            radius: 10.0,
            material: Material::Lambertian(Color::from(0.73)),
        }))
    }

    let bvh = {
        let bvh = BVHNode::new(spheres, 0.0, 1.0);
        let rot = RotateY::new(Box::new(bvh), 15.0);
        Translate::new(Box::new(rot), Vec3::new(-100.0, 270.0, 395.0))
    };
    hittables.add(Box::new(bvh));

    // Camera
    let lookfrom = Point::new(478.0, 278.0, -600.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        ..World::default()
    }
}

fn custom_scene() -> World {
    let mut boxes: Vec<Box<dyn Hittable>> = Vec::new();

    let mut rng = rand::thread_rng();

    // Floor
    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rand::thread_rng().gen::<f32>() + 0.01);
            let z1 = z0 + w;
            boxes.push(Box::new(Cube::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                if rng.gen::<f32>() < 0.2 {
                    Material::DiffuseLight(Color::new(10., 5., 5.))
                } else {
                    Material::Lambertian(Color::new(0.48, 0.83, 0.53))
                },
            )));
        }
    }
    let bvh = BVHNode::new(boxes, 0.0, 1.0);

    let mut hittables = HittableList::new();
    hittables.add(Box::new(bvh));

    // Spheres
    hittables.add(Box::new(Sphere {
        center: Point::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Material::Dielectric(1.5),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Material::Metal(Color::new(0.8, 0.8, 0.9), 10.0),
    }));

    // Volumes
    let boundary = Sphere {
        center: Point::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Material::Dielectric(1.5),
    };
    hittables.add(Box::new(boundary.clone()));
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(boundary),
        0.2,
        Color::new(0.6, 0.1, 0.1),
    )));

    hittables.add(Box::new(Sphere {
        center: Point::new(220.0, 280.0, 300.0),
        radius: 100.0,
        material: Material::LambertianTexture(texture::image("neptune.jpg")),
    }));
    let perlin = Perlin::new();
    hittables.add(Box::new(Sphere {
        center: Point::new(400.0, 200.0, 400.0),
        radius: 80.0,
        material: Material::LambertianTexture(texture::marble(perlin, 4.0)),
    }));

    let mut spheres: Vec<Box<dyn Hittable>> = Vec::new();

    for _ in 0..1000 {
        spheres.push(Box::new(Sphere {
            center: Vec3::random_range(0.0, 165.0),
            radius: 10.0,
            material: Material::Lambertian(Color::from(0.73)),
        }))
    }

    let bvh = {
        let bvh = BVHNode::new(spheres, 0.0, 1.0);
        let rot = RotateY::new(Box::new(bvh), 15.0);
        Translate::new(Box::new(rot), Vec3::new(-100.0, 270.0, 395.0))
    };
    hittables.add(Box::new(bvh));

    // Camera
    let lookfrom = Point::new(478.0, 278.0, -600.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        ..World::default()
    }
}

fn space_dount() -> World {
    let mut hittables = HittableList::new();
    let mut rng = rand::thread_rng();

    // Floor
    let mut boxes: Vec<Box<dyn Hittable>> = Vec::new();
    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rand::thread_rng().gen::<f32>() + 0.01);
            let z1 = z0 + w;
            boxes.push(Box::new(Cube::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                if rng.gen::<f32>() < 0.2 {
                    Material::DiffuseLight(Color::new(10., 5., 5.))
                } else {
                    Material::Lambertian(Color::random())
                },
            )));
        }
    }
    let bvh = BVHNode::new(boxes, 0.0, 1.0);
    hittables.add(Box::new(bvh));

    // Stars
    let mut stars: Vec<Box<dyn Hittable>> = Vec::new();
    for i in 0..200 {
        let rad = (i as f32 / 90.) * std::f32::consts::TAU;

        let x = rad.cos() * 800.;
        let y = 400. + rng.gen_range(-400., 400.);
        let z = rad.sin() * 800.;
        stars.push(Box::new(Sphere {
            center: Point::new(x, y, z),
            radius: 0.5,
            material: Material::DiffuseLight(Color::ones() * 10.),
        }));
    }
    let bvh = BVHNode::new(stars, 0.0, 1.0);
    hittables.add(Box::new(bvh));

    hittables.add(Box::new(TracedSDF {
        sdf: Box::new(SDFDonut {
            center: Point::new(220.0, 380.0, 300.0),
            radius0: 100.,
            radius1: 50.,
        }),
        material: Material::Metal(Color::from(0.7), 0.),
    }));
    // hittables.add(Box::new(Sphere {
    //     center: Point::new(200., 200., 100.),
    //     radius: 50.,
    //     material: Material::Metal(Color::from(0.7), 0.),
    // }));

    // Ceiling light
    hittables.add(Box::new(Rect {
        a0: 123.0,
        a1: 423.0,
        b0: 147.0,
        b1: 412.0,
        k: 554.0,
        material: Material::DiffuseLight(Color::new(7.0, 7.0, 7.0)),
        plane: Plane::XZ,
    }));

    // Camera
    let lookfrom = Point::new(478.0, 278.0, -600.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        ..World::default()
    }
}

fn imagine() -> World {
    let mut hittables = HittableList::new();
    let mut rng = rand::thread_rng();

    let orange = Color::new(270. / 255., 106. / 255., 7. / 255.);
    let teal = Color::new(64. / 255., 231. / 255., 184. / 255.);
    let blue = Color::new(142. / 255., 226. / 255., 224. / 255.);

    // Cylinder lights
    let orange_cylinder = Cylinder {
        center: Point::new(100., 0., -30.),
        radius: 10.,
        material: Material::DiffuseLight(orange),
    };
    let teal_cylinder = Cylinder {
        center: Point::new(100., 0., 0.),
        radius: 10.,
        material: Material::DiffuseLight(teal),
    };
    let blue_cylinder = Cylinder {
        center: Point::new(100., 0., 30.),
        radius: 10.,
        material: Material::DiffuseLight(blue),
    };
    hittables.add(box orange_cylinder.clone());
    hittables.add(box teal_cylinder.clone());
    hittables.add(box blue_cylinder.clone());

    // Gasses
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(Cube::new(
            Point::new(80., -100., -60.),
            Point::new(120., 300., 60.),
            Material::Dielectric(1.5),
        )),
        0.01,
        Color::ones(),
    )));
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(Cube::new(
            Point::new(80., -100., -100.),
            Point::new(120., 300., 100.),
            Material::Dielectric(1.5),
        )),
        0.001,
        Color::ones(),
    )));
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(Sphere {
            center: Point::zeros(),
            radius: 5000.,
            material: Material::Dielectric(1.5),
        }),
        0.0001,
        Color::ones(),
    )));

    // Floor
    hittables.add(Box::new(Rect {
        a0: -100.,
        a1: 90.,
        b0: -300.,
        b1: 300.,
        k: 0.,
        material: Material::Metal(Color::from(0.9), 0.2),
        plane: Plane::XZ,
    }));

    // Pyramids
    let mut pyramids: Vec<Box<dyn Hittable>> = Vec::new();
    for _ in 0..20 {
        let size = rng.gen_range(10., 30.);
        let top = Point::new(
            rng.gen_range(40., 100. - 2. * size),
            rng.gen_range(10., 30.),
            rng.gen_range(-140., 140.),
        );

        pyramids.push(Box::new(RotateY::new(
            Box::new(Pyramid::new(
                top,
                Point::new(top.x - size, -10., top.z + size),
                Point::new(top.x + size, -10., top.z + size),
                Point::new(top.x + size, -10., top.z - size),
                Point::new(top.x - size, -10., top.z - size),
                Material::Metal(Color::from(0.9), 0.2),
            )),
            rng.gen_range(-60., 60.),
        )));
    }
    let bvh = BVHNode::new(pyramids, 0.0, 1.0);
    hittables.add(Box::new(bvh));

    // Camera
    let lookfrom = Point::new(-100.0, 40.0, 0.0);
    let lookat = Point::new(200., 30., 0.);
    let dist_to_focus = 100.; //(lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        60.,
        3. / 2.,
        0.1,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        background_color_top: Color::new(0.05, 0.05, 0.2),
        background_color_bottom: Color::zeros(),
        samples_per_pixel: 100,
        ..World::default()
    }
}

fn menger_sponge() -> World {
    let mut hittables = HittableList::new();

    hittables.add(sdf::menger_sponge(9));

    // Camera
    // let lookfrom = Point::new(3.0, 0.0, 3.0);
    // let lookat = Point::new(13.0, 0.0, 13.0);
    let lookfrom = Point::new(13.0, 9.0, 13.0) * 10.;
    let lookat = Point::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        samples_per_pixel: 100,
        background_color_top: Color::ones(),
        background_color_bottom: Color::ones(),
        ..World::default()
    }
}

fn mandelbulb() -> World {
    let mut hittables = HittableList::new();

    hittables.add(box TracedSDF {
        sdf: box SDFMandelBulb {
            center: Vec3::zeros(),
        },
        material: Material::Lambertian(Color::new(0.8, 0.1, 0.1)),
    });

    // Ceiling light
    hittables.add(box FlipFace {
        hittable: box Rect {
            a0: -1.0,
            a1: 1.0,
            b0: -1.0,
            b1: 1.0,
            k: 5.0,
            material: Material::DiffuseLight(Color::new(7.0, 7.0, 7.0)),
            plane: Plane::XZ,
        },
    });

    // Camera
    let lookfrom = Point::new(13.0, 9.0, 13.0) * 0.25;
    let lookat = Point::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        samples_per_pixel: 100,
        background_color_top: Color::new(0.02, 0.02, 0.05),
        background_color_bottom: Color::new(0.1, 0.1, 0.2),
        ..World::default()
    }
}

fn mandelbox() -> World {
    let mut hittables = HittableList::new();

    hittables.add(box TracedSDF {
        sdf: box SDFMandelBox {
            center: Vec3::zeros(),
            scale: 2.,
        },
        material: Material::Lambertian(Color::new(0.8, 0.1, 0.1)),
    });

    // Camera
    let lookfrom = Point::new(13.0, 9.0, 13.0) * 1.8;
    let lookat = Point::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        samples_per_pixel: 10,
        background_color_top: Color::ones(),
        background_color_bottom: Color::ones(),
        ..World::default()
    }
}

fn knot() -> World {
    let mut hittables = HittableList::new();

    hittables.add(box TracedSDF {
        sdf: box SDFKnot {
            center: Point::zeros(),
            k: 3.5,
        },
        material: Material::Lambertian(Color::new(0.8, 0.1, 0.1)),
    });

    // Camera
    let lookfrom = Point::new(0.0, 0.0, 50.0) * 1.;
    let lookat = Point::zeros();
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        3. / 2.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        samples_per_pixel: 10,
        background_color_top: Color::ones(),
        background_color_bottom: Color::ones(),
        ..World::default()
    }
}

fn cornell_mandelbox() -> World {
    let mut cornell = HittableList::new();

    // Walls
    let size = 17.0;
    cornell.add(Box::new(Rect {
        a0: -size,
        a1: size,
        b0: -size,
        b1: size,
        k: size,
        material: Material::Lambertian(Color::new(0.12, 0.45, 0.15)),
        plane: Plane::YZ,
    }));
    cornell.add(Box::new(Rect {
        a0: -size,
        a1: size,
        b0: -size,
        b1: size,
        k: -size,
        material: Material::Lambertian(Color::new(0.65, 0.05, 0.05)),
        plane: Plane::YZ,
    }));
    cornell.add(Box::new(Rect {
        a0: -size,
        a1: size,
        b0: -size,
        b1: size,
        k: -size,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XZ,
    }));
    cornell.add(Box::new(Rect {
        a0: -size,
        a1: size,
        b0: -size,
        b1: size,
        k: size,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XZ,
    }));
    cornell.add(Box::new(Rect {
        a0: -size,
        a1: size,
        b0: -size,
        b1: size,
        k: size,
        material: Material::Lambertian(Color::from(0.73)),
        plane: Plane::XY,
    }));

    // Light
    cornell.add(Box::new(FlipFace {
        hittable: Box::new(Rect {
            a0: -3.0,
            a1: 3.0,
            b0: -3.0,
            b1: 3.0,
            k: size,
            material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
            plane: Plane::XZ,
        }),
    }));

    let mut hittables = HittableList::new();
    hittables.add(box Translate::new(box cornell, Vec3::new(0., 11., 0.)));
    hittables.add(box RotateY::new(
        box TracedSDF {
            sdf: box SDFMandelBox {
                center: Point::zeros(),
                scale: 2.,
            },
            material: Material::Metal(Color::new(0.8, 0.8, 0.8), 0.),
        },
        15.0,
    ));

    let lookfrom = Point::new(0.0, 15.0, -61.0);
    let lookat = Point::new(0., 10., 0.);
    let dist_to_focus = (lookfrom - lookat).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        1.,
        0.,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        aspect_ratio: 1.,
        samples_per_pixel: 10,
        background_color_top: Color::new(225. / 255., 41. / 255., 131. / 255.),
        background_color_bottom: Color::new(0., 78. / 255., 182. / 255.),
        ..World::default()
    }
}
