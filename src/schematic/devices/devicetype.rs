use euclid::Vector2D;
use iced::{Size, widget::canvas::{self, stroke, LineCap, path::Builder}, Color};

use crate::{
    transforms::{
        SSPoint, VSBox, VSPoint, VCTransform, Point, ViewportSpace, SSBox
    }, schematic::Drawable, 
};
use iced::{widget::canvas::{Frame, Stroke}};
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Port {
    pub name: &'static str,
    pub offset: SSPoint,
}

impl Drawable for Port {
    fn draw_persistent(&self, vct: VCTransform, vcscale: f32, frame: &mut iced::widget::canvas::Frame) {
        let f = canvas::Fill {
            style: canvas::Style::Solid(Color::from_rgba(1.0, 0.0, 0.0, 1.0)),
            ..canvas::Fill::default()
        };
        let dim = 0.4;
        let ssb = VSBox::new(
            self.offset.cast::<f32>().cast_unit() - Vector2D::new(dim/2.0, dim/2.0), 
            self.offset.cast::<f32>().cast_unit() + Vector2D::new(dim/2.0, dim/2.0), 
        );

        let csbox = vct.outer_transformed_box(&ssb);
        
        let top_left = csbox.min;
        let size = Size::new(csbox.width(), csbox.height());
        frame.fill_rectangle(Point::from(top_left).into(), size, f);
    }

    fn draw_selected(&self, vct: crate::transforms::VCTransform, vcscale: f32, frame: &mut iced::widget::canvas::Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 1.),
            style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 0.0)),
            line_cap: LineCap::Square,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        let dim = 0.4;
        let vsb = VSBox::new(
            self.offset.cast::<f32>().cast_unit() - Vector2D::new(dim/2.0, dim/2.0), 
            self.offset.cast::<f32>().cast_unit() + Vector2D::new(dim/2.0, dim/2.0), 
        );
        let csb = vct.outer_transformed_box(&vsb);
        let size = Size::new(csb.width(), csb.height());
        path_builder.rectangle(Point::from(csb.min).into(), size);
        frame.stroke(&path_builder.build(), stroke);     
    }

    fn draw_preview(&self, vct: crate::transforms::VCTransform, vcscale: f32, frame: &mut iced::widget::canvas::Frame) {
        let stroke = Stroke {
            width: (STROKE_WIDTH * vcscale).max(STROKE_WIDTH * 1.),
            style: stroke::Style::Solid(Color::from_rgb(1.0, 1.0, 0.5)),
            line_cap: LineCap::Square,
            ..Stroke::default()
        };
        let mut path_builder = Builder::new();
        let dim = 0.4;
        let vsb = VSBox::new(
            self.offset.cast::<f32>().cast_unit() - Vector2D::new(dim/2.0, dim/2.0), 
            self.offset.cast::<f32>().cast_unit() + Vector2D::new(dim/2.0, dim/2.0), 
        );
        let csb = vct.outer_transformed_box(&vsb);
        let size = Size::new(csb.width(), csb.height());
        path_builder.rectangle(Point::from(csb.min).into(), size);
        frame.stroke(&path_builder.build(), stroke);     
    }
}
const STROKE_WIDTH: f32 = 0.03;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Graphics {
    pub pts: Vec<Vec<VSPoint>>
}

impl Graphics {
    pub fn new_gnd() -> Self {
        Self {
            pts: vec![
                vec![
                    VSPoint::new(0., 2.),
                    VSPoint::new(0., -1.)
                ],
                vec![
                    VSPoint::new(0., -2.),
                    VSPoint::new(1., -1.),
                    VSPoint::new(-1., -1.),
                    VSPoint::new(0., -2.),
                ],
            ]
        }
    }

    pub fn new_res() -> Self {
        Self {
            pts: vec![
                vec![
                    VSPoint::new(0., 3.),
                    VSPoint::new(0., -3.),
                ],
                vec![
                    VSPoint::new(-1., 2.),
                    VSPoint::new(-1., -2.),
                    VSPoint::new(1., -2.),
                    VSPoint::new(1., 2.),
                    VSPoint::new(-1., 2.),
                ],
            ]
        }
    }
}

// struct RawDef {
//     raw_definition: String,
// }

// impl RawDef {
//     fn definition(&self) -> &str {
//         &self.raw_definition
//     }
// }

// struct ValueDef {
//     value: f32,
// }

// impl ValueDef {
//     fn definition(&self) -> &str {
//         &self.value.to_string()
//     }
// }

// enum RDef {
//     Raw(RawDef),
//     Value(ValueDef),
// }
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
enum TypeEnum {
    // R(RDef),
    #[default] L,
    C,
    V,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceType {
    ports: Vec<Port>,
    bounds: SSBox,
    graphic: Graphics,
    type_enum: TypeEnum,  // R, L, C, V, etc.
}

impl DeviceType {
    pub fn get_ports(&self) -> &[Port] {
        &self.ports
    }
    pub fn get_bounds(&self) -> &SSBox {
        &self.bounds
    }
    pub fn get_graphics(&self) -> &Graphics {
        &self.graphic
    }
    pub fn get_type(&self) -> &TypeEnum {
        &self.type_enum
    }
    pub fn new_gnd() -> Self {
        Self {
            ports: vec![
                Port {name: "gnd", offset: SSPoint::new(0, 2)}
            ],
            bounds: SSBox::new(SSPoint::new(-1, 2), SSPoint::new(1, -2)), 
            graphic: Graphics::new_gnd(),
            type_enum: TypeEnum::V, 
        }
    }
    pub fn new_res() -> Self {
        Self {
            ports: vec![
                Port {name: "+", offset: SSPoint::new(0, 3)},
                Port {name: "-", offset: SSPoint::new(0, -3)},
            ],
            bounds: SSBox::new(SSPoint::new(-2, 3), SSPoint::new(2, -3)), 
            graphic: Graphics::new_res(),
            type_enum: TypeEnum::C, 
        }
    }
}