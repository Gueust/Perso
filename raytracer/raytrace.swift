import Foundation

struct Color {
  let red: Double
  let green: Double
  let blue: Double

  func toString() -> String {
    return String(format: "%.2f %.2f %.2f", red, green, blue)
  }

  func isBlack() -> Bool {
    return red == 0.0 && green == 0.0 && blue == 0.0
  }
}

func +(lhs: Color, rhs: Color) -> Color {
  return Color(
    red: min(1, lhs.red + rhs.red),
    green: min(1, lhs.green + rhs.green),
    blue: min(1, lhs.blue + rhs.blue))
}

func *(lhs: Color, rhs: Color) -> Color {
  return Color(
    red: lhs.red * rhs.red,
    green: lhs.green * rhs.green,
    blue: lhs.blue * rhs.blue)
}

func *(color: Color, scale: Double) -> Color {
  return Color(
    red: min(1, scale * color.red),
    green: min(1, scale * color.green),
    blue: min(1, scale * color.blue))
}

let black = Color(red: 0, green: 0, blue: 0)
let red   = Color(red: 1, green: 0, blue: 0)
let green = Color(red: 0, green: 1, blue: 0)
let blue  = Color(red: 0, green: 0, blue: 1)
let white = Color(red: 1, green: 1, blue: 1)

func writePPM(filename: String, image: [[Color]]) {
  FileManager.default.createFile(atPath: filename, contents: nil, attributes: nil)
  let file = FileHandle(forWritingAtPath: filename)!
  let nbRows = image.count
  precondition(nbRows > 0, "number of rows has to be positive")
  let nbCols = image[0].count
  let header = String(format: "P6 %d %d 255\n", nbCols, nbRows);
  file.write(header.data(using: .utf8)!)
  var bytes: [UInt8] = []
  for row in image {
    for color in row {
      bytes.append(UInt8(color.red * 255))
      bytes.append(UInt8(color.green * 255))
      bytes.append(UInt8(color.blue * 255))
    }
  }
  file.write(Data(bytes: bytes))
}

struct Vec3 {
  let x: Double
  let y: Double
  let z: Double

  func norm() -> Double {
    return sqrt(x*x + y*y + z*z)
  }

  func normalize() -> Vec3 {
    let norm = self.norm()
    return Vec3(x: x / norm, y: y / norm, z: z / norm)
  }

  func toString() -> String {
    return String(format: "%.4f %.4f %.4f", x, y, z)
  }
}

let zero3 = Vec3(x: 0, y: 0, z: 0)

infix operator **

