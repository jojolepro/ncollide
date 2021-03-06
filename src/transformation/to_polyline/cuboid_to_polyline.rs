use super::ToPolyline;
use alga::general::Real;
use na;
use crate::procedural;
use crate::procedural::Polyline;
use crate::shape::Cuboid;

impl<N: Real> ToPolyline<N> for Cuboid<N> {
    type DiscretizationParameter = ();

    fn to_polyline(&self, _: ()) -> Polyline<N> {
        let _2: N = na::convert(2.0f64);

        procedural::rectangle(&(*self.half_extents() * _2))
    }
}
