use crate::normalize::{normalize_alpha, normalize_hue, normalize_percent};
use crate::{ColorTuple, ColorTupleA};

#[derive(Debug, PartialEq)]
pub struct Hsl {
  h: f32,
  s: f32,
  l: f32,
  a: Option<f32>,
}

impl Hsl {
  pub fn default() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0, a: None }
  }

  pub fn from(h: f32, s: f32, l: f32) -> Hsl {
    Hsl { h: normalize_hue(h), s: normalize_percent(s), l: normalize_percent(l), a: None }
  }
  pub fn from_with_alpha(h: f32, s: f32, l: f32, a: f32) -> Hsl {
    let a = Some(normalize_alpha(a));
    Hsl { h: normalize_hue(h), s: normalize_percent(s), l: normalize_percent(l), a }
  }

  pub fn from_tuple(t: &ColorTuple) -> Hsl {
    Hsl::from(t.0, t.1, t.2)
  }

  pub fn from_tuple_with_alpha(t: &ColorTupleA) -> Hsl {
    Hsl::from_with_alpha(t.0, t.1, t.2, t.3)
  }

  pub fn as_tuple(&self) -> ColorTuple {
    (self.h, self.s, self.l)
  }

  pub fn as_tuple_with_alpha(&self) -> ColorTupleA {
    (self.h, self.s, self.l, self.get_alpha())
  }

  pub fn get_hue(&self) -> f32 {
    self.h
  }
  pub fn get_saturation(&self) -> f32 {
    self.s
  }
  pub fn get_lightness(&self) -> f32 {
    self.l
  }
  pub fn get_alpha(&self) -> f32 {
    self.a.unwrap_or(1.0)
  }

  pub fn set_hue(&mut self, val: f32) {
    self.h = normalize_hue(val);
  }
  pub fn set_saturation(&mut self, val: f32) {
    self.s = normalize_percent(val);
  }
  pub fn set_lightness(&mut self, val: f32) {
    self.l = normalize_percent(val);
  }
  pub fn set_alpha(&mut self, val: f32) {
    self.a = Some(normalize_alpha(val));
  }
}
