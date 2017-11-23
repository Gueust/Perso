use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
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

    fn add(&self, rhs: &Color) -> Color {
        Color {
            red: f64::min(1.0, self.red + rhs.red),
            green: f64::min(1.0, self.green + rhs.green),
            blue: f64::min(1.0, self.blue + rhs.blue),
        }
    }

    fn mult(&self, rhs: &Color) -> Color {
        Color {
            red: f64::min(1.0, self.red * rhs.red),
            green: f64::min(1.0, self.green * rhs.green),
            blue: f64::min(1.0, self.blue * rhs.blue),
        }
    }

    fn scale(&self, s: f64) -> Color {
        Color {
            red: f64::min(1.0, s * self.red),
            green: f64::min(1.0, s * self.green),
            blue: f64::min(1.0, s * self.blue),
        }
    }

    fn is_black(&self) -> bool {
        self.red == 0.0 && self.green == 0.0 && self.blue == 0.0
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

#[derive(Clone)]
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

    fn mul(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - rhs.y * self.z,
            self.z * rhs.x - rhs.z * self.x,
            self.x * rhs.y - rhs.x * self.y)
    }

    fn add(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    fn sub(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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

struct Ray<'a> {
    origin: &'a Vec3,
    direction: &'a Vec3
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
    intersect: Box<Fn(&Ray) -> Option<Intersection>>,
    material: Material,
    transf: Mat4,
    revtr: Mat4,
    ambient: Color,
}

enum Light {
    Point(Vec3, Color, (f64, f64, f64)),
    Directional(Vec3, Color),
}

impl Light {
    fn dir(&self, for_point: &Vec3) -> Vec3 {
        match self {
            &Light::Point(ref d, _, (_, _, _)) => d.sub(for_point),
            &Light::Directional(ref dir, _) => dir.clone(),
        }
    }

    fn attenuation(&self, for_point: &Vec3) -> (f64, Color) {
        match self {
            &Light::Point(ref p, ref c, (c0, c1, c2)) => {
                let r = for_point.sub(&p).norm();
                (1.0 / (c0 + c1 * r + c2 * r * r), c.clone())
            },
            &Light::Directional(_, ref color) => (1.0, color.clone()),
        }
    }
}

struct Scene {
    camera: Camera,
    objects: Vec<Object>,
    lights: Vec<Light>,
    height: i64,
    width: i64,
}

struct RenderFn<'a>(Box<Fn(i64, i64) -> Color + 'a>);


impl Scene {
    fn intersect(&self, ray: &Ray) -> Option<(Intersection, &Object)> {
        self.objects.iter().fold(None, |acc, object| {
            match (object.intersect)(ray) {
                None => acc,
                Some(int) => {
                    let d = int.point.sub(&ray.origin).norm();
                    match acc {
                        Some((dd, _, _)) if dd < d => acc,
                        Some(_) | None => Some((d, int, object)),
                    }
                }
            }
        }).map(|(_, int, object)| (int, object))
    }

    fn get_color(&self, max_depth: i64, ray: &Ray) -> Color {
        if max_depth > 0 {
            self.intersect(ray).map_or(Color::BLACK, |(int, object)| {
                let color = object.ambient.add(&object.material.emission);
                self.lights.iter().fold(color, |acc, light| {
                    let dir = light.dir(&int.point).normalize();
                    let light_ray = Ray {
                        origin: &int.point,
                        direction: &dir,
                    };
                    if self.intersect(ray).is_none() {
                        let (attenuation, color) = light.attenuation(&int.point);
                        let diff = object.material.diffuse
                            .scale(f64::max(0.0, int.normal.dot(&dir)));
                        let shin =
                            if object.material.specular.is_black() {
                                Color::BLACK
                            } else {
                                let refl_dir = ray.direction
                                    .sub(&int.normal.scale(2.0 * int.normal.dot(ray.direction)));
                                let refl_ray = Ray { direction: &refl_dir, origin: &int.point };
                                let refl_color = self.get_color(max_depth-1, &refl_ray);
                                object.material.specular.mult(&refl_color)
                            };
                        let color = color.add(&shin);
                        acc.add(&color.scale(attenuation).mult(&diff.add(&shin)))

                    } else {
                        acc
                    }
                })
            })
        } else {
            Color::BLACK
        }
    }

    fn render_pixel(&self) -> RenderFn {
        let tany = f64::tan(self.camera.fov * std::f64::consts::PI / 360.0);
        let width = self.width as f64 / 2.0;
        let height = self.height as f64 / 2.0;
        let tanx = tany * width / height;
        let w = self.camera.look_from.sub(&self.camera.look_at).normalize();
        let u = self.camera.up .mul(&w).normalize();
        let v = w.mul(&u);
        RenderFn(Box::new(move |i, j| {
            let alpha = tanx * ((0.5 + j as f64) / width - 1.0);
            let beta = tany * (1.0 - (0.5 + i as f64) / height);
            let ray = Ray {
                origin: &self.camera.look_from,
                direction: &u.scale(alpha).add(&v.scale(beta)).sub(&w).normalize(),
            };
            self.get_color(5, &ray) } ) )
    }

    fn render(&self) -> Image {
        let RenderFn(ref render_pixel) = self.render_pixel();
        Image((0..self.height).map(|i|
            (0..self.width).map(|j| render_pixel(i, j)
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
        Light::Directional(Vec3 { x: -1.0, y: 1.0, z: 1.0 }, Color::WHITE),
        Light::Directional(Vec3 { x: -1.0, y: -1.0, z: 1.0 }, Color::WHITE),
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
