module Slayout
open System
open System.Drawing
open System.Windows.Forms

type size = { s_width: int; s_height: int }

type layout =
  | L_control of Control * size
  | L_vlist of layout list
  | L_hlist of layout list
  | L_margin of size

let vlist list = L_vlist list
let hlist list = L_hlist list
let margin x y = L_margin {s_width = x; s_height = y}
let ctrl x y (c: #Control) =
  c.Width <- x
  c.Height <- y
  L_control (c, {s_width = x; s_height = y})

let textbox s = new TextBox(Text = s)
let label s = new Label(Text = s)

let create_layout (form: Form) layout =
  let rec aux x y = function
    | L_control (ctrl, {s_width = w; s_height = h}) ->
        form.Controls.Add(ctrl)
        ctrl.Left <- x
        ctrl.Top <- y
        (w, h)
    | L_margin {s_width = w; s_height = h} -> (w, h)
    | L_vlist l ->
        let f (acc_w, acc_h) elem =
          let (w, h) = aux x (y + acc_h) elem
          (acc_w + w, acc_h + h)
        List.fold f (0, 0) l
    | L_hlist l ->
        let f (acc_w, acc_h) elem =
          let (w, h) = aux (x + acc_w) y elem
          (acc_w + w, acc_h + h)
        List.fold f (0, 0) l
  ignore (aux 0 0 layout)

