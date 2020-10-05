#[allow(unused_imports)]
use crate::{
    bvh::*,
    camera::*,
    hittable::{
        cube::*, cylinder::*, medium::*, moving_sphere::*, pyramid::*, rectangle::*, rotate::*,
        sdf::*, sphere::*, translate::*, triangle::*, *,
    },
    hittable_list::*,
    material::*,
    perlin::*,
    texture,
    vec3::*,
};
use rand::Rng;

const SCENE: Scene = Scene::CornellBox;

#[allow(dead_code)]
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
}

pub struct World {
    pub hittables: HittableList,
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

pub fn generate_world() -> World {
    match SCENE {
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
    }
}

fn test() -> World {
    let mut hittables = HittableList::new();

    let top = Point::new(0.0, 1.0, 0.0);
    let size = 1.0;
    hittables.add(Box::new(Pyramid::new(
        top,
        Point::new(top.x - size, 0., top.x + size),
        Point::new(top.x + size, 0., top.x + size),
        Point::new(top.x + size, 0., top.x - size),
        Point::new(top.x - size, 0., top.x - size),
        Material::Lambertian(Color::new(0.8, 0.2, 0.2)),
    )));
    hittables.add(Box::new(Sphere {
        center: Point::new(0., 4., 3.),
        radius: 1.,
        material: Material::DiffuseLight(Color::from(2.)),
    }));

    let aspect_ratio = 3. / 2.;
    let camera = Camera::new(
        Point::new(-10., 2., 4.),
        Point::zeros(),
        Vec3::new(0.0, 1.0, 0.0),
        40.,
        aspect_ratio,
        0.,
        10.,
        0.0,
        1.0,
    );

    World {
        hittables,
        samples_per_pixel: 50,
        max_depth: 50,
        camera,
        aspect_ratio,
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

    hittables.add(Box::new(XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        material: Material::DiffuseLight(Color::new(4.0, 4.0, 4.0)),
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
    hittables.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.12, 0.45, 0.15)),
    }));
    hittables.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::new(0.65, 0.05, 0.05)),
    }));
    hittables.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::from(0.73)),
    }));
    hittables.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
    }));
    hittables.add(Box::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
    }));

    let tall_cube = {
        let cube = Box::new(Cube::new(
            Point::zeros(),
            Point::new(165.0, 330.0, 165.0),
            Material::Lambertian(Color::from(0.73)),
        ));
        let rot = Box::new(RotateY::new(cube, 15.0));
        Box::new(Translate::new(rot, Vec3::new(265.0, 0.0, 295.0)))
    };
    hittables.add(tall_cube);
    let short_cube = {
        let cube = Box::new(Cube::new(
            Point::zeros(),
            Point::from(165.0),
            Material::Lambertian(Color::from(0.73)),
        ));
        let rot = Box::new(RotateY::new(cube, -18.0));
        Box::new(Translate::new(rot, Vec3::new(130.0, 0.0, 65.0)))
    };
    hittables.add(short_cube);

    // Light
    hittables.add(Box::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 550.0,
        material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
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
        samples_per_pixel: 300,
        ..World::default()
    }
}

fn cornell_smokes() -> World {
    let mut hittables = HittableList::new();

    // Walls
    hittables.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.12, 0.45, 0.15)),
    }));
    hittables.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::new(0.65, 0.05, 0.05)),
    }));
    hittables.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::from(0.73)),
    }));
    hittables.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
    }));
    hittables.add(Box::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::from(0.73)),
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
    hittables.add(Box::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 550.0,
        material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
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
    hittables.add(Box::new(XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: Material::DiffuseLight(Color::new(7.0, 7.0, 7.0)),
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
    hittables.add(Box::new(XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        material: Material::DiffuseLight(Color::new(7.0, 7.0, 7.0)),
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

    let orange = Color::new(253. / 255., 106. / 255., 7. / 255.);
    let teal = Color::new(64. / 255., 231. / 255., 184. / 255.);
    let blue = Color::new(142. / 255., 226. / 255., 224. / 255.);

    // Cylinder lights
    hittables.add(Box::new(Cylinder {
        center: Point::new(100., 0., -30.),
        axis: Vec3::new(0., 1., 0.),
        radius: 10.,
        material: Material::DiffuseLight(orange * 0.8),
    }));
    hittables.add(Box::new(Cylinder {
        center: Point::new(100., 0., 0.),
        axis: Vec3::new(0., 1., 0.),
        radius: 10.,
        material: Material::DiffuseLight(teal * 0.8),
    }));
    hittables.add(Box::new(Cylinder {
        center: Point::new(100., 0., 30.),
        axis: Vec3::new(0., 1., 0.),
        radius: 10.,
        material: Material::DiffuseLight(blue * 0.8),
    }));

    // Gasses
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(Cube::new(
            Point::new(80., 0., -60.),
            Point::new(120., 300., 60.),
            Material::Dielectric(1.5),
        )),
        0.01,
        Color::ones(),
    )));
    hittables.add(Box::new(ConstantMedium::new(
        Box::new(Cube::new(
            Point::new(80., 0., -100.),
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
    hittables.add(Box::new(XZRect {
        x0: -100.,
        x1: 100.,
        z0: -300.,
        z1: 300.,
        k: 0.,
        material: Material::Metal(Color::new(0.95, 0.95, 0.95), 0.01),
    }));

    // Pyramids
    let mut pyramids: Vec<Box<dyn Hittable>> = Vec::new();
    for _ in 0..20 {
        let size = rng.gen_range(10., 30.);
        let top = Point::new(
            rng.gen_range(70., 100. - size),
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
                Material::Metal(Color::new(0.95, 0.95, 0.95), 0.3),
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
        1.,
        0.1,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        background_color_top: Color::zeros(),
        background_color_bottom: Color::new(0.005, 0.005, 0.02),
        samples_per_pixel: 200,
        ..World::default()
    }
}

fn menger_sponge() -> World {
    let mut hittables = HittableList::new();

    let sponge = {
        let a = Box::new(SDFCube {
            center: Point::zeros(),
            dimensions: Vec3::from(33.),
        });

        let mut b = sdf::cross(Point::zeros(), 1. / 3.);
        for i in 0..20 {
            b = Box::new(SDFUnion {
                a: Box::new(SDFRepetition {
                    a: sdf::cross(Point::zeros(), 1. / 3. * i as f32),
                    repetition: Vec3::from(3. * i as f32),
                }),
                b,
            });
        }

        TracedSDF {
            sdf: Box::new(SDFSubstraction { a, b }),
            material: Material::Lambertian(Color::new(0.8, 0.1, 0.1)),
        }
    };
    hittables.add(Box::new(sponge));

    hittables.add(Box::new(Sphere {
        center: Point::new(15.0, 0.0, 13.) * 1.2,
        radius: 0.5,
        material: Material::DiffuseLight(Color::from(3.)),
    }));

    // Camera
    // let lookfrom = Point::new(3.0, 0.0, 3.0);
    // let lookat = Point::new(13.0, 0.0, 13.0);
    let lookfrom = Point::new(13.0, 0.0, 0.0);
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
        ..World::default()
    }
}
