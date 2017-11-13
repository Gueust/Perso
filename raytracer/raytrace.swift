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
  let _ = FileManager.default.createFile(atPath: filename, contents: nil, attributes: nil)
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

// Cross product
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
        result[i][j] += lhs[i][k] * rhs[k][j]
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

func scale4(_ s1: Double, _ s2: Double, _ s3: Double) -> Mat {
  var mat = zero(dim1: 4, dim2: 4)
  mat[0][0] = s1
  mat[1][1] = s2
  mat[2][2] = s3
  mat[3][3] = 1
  return mat
}

func tr4(_ s1: Double, _ s2: Double, _ s3: Double) -> Mat {
  var mat = zero(dim1: 4, dim2: 4)
  mat[0][3] = s1
  mat[1][3] = s2
  mat[2][3] = s3
  for i in 0..<4 {
    mat[i][i] = 1
  }
  return mat
}

func rot4(_ axis: Vec3, _ deg: Double) -> Mat {
  var mat = zero(dim1: 4, dim2: 4)
  let rad = deg * Double.pi / 180
  let cosT = cos(rad)
  let sinT = sin(rad)
  let x = axis.x
  let y = axis.y
  let z = axis.z
  mat[0][0] = cosT + (1 - cosT) * x * x
  mat[0][1] = (1 - cosT) * x * y - z * sinT
  mat[0][2] = (1 - cosT) * x * z + y * sinT
  mat[1][0] = (1 - cosT) * x * y + z * sinT
  mat[1][1] = cosT + (1 - cosT) * y * y
  mat[1][2] = (1 - cosT) * y * z - x * sinT
  mat[2][0] = (1 - cosT) * x * z - y * sinT
  mat[2][1] = (1 - cosT) * y * z + x * sinT
  mat[2][2] = cosT + (1 - cosT) * z * z
  mat[3][3] = 1
  return mat
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

  // There must be an easier way to do this copy-all-but-one initialization.
  init(_ material: Material, diffuse: Color) {
    self.diffuse = diffuse
    self.specular = material.specular
    self.shininess = material.shininess
    self.emission = material.emission
  }
  init(_ material: Material, specular: Color) {
    self.diffuse = material.diffuse
    self.specular = specular
    self.shininess = material.shininess
    self.emission = material.emission
  }
  init(_ material: Material, shininess: Double) {
    self.diffuse = material.diffuse
    self.specular = material.specular
    self.shininess = shininess
    self.emission = material.emission
  }
  init(_ material: Material, emission: Color) {
    self.diffuse = material.diffuse
    self.specular = material.specular
    self.shininess = material.shininess
    self.emission = emission
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

func sphere(_ material: Material,
            _ center: Vec3,
            _ r: Double,
            transf: Mat = id4,
            revtr: Mat = id4,
            ambient: Color = black) -> Object {
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
    transf: transf,
    revtr: revtr,
    ambient: ambient,
    int: int)
}

func triangle(_ material: Material,
              _ a: Vec3,
              _ b: Vec3,
              _ c: Vec3,
              transf: Mat = id4,
              revtr: Mat = id4,
              ambient: Color = black) -> Object {
  let normal = ((c - a) * (b - a)).normalize()
  let orthAB = (b - a) * normal
  let orthCA = (a - c) * normal
  let orthBC = (c - b) * normal
  let dAB_C = orthAB ** (c - a)
  let dBC_A = orthBC ** (a - b)
  let dCA_B = orthCA ** (b - c)
  let int: (Ray) -> Intersection? = { (ray: Ray) in
    let div = ray.direction ** normal
    if abs(div) < 1e-10 {
      return nil
    }
    let t = ((a ** normal) - (ray.origin ** normal)) / div
    let p = ray.origin + (ray.direction * (one * t))
    if t <= 0
        || (orthAB ** (p - a)) * dAB_C < 0
        || (orthBC ** (p - b)) * dBC_A < 0
        || (orthCA ** (p - c)) * dCA_B < 0 {
      return nil
    }
    return Intersection(
      point: p,
      normal: (normal ** ray.direction) > 0 ? zero3 - normal : normal)
  }
  return Object(
    material: material,
    transf: transf,
    revtr: revtr,
    ambient: ambient,
    int: int)
}

struct Scene {
  var camera: Camera
  var objects: [Object]
  var lights: [Light]
  var height: Int
  var width: Int

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
      if !object.material.specular.isBlack() {
        let reflDir = ray.direction - int.normal * (2 * (int.normal ** ray.direction))
        let reflRay = Ray(origin: int.point, direction: reflDir)
        let reflColor = getColor(maxDepth: maxDepth-1, ray: reflRay)
        result = result + (object.material.specular * reflColor)
      }
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

struct Defaults {
  var material: Material
  var ambient: Color
  var ver: [Vec3]
  var stack: [(Mat, Mat)]

  mutating func addTransform(_ m1: Mat, _ m2: Mat) {
    let (prev1, prev2) = stack.popLast()!
    stack.append((prev1 * m1, m2 * prev2))
  }
}

func parseVec3(_ words: [Substring], _ offset: Int) -> Vec3 {
  return Vec3(
    x: Double(words[offset])!,
    y: Double(words[offset + 1])!,
    z: Double(words[offset + 2])!)
}

func parseColor(_ words: [Substring], _ offset: Int) -> Color {
  return Color(
    red: Double(words[offset])!,
    green: Double(words[offset + 1])!,
    blue: Double(words[offset + 2])!)
}

func parseSceneFile(path: String) -> Scene {
  let data = try! String(contentsOfFile: path, encoding: .utf8)
  var scene = Scene(
    camera: Camera(fov: 0, lookFrom: zero3, lookAt: zero3, up: zero3),
    objects: [],
    lights: [],
    height: 0,
    width: 0)
  var defaults = Defaults(
    material: Material(black),
    ambient: black,
    ver: [],
    stack: [(id4, id4)])
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
        scene.width = Int(words[1])!
        scene.height = Int(words[2])!
      case ("camera", 11):
        scene.camera = Camera(
          fov: Double(words[10])!,
          lookFrom: parseVec3(words, 1),
          lookAt: parseVec3(words, 4),
          up: parseVec3(words, 7))
      case ("directional", 7):
        let dir = parseVec3(words, 1)
        let color = parseColor(words, 4)
        scene.lights.append(Light.directional(dir, color))
      case ("point", 7):
        let dir = parseVec3(words, 1)
        let color = parseColor(words, 4)
        scene.lights.append(Light.point(dir, color, 1, 0, 0))
      case ("ambient", 4):
        defaults.ambient = parseColor(words, 1)
      case ("vertex", 4):
        defaults.ver.append(parseVec3(words, 1))
      case ("diffuse", 4):
        defaults.material = Material(defaults.material, diffuse: parseColor(words, 1))
      case ("emission", 4):
        defaults.material = Material(defaults.material, emission: parseColor(words, 1))
      case ("specular", 4):
        defaults.material = Material(defaults.material, specular: parseColor(words, 1))
      case ("shininess", 2):
        defaults.material = Material(defaults.material, shininess: Double(words[1])!)
      case ("sphere", 5):
        let center = parseVec3(words, 1)
        let r = Double(words[4])!
        let (transf, revtr) = defaults.stack.last!
        scene.objects.append(sphere(
          defaults.material, center, r, transf: transf, revtr: revtr, ambient: defaults.ambient))
      case ("tri", 4):
        let a = defaults.ver[Int(words[1])!]
        let b = defaults.ver[Int(words[2])!]
        let c = defaults.ver[Int(words[3])!]
        let (transf, revtr) = defaults.stack.last!
        scene.objects.append(triangle(
          defaults.material, a, b, c, transf: transf, revtr: revtr, ambient: defaults.ambient))
      case ("pushTransform", 1):
        defaults.stack.append(defaults.stack.last!)
      case ("popTransform", 1):
        let _ = defaults.stack.popLast()
      case ("scale", 4):
        let sx = Double(words[1])!
        let sy = Double(words[2])!
        let sz = Double(words[3])!
        defaults.addTransform(scale4(sx, sy, sz), scale4(1/sx, 1/sy, 1/sz))
      case ("translate", 4):
        let sx = Double(words[1])!
        let sy = Double(words[2])!
        let sz = Double(words[3])!
        defaults.addTransform(tr4(sx, sy, sz), tr4(-sx, -sy, -sz))
      case ("rotate", 5):
        let axis = parseVec3(words, 1)
        let deg = Double(words[4])!
        defaults.addTransform(rot4(axis, -deg), rot4(axis, deg))
      case _:
        print("Unable to parse line: " + line)
    }
  }
  return scene
}

func main() {
  if CommandLine.arguments.count == 2 {
    let scene = parseSceneFile(path: CommandLine.arguments[1])
    let image = scene.render(maxDepth: 1)
    writePPM(filename: "out.ppm", image: image)
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
