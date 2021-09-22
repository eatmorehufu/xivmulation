use delegate::delegate;
use std::collections::HashSet;

#[derive(Default)]
pub struct ActiveCombos(HashSet<u32>);

impl ActiveCombos {
    delegate! {
        to self.0 {
            #[call(contains)]
            pub fn has_action(&self, action_id: &u32) -> bool;
            #[call(insert)]
            pub fn add_action(&mut self, action_id: u32);
            #[call(remove)]
            pub fn remove_action(&mut self, action_id: &u32);
            #[call(clear)]
            pub fn reset(&mut self);
        }
    }
}
