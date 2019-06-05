use std::marker::PhantomData;

use super::context::ChartContext;
use crate::coord::{MeshLine, Ranged, RangedCoord};
use crate::drawing::backend::DrawingBackend;
use crate::drawing::DrawingAreaErrorKind;
use crate::style::{FontDesc, Mixable, RGBColor, ShapeStyle, TextStyle};

/// The struct that is used for tracking the configuration of a mesh of any chart
pub struct MeshStyle<'a, X: Ranged, Y: Ranged, DB>
where
    DB: DrawingBackend,
{
    pub(super) draw_x_mesh: bool,
    pub(super) draw_y_mesh: bool,
    pub(super) draw_x_axis: bool,
    pub(super) draw_y_axis: bool,
    pub(super) x_label_offset: i32,
    pub(super) n_x_labels: usize,
    pub(super) n_y_labels: usize,
    pub(super) axis_desc_style: Option<TextStyle<'a>>,
    pub(super) x_desc: Option<String>,
    pub(super) y_desc: Option<String>,
    pub(super) line_style_1: Option<ShapeStyle<'a>>,
    pub(super) line_style_2: Option<ShapeStyle<'a>>,
    pub(super) axis_style: Option<ShapeStyle<'a>>,
    pub(super) label_style: Option<TextStyle<'a>>,
    pub(super) format_x: &'a dyn Fn(&X::ValueType) -> String,
    pub(super) format_y: &'a dyn Fn(&Y::ValueType) -> String,
    pub(super) target: Option<&'a mut ChartContext<DB, RangedCoord<X, Y>>>,
    pub(super) _pahtom_data: PhantomData<(X, Y)>,
}

impl<'a, X, Y, DB> MeshStyle<'a, X, Y, DB>
where
    X: Ranged,
    Y: Ranged,
    DB: DrawingBackend,
{
    /// The offset of x labels. This is used when we want to place the label in the middle of
    /// the grid. This is useful if we are drawing a histogram
    /// - `value`: The offset in pixel
    pub fn x_label_offset(&mut self, value: i32) -> &mut Self {
        self.x_label_offset = value;
        self
    }

    /// Disable the mesh for the x axis.
    pub fn disable_x_mesh(&mut self) -> &mut Self {
        self.draw_x_mesh = false;
        self
    }

    /// Disable the mesh for the y axis
    pub fn disable_y_mesh(&mut self) -> &mut Self {
        self.draw_y_mesh = false;
        self
    }

    /// Disable drawing the X axis
    pub fn disable_x_axis(&mut self) -> &mut Self {
        self.draw_x_axis = false;
        self
    }

    /// Disable drawing the Y axis
    pub fn disable_y_axis(&mut self) -> &mut Self {
        self.draw_y_axis = false;
        self
    }

    /// Set the style definition for the axis
    /// - `style`: The style for the axis
    pub fn axis_style<T: Into<ShapeStyle<'a>>>(&mut self, style: T) -> &mut Self {
        self.axis_style = Some(style.into());
        self
    }
    /// Set how many labels for the X axis at most
    /// - `value`: The maximum desired number of labels in the X axis
    pub fn x_labels(&mut self, value: usize) -> &mut Self {
        self.n_x_labels = value;
        self
    }

    /// Set how many label for the Y axis at most
    /// - `value`: The maximum desired number of labels in the Y axis
    pub fn y_labels(&mut self, value: usize) -> &mut Self {
        self.n_y_labels = value;
        self
    }

    /// Set the style for the coarse grind grid
    /// - `style`: This is the fcoarse grind grid style
    pub fn line_style_1<T: Into<ShapeStyle<'a>>>(&mut self, style: T) -> &mut Self {
        self.line_style_1 = Some(style.into());
        self
    }

    /// Set the style for the fine grind grid
    /// - `style`: The fine grind grid style
    pub fn line_style_2<T: Into<ShapeStyle<'a>>>(&mut self, style: T) -> &mut Self {
        self.line_style_2 = Some(style.into());
        self
    }

    /// Set the style of the label text
    /// - `style`: The text style that would be applied to the labels
    pub fn label_style<T: Into<TextStyle<'a>>>(&mut self, style: T) -> &mut Self {
        self.label_style = Some(style.into());
        self
    }

    /// Set the formatter function for the X label text
    /// - `fmt`: The formatter function
    pub fn x_label_formatter(&mut self, fmt: &'a dyn Fn(&X::ValueType) -> String) -> &mut Self {
        self.format_x = fmt;
        self
    }

    /// Set the formatter function for the Y label text
    /// - `fmt`: The formatter function
    pub fn y_label_formatter(&mut self, fmt: &'a dyn Fn(&Y::ValueType) -> String) -> &mut Self {
        self.format_y = fmt;
        self
    }

    /// Set the axis description's style. If not given, use label style instead.
    /// - `style`: The text style that would be applied to descriptions
    pub fn axis_desc_style<T: Into<TextStyle<'a>>>(&mut self, style: T) -> &mut Self {
        self.axis_desc_style = Some(style.into());
        self
    }

    /// Set the X axis's description
    /// - `desc`: The description of the X axis
    pub fn x_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.x_desc = Some(desc.into());
        self
    }

    /// Set the Y axis's description
    /// - `desc`: The description of the Y axis
    pub fn y_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.y_desc = Some(desc.into());
        self
    }

    /// Draw the configured mesh on the target plot
    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        let mut target = None;
        std::mem::swap(&mut target, &mut self.target);
        let target = target.unwrap();

        let default_mesh_color_1 = RGBColor(0, 0, 0).mix(0.2);
        let default_mesh_color_2 = RGBColor(0, 0, 0).mix(0.1);
        let default_axis_color = RGBColor(0, 0, 0);
        let default_label_font = FontDesc::new("Arial", 12.0);

        let mesh_style_1 = self
            .line_style_1
            .clone()
            .unwrap_or_else(|| (&default_mesh_color_1).into());
        let mesh_style_2 = self
            .line_style_2
            .clone()
            .unwrap_or_else(|| (&default_mesh_color_2).into());
        let axis_style = self
            .axis_style
            .clone()
            .unwrap_or_else(|| (&default_axis_color).into());

        let label_style =
            unsafe { std::mem::transmute::<_, Option<TextStyle>>(self.label_style.clone()) }
                .unwrap_or_else(|| (&default_label_font).into());

        let axis_desc_style =
            unsafe { std::mem::transmute::<_, Option<TextStyle>>(self.axis_desc_style.clone()) }
                .unwrap_or_else(|| label_style.clone());

        target.draw_mesh(
            (self.n_y_labels * 10, self.n_x_labels * 10),
            &mesh_style_2,
            &label_style,
            |_| None,
            self.draw_x_mesh,
            self.draw_y_mesh,
            self.x_label_offset,
            false,
            false,
            &axis_style,
            &axis_desc_style,
            self.x_desc.clone(),
            self.y_desc.clone(),
        )?;

        target.draw_mesh(
            (self.n_y_labels, self.n_x_labels),
            &mesh_style_1,
            &label_style,
            |m| match m {
                MeshLine::XMesh(_, _, v) => Some((self.format_x)(v)),
                MeshLine::YMesh(_, _, v) => Some((self.format_y)(v)),
            },
            self.draw_x_mesh,
            self.draw_y_mesh,
            self.x_label_offset,
            self.draw_x_axis,
            self.draw_y_axis,
            &axis_style,
            &axis_desc_style,
            None,
            None,
        )
    }
}