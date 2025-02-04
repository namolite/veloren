use crate::{RtState, Rule};
use common::{
    resources::{Time, TimeOfDay},
    rtsim::Actor,
};
use vek::*;
use world::{IndexRef, World};

pub trait Event: Clone + 'static {}

pub struct EventCtx<'a, R: Rule, E: Event> {
    pub state: &'a RtState,
    pub rule: &'a mut R,
    pub event: &'a E,
    pub world: &'a World,
    pub index: IndexRef<'a>,
}

#[derive(Clone)]
pub struct OnSetup;
impl Event for OnSetup {}

#[derive(Clone)]
pub struct OnTick {
    pub time_of_day: TimeOfDay,
    pub time: Time,
    pub tick: u64,
    pub dt: f32,
}
impl Event for OnTick {}

#[derive(Clone)]
pub struct OnDeath {
    pub actor: Actor,
    pub wpos: Option<Vec3<f32>>,
    pub killer: Option<Actor>,
}
impl Event for OnDeath {}
