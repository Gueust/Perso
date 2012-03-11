open System
open System.Drawing
open System.Windows.Forms
open Slayout

(* simple test for now *)
let form = new Form(TopMost = true, Visible = true, Width = 500)
let my_hl l1 l2 = hlist [ ctrl 70 20 (label l1); ctrl 170 20 (textbox l2) ]
let layout = vlist [ margin 10 10; my_hl "test" "field"; my_hl "foo" "bar" ]
create_layout form layout
[<STAThread>]
Application.Run(form)