// TODO: find a better operator name for the dot product.
func **(lhs: Vec3, rhs: Vec3) -> Double {
  return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

func +(lhs: Vec3, rhs: Vec3) -> Vec3 {
  return Vec3(x: lhs.x + rhs.x, y: lhs.y + rhs.y, z: lhs.z + rhs.z)
}

func -(lhs: Vec3, rhs: Vec3) -> Vec3 {
  return Vec3(x: lhs.x - rhs.x, y: lhs.y - rhs.y, z: lhs.z - rhs.z)
}

func *(v: Vec3, scale: Double) -> Vec3 {
  return Vec3(x: v.x * scale, y: v.y * scale, z: v.z * scale)
}

// Outer product
func *(lhs: Vec3, rhs: Vec3) -> Vec3 {
  return Vec3(
    x: lhs.y * rhs.z - rhs.y * lhs.z,
    y: lhs.z * rhs.x - rhs.z * lhs.x,
    z: lhs.x * rhs.y - rhs.x * lhs.y
    )
}

typealias Mat = [[Double]]
typealias Vec = [Double]

func zero(dim1: Int, dim2: Int) -> Mat {
  return Array(repeating: Array(repeating: 0, count: dim2), count: dim1)
}
func diag(dim: Int, value: Double) -> Mat {
  var mat = zero(dim1: dim, dim2: dim)
  for index in 0..<dim {
    mat[index][index] = value
  }
  return mat
}

func +(lhs: Mat, rhs: Mat) -> Mat {
  let dim1 = lhs.count
  precondition(dim1 == rhs.count, "different sizes on first dimension")
  precondition(dim1 > 0, "the first dimension size must be positive")
  let dim2 = lhs[0].count
  precondition(dim2 == rhs[0].count, "different sizes on second dimension")
  var result = zero(dim1: dim1, dim2: dim2)
  for i in 0..<dim1 {
    for j in 0..<dim2 {
      result[i][j] = lhs[i][j] + rhs[i][j]
    }
  }
  return result
}

func *(lhs: Mat, rhs: Mat) -> Mat {
  let dim1 = lhs.count
  precondition(dim1 > 0, "the first dimension size must be positive")
  let dim2 = lhs[0].count
  precondition(dim2 > 0, "the second dimension size must be positive")
  precondition(dim2 == rhs.count, "incoherent sizes")
  let dim3 = lhs[0].count
  var result = zero(dim1: dim1, dim2: dim3)
  for i in 0..<dim1 {
    for j in 0..<dim3 {
      for k in 0..<dim2 {
        result[i][j] += lhs[i][k] + rhs[k][j]
      }
    }
  }
  return result
}

let zero4 = zero(dim1: 4, dim2: 4)
func diag4(value: Double) -> Mat {
  return diag(dim: 4, value: value)
}
let id4 = diag4(value: 1)

func mult4v(_ a: Mat, _ v: Vec3) -> Vec3 {
  return Vec3(
    x: a[0][0] * v.x + a[0][1] * v.y + a[0][2] * v.z,
    y: a[1][0] * v.x + a[1][1] * v.y + a[1][2] * v.z,
    z: a[2][0] * v.x + a[2][1] * v.y + a[2][2] * v.z)
}

func mult4t(_ a: Mat, _ v: Vec3) -> Vec3 {
  return Vec3(
    x: a[0][0] * v.x + a[1][0] * v.y + a[2][0] * v.z,
    y: a[0][1] * v.x + a[1][1] * v.y + a[2][1] * v.z,
    z: a[0][2] * v.x + a[1][2] * v.y + a[2][2] * v.z)
}

func mult4p(_ a: Mat, _ v: Vec3) -> Vec3 {
  precondition(a.count == 4)
  precondition(a[0].count == 4)
  let n = a[3][0] * v.x + a[3][1] * v.y + a[3][2] * v.z + a[3][3]
  return Vec3(
    x: (a[0][0] * v.x + a[0][1] * v.y + a[0][2] * v.z + a[0][3]) / n,
    y: (a[1][0] * v.x + a[1][1] * v.y + a[1][2] * v.z + a[1][3]) / n,
    z: (a[2][0] * v.x + a[2][1] * v.y + a[2][2] * v.z + a[2][3]) / n)
}

struct Material {
  let diffuse: Color
  let specular: Color
  let shininess: Double
  let emission: Color

  init(_ color: Color) {
    self.diffuse = color
    self.specular = black
    self.shininess = 0
    self.emission = black
  }
}

struct Camera {
  let fov: Double
  let lookFrom: Vec3
  let lookAt: Vec3
  let up: Vec3
}

enum Light {
  case point(Vec3, Color, Double, Double, Double)
  case directional(Vec3, Color)

  func dir() -> Vec3 {
    switch self {
      case .point(let d, _, _, _, _): return d
      case .directional(let d, _): return d
    }
  }

  func attenuation(forPoint: Vec3) -> (Double, Color) {
    switch self {
    case .point(let p, let c, let c0, let c1, let c2):
      let r = (forPoint - p).norm()
      return (1.0 / (c0 + c1 * r + c2 * r * r), c)
      case .directional(_, let color): return (1.0, color)
    }
  }
}

struct Ray {
  let origin: Vec3
  let direction: Vec3
}

struct Intersection {
  let point: Vec3
  let normal: Vec3
}

struct Object {
  let material: Material
  let transf: Mat
  let revtr: Mat
  let ambient: Color
  let int: (Ray) -> Intersection?

  func intersect(ray: Ray) -> Intersection? {
    let ray = Ray(
      origin: mult4p(revtr, ray.origin),
      direction: mult4v(revtr, ray.direction))
    return int(ray).map { i in Intersection(
      point: mult4p(transf, i.point),
      normal: mult4t(revtr, i.normal).normalize()) }
  }
}

let one = 1 - 1e-10

func sphere(_ material: Material, _ center: Vec3, _ r: Double) -> Object {
  let int: (Ray) -> Intersection? = { (ray: Ray) in
    let orc = ray.origin - center
    let a = ray.direction ** ray.direction
    let b = 2 * (ray.direction ** orc)
    let c = (orc ** orc) - r * r
    let delta = b * b - 4 * a * c
    if delta < 0 {
      return nil
    }
    let sqrt_delta = sqrt(delta)
    let t1 = (-b - sqrt_delta) / (2 * a)
    let t2 = (-b + sqrt_delta) / (2 * a)
    if t1 <= 0 && t2 <= 0 {
      return nil
    }
    let t = t1 <= 0 ? t2 : t2 <= 0 ? t1 : min(t1, t2)
    let p = ray.origin + (ray.direction * (one * t))
    return Intersection(
      point: p,
      normal: (p - center).normalize())
  }
  return Object(
    material: material,
    transf: id4,
    revtr: id4,
    ambient: black,
    int: int)
}

struct Scene {
  let camera: Camera
  let objects: [Object]
  let lights: [Light]
  let height: Int
  let width: Int

  func intersect(ray: Ray) -> (Intersection, Object)? {
    var result: (Double, Intersection, Object)? = nil
    for object in objects {
      if let int = object.intersect(ray: ray) {
        let d = (int.point - ray.origin).norm()
        if let (dd, _, _) = result, dd < d {
        }
        else {
          result = (d, int, object)
        }
      }
    }
    return result.map { (_, int, object) in (int, object) }
  }

  func getColor(maxDepth: Int, ray: Ray) -> Color {
    if maxDepth > 0, let (int, object) = intersect(ray: ray) {
      var result = object.ambient + object.material.emission
      for light in lights {
        let dir = light.dir().normalize()
        let lightRay = Ray(origin: int.point, direction: dir)
        if (intersect(ray: lightRay) == nil) {
          let (attenuation, color) = light.attenuation(forPoint: int.point)
          let diff = object.material.diffuse * max(0.0, int.normal ** dir)
          let shin = object.material.specular.isBlack() ? black
            : object.material.specular *
                max(0.0, pow(int.normal ** (dir - ray.direction).normalize(), object.material.shininess))
          result = result + (color * attenuation) * (diff + shin)
        }
      }
      // TODO: add mirror effect
      return result
    }
    return black
  }

  func render(maxDepth: Int) -> [[Color]] {
    let tany = tan(camera.fov * Double.pi / 360)
    let width2 = Double(width) / 2.0
    let height2 = Double(height) / 2.0
    let tanx = tany * width2 / height2
    let w = (camera.lookFrom - camera.lookAt).normalize()
    let u = (camera.up * w).normalize()
    let v = w * u
    var pixels = Array(repeating: Array(repeating: black, count: width), count: height)
    for row in 0..<height {
      for col in 0..<width {
        let alpha = tanx * ((0.5 + Double(col)) / width2 - 1)
        let beta = tany * (1 - (0.5 + Double(row)) / height2)
        let dir = u * alpha + v * beta - w
        let ray = Ray(origin: camera.lookFrom, direction: dir.normalize())
        pixels[row][col] = getColor(maxDepth: maxDepth, ray: ray)
      }
    }
    return pixels
  }
}

func parseInputFile(path: String) {
  let data = try! String(contentsOfFile: path, encoding: .utf8)
  for line in data.split(separator: "\n") {
    if line.isEmpty || line[line.startIndex] == "#" {
      continue
    }
    let words = line.split(separator: " ")
    if words.isEmpty {
      continue
    }
    switch (words[0], words.count) {
      case ("size", 3):
        let x = Int(words[1])!
        let y = Int(words[2])!
      case _:
        print("Unable to parse line: " + line)
    }
  }
}

func main() {
  if CommandLine.arguments.count == 2 {
    let input = parseInputFile(path: CommandLine.arguments[1])
  } else {
    let camera = Camera(
      fov: 45,
      lookFrom: Vec3(x: 5, y: 0, z: 0),
      lookAt: zero3,
      up: Vec3(x: 0, y: 1, z: 0))
    let scene = Scene(
      camera: camera,
      objects: [
        sphere(Material(red), zero3, 1),
        sphere(Material(green), Vec3(x: 1, y: 0.1, z: 0.1), 0.2)
      ],
      lights: [
        Light.directional(Vec3(x: -1, y: 1, z: 1), white),
        Light.directional(Vec3(x: 1, y: -1, z: 1), white * 0.3)
      ],
      height: 240,
      width: 320)
    let image = scene.render(maxDepth: 1)
    writePPM(filename: "out.ppm", image: image)
  }
}

main()
