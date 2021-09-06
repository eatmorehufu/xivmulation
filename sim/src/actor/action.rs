use super::apply::Apply;
use super::QueryActor;
use crate::sim::SimTime;
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
    pub name: String,
    pub target_self: bool,
    pub ogcd: bool,
    // recast_duration is the duration of time in ms till the action can be performed again.
    pub recast_duration: SimTime,
    pub results: Vec<Arc<dyn Apply + Send + Sync>>,
}

impl Action {
    pub fn perform(
        &self,
        sim_time: SimTime,
        query: &mut QueryActor,
        source: Entity,
        target: Entity,
    ) {
        for result in &self.results {
            if self.target_self {
                result.apply(sim_time, query, source, source);
            } else {
                result.apply(sim_time, query, source, target);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
