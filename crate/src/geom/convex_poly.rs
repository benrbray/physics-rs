extern crate nalgebra as nalg;
use std::f32::consts::PI;

use nalg::OMatrix;

type PointMatrix<C> = OMatrix<f32, nalg::U2, C>;

pub struct ConvexPoly {
  pub points: PointMatrix<nalg::Dyn>
}

impl ConvexPoly {
  // construct a regular polygon
  pub fn regular(
    num_points: u16,
    scale: f32
  ) -> ConvexPoly {
    assert!(num_points > 2);

    let mut data = Vec::<f32>::new(); 
    let angle = 2.0 * PI / f32::from(num_points);

    for n in 0..num_points {
      let theta = angle * n as f32;
      let x = scale * f32::cos(theta);
      let y = scale * f32::sin(theta);
      data.push(x);
      data.push(y);
    }

    ConvexPoly { points: PointMatrix::<nalg::Dyn>::from_vec(data) }
  }
}