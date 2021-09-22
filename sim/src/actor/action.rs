use super::apply::Apply;
use super::QueryActor;
use crate::sim::SimState;
use bevy_ecs::prelude::Entity;
use delegate::delegate;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct Actions(HashMap<u32, Action>);

impl Actions {
    pub fn add(&mut self, action: Action) {
        self.0.insert(action.id, action);
    }

    delegate! {
        to self.0 {
            pub fn get(&self, id: &u32) -> Option<&Action>;
        }
    }
}

#[derive(Default, Clone)]
pub struct Action {
    pub id: u32,
    pub name: &'static str,
    // oGCD indicates this action is off the global cooldown
    pub ogcd: bool,
    pub results: Vec<Arc<dyn Apply + Send + Sync>>,
}

impl Action {
    pub fn perform(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity) {
        for result in &self.results {
            result.apply(sim, query, source, target);
        }
    }
}

#[cfg(test)]
mod tests {}
