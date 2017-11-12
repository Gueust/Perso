(* Compile with: ocamlopt.opt str.cmxa raytrace.ml *)

let pi = 4. *. atan 1.

type color = {
  cred: float;
  cgreen: float;
  cblue: float;
}
let color cred cgreen cblue = {cred; cgreen; cblue}
let black = {cred = 0.; cgreen = 0.; cblue = 0.;}
let red = {cred = 1.; cgreen = 0.; cblue = 0.;}
let green = {cred = 0.; cgreen = 1.; cblue = 0.;}
let blue = {cred = 0.; cgreen = 0.; cblue = 1.;}
let white = {cred = 1.; cgreen = 1.; cblue = 1.;}
let (+@) {cred = r; cgreen = g; cblue = b;} {cred = r'; cgreen = g'; cblue = b'} =
  let n x = min x 1. in
  {cred = n (r +. r'); cgreen = n (g +. g'); cblue = n (b +. b')}

let ( *@) {cred = r; cgreen = g; cblue = b;} {cred = r'; cgreen = g'; cblue = b'} =
  {cred = r *. r'; cgreen = g *. g'; cblue = b *. b'}

let ( *@@) {cred; cgreen; cblue} f =
  let n x = min (f *. x) 1. in
  {cred = n cred; cgreen = n cgreen; cblue = n cblue}

let print_color {cred; cgreen; cblue} = Printf.printf "%.2f %.2f %.2f\n" cred cgreen cblue

type image = color array array

(* Vectors operation *)
type vec3 = {x: float; y: float; z: float;}
type point3 = vec3

let vec3 x y z = {x; y; z}
let zero3 = {x = 0.; y = 0.; z = 0.;}
let dot3 {x; y; z} {x = x'; y = y'; z = z'} =
  x*.x' +. y*.y' +. z*.z'

let norm3 v = sqrt (dot3 v v)

let scale3 {x; y; z} lambda =
  {x = lambda *. x; y = lambda *. y; z = lambda *. z}

let ( *~) {x; y; z} {x = x'; y = y'; z = z'} =
  {x = y*.z' -. y'*.z; y = z*.x' -. z'*.x; z = x*.y' -. x'*.y}

let (+~) {x; y; z} {x = x'; y = y'; z = z'} =
  {x = x+.x'; y = y+.y'; z = z+.z'}

let (-~) {x; y; z} {x = x'; y = y'; z = z'} =
  {x = x-.x'; y = y-.y'; z = z-.z'}

let ( *~~) {x; y; z} lambda =
  {x = lambda*.x; y = lambda*.y; z = lambda*.z}

let normalize3 v = v *~~ (1. /. norm3 v)

let print_v3 {x; y; z} = Printf.printf "%.4f %.4f %.4f\n" x y z

(* Matrix operations. *)
type mat4 = float array (* 16 floats *)
let diag4 d = Array.init 16 (fun i -> if i mod 4 == i / 4 then d else 0.)
let id4 = diag4 1.

let add4 a b = Array.init 16 (fun i -> a.(i) +. b.(i))

let mult4 a b =
  let prod idx =
    let i, j = idx / 4, idx mod 4 in
    a.(4*i+0) *. b.(4*0+j) +.
    a.(4*i+1) *. b.(4*1+j) +.
    a.(4*i+2) *. b.(4*2+j) +.
    a.(4*i+3) *. b.(4*3+j)
  in
  Array.init 16 prod

let mult4v a {x; y; z} = {
  x = a.(0) *. x +. a.(1) *. y +. a.(2) *. z;
  y = a.(4) *. x +. a.(5) *. y +. a.(6) *. z;
  z = a.(8) *. x +. a.(9) *. y +. a.(10) *. z;
}

let mult4t a {x; y; z} = {
  x = a.(0) *. x +. a.(4) *. y +. a.(8) *. z;
  y = a.(1) *. x +. a.(5) *. y +. a.(9) *. z;
  z = a.(2) *. x +. a.(6) *. y +. a.(10) *. z;
}

let mult4p a {x; y; z} =
  let n = a.(12) *. x +. a.(13) *. y +. a.(14) *. z +. a.(15) in
  {
    x = (a.(0) *. x +. a.(1) *. y +. a.(2) *. z +. a.(3)) /. n;
    y = (a.(4) *. x +. a.(5) *. y +. a.(6) *. z +. a.(7)) /. n;
    z = (a.(8) *. x +. a.(9) *. y +. a.(10) *. z +. a.(11)) /. n;
  }

(* Matrix transformations. *)
let scale4 s1 s2 s3 =
  let fill = function
    | 0 -> s1
    | 5 -> s2
    | 10 -> s3
    | 15 -> 1.
    | _ -> 0.
  in
  Array.init 16 fill

let tr4 t1 t2 t3 =
  let fill = function
    | 3 -> t1
    | 7 -> t2
    | 11 -> t3
    | i when i mod 4 == i / 4 -> 1.
    | _ -> 0.
  in
  Array.init 16 fill

let rot4 {x; y; z} degrees =
  let rad = degrees *. pi /. 180. in
  let cosT, sinT = cos rad, sin rad in
  (* Use Rodrigue rotation formulae. *)
  let r11 = cosT +. (1.-.cosT)*.x*.x in
  let r12 = (1.-.cosT)*.x*.y -. z*.sinT in
  let r13 = (1.-.cosT)*.x*.z +. y*.sinT in
  let r21 = (1.-.cosT)*.x*.y +. z*.sinT in
  let r22 = cosT +. (1.-.cosT)*.y*.y in
  let r23 = (1.-.cosT)*.y*.z -. x*.sinT in
  let r31 = (1.-.cosT)*.x*.z -. y*.sinT in
  let r32 = (1.-.cosT)*.y*.z +. x*.sinT in
  let r33 = cosT +. (1.-.cosT)*.z*.z in
  [|r11; r21; r31; 0.; r12; r22; r32; 0.; r13; r23; r33; 0.; 0.; 0.; 0.; 1.|]

let default_stack = [id4, id4]

(* Materials. *)
type material = {
  m_diffuse: color;
  m_specular: color;
  m_shininess: float;
  m_emission: color;
}

let default_material clr = {
  m_diffuse = clr;
  m_specular = black;
  m_shininess = 0.;
  m_emission = black;
}

(* Scene description primitives. *)
type ray = {
  origin: point3;
  direction: vec3;
}

(* Returns the intersection and the normal. *)
type intersection = {
  i_point: point3;
  i_normal: vec3;
}

type object_t = {
  o_int: ray -> intersection option;
  o_material: material;
  o_transf: mat4;
  o_revtr: mat4;
  o_ambient: color;
}

let default_object = {
  o_int = (fun _ -> failwith "not implemented");
  o_material = default_material black;
  o_transf = id4;
  o_revtr = id4;
  o_ambient = black;
}
type light =
  | L_point of point3 * color * (float * float * float) (* Light attenuation. *)
  | L_dir of vec3 * color

type camera = {
  look_from: point3;
  look_at: point3;
  up: vec3;
  fov: float;
}

type input = {
  i_camera: camera;
  i_objects: object_t list;
  i_lights: light list;
  i_height: int;
  i_width: int;
}

let default_input = {
  i_camera = {look_from = zero3; look_at = zero3; up = zero3; fov = 0.};
  i_objects = [];
  i_lights = [];
  i_height = 0;
  i_width = 0;
}

let one = 1. -. 1e-10
let sphere center r =
  fun {origin; direction} -> 
    let orc = origin -~ center in
    let a = dot3 direction direction in
    let b = 2. *. (dot3 direction orc) in
    let c = dot3 orc orc -. r *. r in
    let delta = b *. b -. 4. *. a *. c in
    if delta < 0. then None
    else
      let t1 = (-. b -. sqrt delta) /. (2. *. a) in
      let t2 = (-. b +. sqrt delta) /. (2. *. a) in
      if t1 <= 0. && t2 <= 0. then None
      else
        let t = if t1 <= 0. then t2 else if t2 <= 0. then t1 else min t1 t2 in
        let p = origin +~ (direction *~~ (one *. t)) in
        let normal = normalize3 (p -~ center) in
        Some {i_point = p; i_normal = normal}

let triangle a b c =
  let normal = normalize3 ((c -~ a) *~ (b -~ a)) in
  let orth_ab = (b -~ a) *~ normal in
  let orth_ca = (a -~ c) *~ normal in
  let orth_bc = (c -~ b) *~ normal in
  let d3_ab_c = dot3 orth_ab (c -~ a) in
  let d3_bc_a = dot3 orth_bc (a -~ b) in
  let d3_ca_b = dot3 orth_ca (b -~ c) in
  fun {origin; direction} -> 
    let div = dot3 direction normal in
    if abs_float div < 1.e-10 then None
    else
      let t = ((dot3 a normal) -. (dot3 origin normal)) /. div in
      let p = origin +~ (direction *~~ (one *.t)) in
      if 0. < t && dot3 orth_ab (p -~ a) *. d3_ab_c >= 0.  && dot3 orth_bc (p -~ b) *. d3_bc_a >= 0.  && dot3 orth_ca (p -~ c) *. d3_ca_b >= 0. then
        let normal = if 0. < dot3 normal direction then zero3 -~ normal else normal in
        Some {i_point = p; i_normal = normal}
      else None

(* Rendering primitives *)
let cast_ray {look_from; look_at; up; fov} width height =
  let tany = tan (fov *. pi /. 360.) in
  let width = float_of_int width /. 2. in
  let height = float_of_int height /. 2. in
  let tanx = tany *. width /. height in
  let w = normalize3 (look_from -~ look_at) in
  let u = normalize3 (up *~ w) in
  let v = w *~ u in
  fun i j ->
    let alpha = tanx *. ((0.5 +. float_of_int j) /. width -. 1.) in
    let beta = tany *. (1. -. (0.5 +. float_of_int i) /. height) in
    let dir = (u *~~ alpha) +~ (v *~~ beta) -~ w in
    {origin = look_from; direction = normalize3 dir}

let intersect objects =
  fun {origin; direction} ->
    let f acc obj =
      let inters =
        let ray = {
          origin = mult4p obj.o_revtr origin;
          direction = mult4v obj.o_revtr direction;
        }
        in
        match obj.o_int ray with
        | None -> None
        | Some {i_point; i_normal} ->
            let i_point = mult4p obj.o_transf i_point in
            let i_normal = normalize3 (mult4t obj.o_revtr i_normal) in
            Some {i_point; i_normal}
      in
      match acc, inters with
      | _, None -> acc
      | None, Some i ->
          let d = norm3 (i.i_point -~ origin) in
          Some(d, i, obj)
      | Some (acc_d, _, _), Some i ->
          let d = norm3 (i.i_point -~ origin) in
          if acc_d < d then acc else Some(d, i, obj)
    in
    List.fold_left f None objects

let rec get_color max_depth input {direction} = function
  | None -> black
  | _ when max_depth = 0 -> black
  | Some (_, {i_normal; i_point}, obj) ->
      let {i_objects; i_lights; i_camera} = input in
      let {o_material; o_ambient} = obj in
      let {m_diffuse; m_specular; m_shininess; m_emission} = o_material in
      let add_light acc light =
        let dir =
          match light with
          | L_dir (d, _) -> d
          | L_point (p, _, _) -> p -~ i_point
        in
        let dir = normalize3 dir in
        let light_ray = {origin = i_point; direction = dir} in
        let is_visible = None == intersect i_objects light_ray in
        if not is_visible then acc
        else
          let attenuation, clr =
            match light with
            | L_dir (_, c) -> 1., c
            | L_point (p, c, (c0, c1, c2)) ->
                let r = norm3 (i_point -~ p) in
                1. /. (c0 +. c1 *. r +. c2 *. r *. r), c
          in
          let clr = clr *@@ attenuation in
          let diff = m_diffuse *@@ (max 0. (dot3 i_normal dir)) in
          let shin =
            if m_specular = black then black
            else
              let half = normalize3 (dir -~ direction) in
              m_specular *@@ ((max 0. (dot3 i_normal half)) ** m_shininess)
          in
          acc +@ (clr *@ (diff +@ shin))
      in
      let color = List.fold_left add_light (o_ambient +@ m_emission) i_lights in
      if m_specular = black then color
      else (* Handle mirror effect. *)
        let refl_dir = direction -~ (i_normal *~~ (2. *. dot3 i_normal direction)) in
        let refl_ray = {origin = i_point; direction = refl_dir} in
        let refl_int = intersect i_objects refl_ray in
        let refl_color = get_color (max_depth - 1) input refl_ray refl_int in
        color +@ (m_specular *@ refl_color)

let render max_depth input =
  let {i_camera; i_objects; i_width; i_height; i_lights} = input in
  let cast_ray = cast_ray i_camera i_width i_height in
  let render_pixel i j =
    let ray = cast_ray i j in
    get_color max_depth input ray (intersect i_objects ray)
  in
  Array.init i_height (fun l -> Array.init i_width (fun c -> render_pixel l c))

(* Parse some text input. *)
type defaults = {
  d_obj: object_t;
  d_ver: point3 list;
  d_vea: point3 array;
  d_stack: (mat4 * mat4) list;
}

let line_update defaults current_scene words =
  let fs = float_of_string in
  let is = int_of_string in
  let add_object o_int =
    let o_transf, o_revtr = List.hd defaults.d_stack in
    let o = {defaults.d_obj with o_int; o_transf; o_revtr} in
    defaults, {current_scene with i_objects = o::current_scene.i_objects}
  in
  let add_transform m m' =
    let d_stack =
      match defaults.d_stack with
      | [] -> failwith "Empty stack!"
      | (t, t')::q -> (mult4 t m, mult4 m' t')::q
    in
    {defaults with d_stack}, current_scene
  in
  match words with
  | ["size"; width; height] ->
      let i_width, i_height = is width, is height in
      defaults, {current_scene with i_width; i_height}
  | ["camera"; lfx; lfy; lfz; lax; lay; laz; upx; upy; upz; fov] ->
      let look_from = vec3 (fs lfx) (fs lfy) (fs lfz) in
      let look_at = vec3 (fs lax) (fs lay) (fs laz) in
      let up = vec3 (fs upx) (fs upy) (fs upz) in
      let fov = fs fov in
      defaults, {current_scene with i_camera = {look_from; look_at; up; fov}}
  | ["sphere"; x; y; z; radius] ->
      let center = vec3 (fs x) (fs y) (fs z) in
      add_object (sphere center (fs radius))
  | ["ambient"; r; g; b] ->
      let o_ambient = color (fs r) (fs g) (fs b) in
      let d_obj = {defaults.d_obj with o_ambient} in
      {defaults with d_obj}, current_scene
  | ["vertex"; x; y; z] ->
      let v = vec3 (fs x) (fs y) (fs z) in
      {defaults with d_ver = v::defaults.d_ver}, current_scene
  | ["tri"; v1; v2; v3] ->
      let d_vea, defaults =
        if 0 == Array.length defaults.d_vea then
          let d_vea = Array.of_list (List.rev defaults.d_ver) in
          d_vea, {defaults with d_vea}
        else defaults.d_vea, defaults
      in
      let v n = d_vea.(is n) in
      add_object (triangle (v v1) (v v2) (v v3))
  | ["directional"; x; y; z; r; g; b] ->
      let color = color (fs r) (fs g) (fs b) in
      let l = L_dir ((vec3 (fs x) (fs y) (fs z)), color) in
      defaults, {current_scene with i_lights = l::current_scene.i_lights}
  | ["point"; x; y; z; r; g; b] ->
      let color = color (fs r) (fs g) (fs b) in
      let att = 1., 0., 0. in
      let l = L_point ((vec3 (fs x) (fs y) (fs z)), color, att) in
      defaults, {current_scene with i_lights = l::current_scene.i_lights}
  | ["diffuse"; r; g; b] ->
      let m_diffuse = color (fs r) (fs g) (fs b) in
      let o_material = {defaults.d_obj.o_material with m_diffuse} in
      let d_obj = {defaults.d_obj with o_material} in
      {defaults with d_obj}, current_scene
  | ["emission"; r; g; b] ->
      let m_emission = color (fs r) (fs g) (fs b) in
      let o_material = {defaults.d_obj.o_material with m_emission} in
      let d_obj = {defaults.d_obj with o_material} in
      {defaults with d_obj}, current_scene
  | ["specular"; r; g; b] ->
      let m_specular = color (fs r) (fs g) (fs b) in
      let o_material = {defaults.d_obj.o_material with m_specular} in
      let d_obj = {defaults.d_obj with o_material} in
      {defaults with d_obj}, current_scene
   | ["shininess"; s] ->
      let o_material = {defaults.d_obj.o_material with m_shininess = fs s} in
      let d_obj = {defaults.d_obj with o_material} in
      {defaults with d_obj}, current_scene
  | ["pushTransform"] ->
      let d = defaults.d_stack in
      {defaults with d_stack = List.hd d::d}, current_scene
  | ["popTransform"] ->
      {defaults with d_stack = List.tl defaults.d_stack}, current_scene
  | ["scale"; sx; sy; sz] ->
      let sx, sy, sz = fs sx, fs sy, fs sz in
      add_transform (scale4 sx sy sz) (scale4 (1./.sx) (1./.sy) (1./.sz))
  | ["translate"; tx; ty; tz] ->
      let tx, ty, tz = fs tx, fs ty, fs tz in
      add_transform (tr4 tx ty tz) (tr4 (-.tx) (-.ty) (-.tz))
  | ["rotate"; x; y; z; deg] ->
      let x, y, z, deg = fs x, fs y, fs z, fs deg in
      let axis = {x; y; z} in
      add_transform (rot4 axis (-.deg)) (rot4 axis deg)
  | _ -> defaults, current_scene

let parse_input filename =
  Printf.printf "Reading from %s...\n" filename;
  let file = open_in filename in
  let split = Str.split (Str.regexp "[ \t]+") in
  let rec read defaults current_scene =
    try
      let line = input_line file in
      let words = split line in
      let defaults, current_scene = line_update defaults current_scene words in
      read defaults current_scene
    with
    | End_of_file -> current_scene
  in
  let defaults = {d_obj = default_object; d_ver = []; d_vea = [||]; d_stack = [id4, id4]} in
  let input = read defaults default_input in
  close_in file;
  input

(* Image writing (using simple text-based ppm format). *)
let write_ppm filename image =
  let file = open_out_bin filename in
  let nb_cols, nb_rows = Array.length image.(0), Array.length image in
  Printf.fprintf file "P6 %d %d 255\n" nb_cols nb_rows;
  let print_rgb {cred; cgreen; cblue} =
    let n f = int_of_float (255. *. f) in
    output_byte file (n cred);
    output_byte file (n cgreen);
    output_byte file (n cblue)
  in
  let print_line line = Array.iter print_rgb line in
  Array.iter print_line image;
  close_out file

let () =
  let input =
    if Array.length Sys.argv == 2 then
      parse_input Sys.argv.(1)
    else
      let look_from = {x = 5.; y = 0.; z = 0.;} in
      let up = {x = 0.; y = 1.; z = 0.;} in
      let i_camera = {fov = 45.; look_from; look_at = zero3; up} in
      let sphere p r c =
        let o_material = default_material c in
        let o_int = sphere p r in
        {default_object with o_material; o_int}
      in
      let s1 = sphere zero3 1. red in
      let s2 = sphere {x = 1.; y = 0.1; z = 0.1}  0.2 green in
      let dl1 = L_dir ({x = -1.; y = 1.; z = 1.}, white) in
      let dl2 = L_dir ({x = -1.; y = -1.; z = 1.}, white *@@ 0.3) in
      {i_camera; i_objects = [s1; s2]; i_lights =[dl1; dl2]; i_width = 320; i_height = 240}
  in
  let img = render 5 input in
  write_ppm "test.ppm" img
