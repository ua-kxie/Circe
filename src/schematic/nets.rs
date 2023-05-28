pub mod graph;
use std::{rc::Rc, cell::RefCell};

pub use graph::Nets;

use crate::transforms::{VSPoint, VSBox, VCTransform, SchematicSpace};
use iced::widget::canvas::Frame;

use flagset::flags;

// assign each edge a name
// on split: if named, give subgraph other than named new name. give sub graph with fewer vertices a new name if not named
// on merge: older name (less ord) overwrites newer name

// have a text label which attaches to the edge - draw a dashed line to center of edge in preview
// each edge have Option<Rc<RefCell<SchematicNetLabel>>>
// all connected edges take on SchematicNetLabel.label as their name
// if multiple connected edges have different Label - throw error during schematic check

// Label: Rc<RefCell<String>>, every netedge to have a copy
// on prune, ensure that all netedges have Label that Rc::ptr_eq a Label in the netname stack
// if not eq, generate a new Label on the netname stack and assign to all connected edges

// all SchematicNetLabel

// type Label = Rc<RefCell<String>>;
// #[derive(Clone, Debug, Default, PartialEq, Eq)]
// struct SchematicNetLabel {
//     label: String,
//     // other stuff for drawing on schematic, being edited from schematic
// } 

// struct NetIds {
//     autogen_stack: Vec<Label>,  // autogenerated Labels
//     // userdef_stack: Vec<Rc<RefCell<SchematicNetLabel>>>,  // userdefined Labels - only need to exist as part of edge
// }  // every label string must be unique at all times
// user may insert SchematicNetLabel which collide with existing auto-generated

// how to link SchematicNetLabel with NetEdge?
// each edge have Option<Rc<RefCell<SchematicNetLabel>>>

// every netedge an enum of Label/SchematicNetLabel ? no, better have a consolidated place to check collision and stuff
// how to check autogen labels do not collide across sub-graphs? find collision, then treat in prune

pub trait Selectable {
    // collision with point, selection box
    fn collision_by_vsp(&self, curpos_vsp: VSPoint) -> bool;
    fn contained_by_vsb(&self, selbox: VSBox) -> bool;
    fn collision_by_vsb(&self, selbox: VSBox) -> bool;
}

pub trait Drawable {
    const SOLDER_DIAMETER: f32 = 0.25;
    const WIRE_WIDTH: f32 = 0.05;
    const ZOOM_THRESHOLD: f32 = 5.0;
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
    fn draw_selected(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
    fn draw_preview(&self, vct: VCTransform, vcscale: f32, frame: &mut Frame);
}

flags! {
    enum DrawState: u8 {
        Persistent,
        Selected,
        Preview,
    }
}

