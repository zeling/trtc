use cucumber::{cucumber, before, after};
use trtc::tuples::Tuple;

#[derive(Default)]
pub struct TRTCWorld {
    t: Tuple
}

impl cucumber::World for TRTCWorld {}

mod tuple_steps {
    use cucumber::steps;
    use super::*;

    steps!(crate::TRTCWorld => {
        given "a ← tuple(4.3, -4.2, 3.1, 1.0)" |world, _step|{
            world.t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        };
        then "a.x = 4.3" |world, _step| {
            assert_eq!(world.t.x, 4.3);
        };
        then "a.y = -4.2" |world, _step| {
            assert_eq!(world.t.y, -4.2);
        };
        then "a.z = 3.1" |world, _step| {
            assert_eq!(world.t.z, 3.1);
        };
        then "a.w = 1.0" |world, _step| {
            assert_eq!(world.t.w, 1.0);
        };
        then "a is a point" |world, _step| {
            assert!(world.t.is_point());
        };
        then "a is not a vector" |world, _step| {
            assert!(!world.t.is_vector());
        };
        given "a ← tuple(4.3, -4.2, 3.1, 0.0)" |world, _step|{
            world.t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        };
        then "a.w = 0.0" |world, _step| {
            assert_eq!(world.t.w, 0.0);
        };
        then "a is not a point" |world, _step| {
            assert!(!world.t.is_point());
        };
        then "a is a vector" |world, _step| {
            assert!(world.t.is_vector());
        };

    });
}

fn setup() {}

before!(before_fn => |_| {});
after!(after_fn => |_| {});

cucumber! {
    features: "./features",
    world: crate::TuplesWorld,
    steps: &[tuple_steps::steps],
    setup: setup,
    before: &[before_fn],
    after: &[after_fn]
}