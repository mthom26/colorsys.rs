//! All color units is f32. Here are their ranges:
//! - red - 0.0 .. 255.0
//! - green - 0.0 .. 255.0
//! - blue - 0.0 .. 255.0
//! - hue - 0.0 .. 360.0
//! - saturation - 0.0 .. 100.0
//! - lightness - 0.0 .. 100.0
//! - alpha - 0.0 .. 1.0
//!
//! If you specify a value that does not fit within these ranges, they are replaced with a minimum or maximum value.
mod colors;
mod converters;
mod error;
mod from_str;
mod normalize;

#[cfg(test)]
mod tests;

pub use colors::{Hsl, Hsla, Rgb, Rgba};

pub use error::ParseError;

/// Tuple type just for data exchange. May be a `($red,$green,$blue)` or `($hue, $saturation, $lightness)`
/// # Example
/// ```
/// use colors_transform::{Rgb,Color,Hsl};
///
/// let rgb = Rgb::from_tuple((255.0,13.0,177.0));
/// assert_eq!(rgb.get_red(), 255.0);
///
/// let hsl: Hsl = "hsl(315,99,12)".parse().unwrap();
/// assert_eq!(hsl.get_saturation(), 99.0);
/// ```
pub type ColorTuple = (f32, f32, f32);

/// Like a `ColorTuple` but with fourth alpha value
pub type ColorTupleA = (f32, f32, f32, f32);

/// Common to all trait
pub trait Color {
  type Tuple;
  fn new() -> Self;

  fn from_tuple(tuple: Self::Tuple) -> Self;
  fn as_tuple(&self) -> Self::Tuple;

  fn get_red(&self) -> f32;
  fn get_green(&self) -> f32;
  fn get_blue(&self) -> f32;
  fn set_red(&self, val: f32) -> Self;
  fn set_green(&self, val: f32) -> Self;
  fn set_blue(&self, val: f32) -> Self;

  fn get_hue(&self) -> f32;
  fn get_saturation(&self) -> f32;
  fn get_lightness(&self) -> f32;
  fn set_hue(&self, val: f32) -> Self;
  fn set_saturation(&self, val: f32) -> Self;
  fn set_lightness(&self, val: f32) -> Self;

  fn to_rgb(&self) -> Rgb;
  fn to_rgba(&self) -> Rgba;
  fn to_hsl(&self) -> Hsl;
  fn to_hsla(&self) -> Hsla;

  /// Returns css string
  /// # Example
  /// ```
  /// use colors_transform::{Hsl,Color};
  ///
  /// let hsl = Hsl::from_tuple((301.0,27.0,91.0));
  /// assert_eq!(hsl.to_css_string(), "hsl(301,27%,91%)");
  /// ```
  fn to_css_string(&self) -> String;

  fn adjust_hue(&self, amt: f32) -> Self;
  fn saturate(&self, amt: f32) -> Self;
  fn lighten(&self, amt: f32) -> Self;
  fn adjust_color(&self, col_name: RgbColor, val: f32) -> Self;
}

/// Some methods for working with alpha channel for Rgba & Hsla
pub trait AlphaColor {
  fn get_alpha(&self) -> f32;
  fn set_alpha(&self, a: f32) -> Self;
  fn opacify(&self, o: f32) -> Self;
}

#[derive(Clone, Copy)]
pub enum RgbColor {
  Red,
  Green,
  Blue,
}
