use std::fs::File;
use std::io::prelude::*;
use std::ops;

struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red: red, green: green, blue: blue }
    }

    fn to_bytes(&self) -> [u8; 3] {
        let to_u8 = |f: f64| (f * 255.0) as u8;
        [ to_u8(self.red), to_u8(self.green), to_u8(self.blue) ]
    }

    const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
    const WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0 };
    const RED: Color = Color { red: 1.0, green: 0.0, blue: 0.0 };
    const GREEN: Color = Color { red: 0.0, green: 1.0, blue: 0.0 };
    const BLUE: Color = Color { red: 0.0, green: 0.0, blue: 1.0 };
}

struct Image(Vec<Vec<Color>>);

impl Image {
    fn write_ppm(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(&path)?;
        let &Image(ref content) = self;
        let nb_rows = content.len();
        let nb_cols = content[0].len();
        write!(file, "P6 {} {} 255\n", nb_cols, nb_rows)?;
        for line in content.iter() {
            for color in line.iter() {
                file.write(&color.to_bytes())?;
            }
        }
        Ok(())
    }
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn scale(&self, f : f64) -> Vec3 {
        Vec3 { x: f * self.x, y: f * self.y, z: f * self.z }
    }

    fn normalize(&self) -> Vec3 {
        self.scale(1.0 / self.norm())
    }

    const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
}

struct Mat4([f64; 16]);

impl Mat4 {
    fn id() -> Mat4 {
        let mut id: [f64; 16] = [0.0; 16];
        for i in 0..4 {
            id[i + 4*i] = 1.0;
        }
        Mat4(id)
    }
}

struct Material {
    diffuse: Color,
    specular: Color,
    shininess: f64,
    emission: Color,
}

struct Ray {
    origin: Vec3,
    direction: Vec3
}

struct Intersection {
    point: Vec3,
    normal: Vec3,
}

struct Camera {
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,
    fov: f64,
}

struct Object {
    // int
    material: Material,
    transf: Mat4,
    revtr: Mat4,
    ambient: Color,
}

enum Light {
    Point(Vec3, Color, (f64, f64, f64)),
    Dir(Vec3, Color),
}

struct Scene {
    camera: Camera,
    objects: Vec<Object>,
    lights: Vec<Light>,
    height: i64,
    width: i64,
}

impl Scene {
    fn render_pixel(&self, i: i64, j: i64) -> Color {
        Color::RED
    }

    fn render(&self) -> Image {
        Image((0..self.height).map(|i|
            (0..self.width).map(|j| self.render_pixel(i, j)
        ).collect()).collect())
    }
}

fn main() {
    let camera = Camera {
        look_from: Vec3 { x: 5.0, y: 0.0, z: 0.0 },
        up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        look_at: Vec3::ZERO,
        fov: 45.0,
    };
    let lights = vec![
        Light::Dir(Vec3 { x: -1.0, y: 1.0, z: 1.0 }, Color::WHITE),
        Light::Dir(Vec3 { x: -1.0, y: -1.0, z: 1.0 }, Color::WHITE),
    ];
    let scene = Scene {
        camera: camera,
        objects: vec![],
        lights: lights,
        width: 640,
        height: 480,
    };
    scene.render().write_ppm("test.ppm").unwrap();
}
