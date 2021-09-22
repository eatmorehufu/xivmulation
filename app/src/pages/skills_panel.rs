use super::skill_description::SkillDescription;
use crate::models::skill::Skill;
use yew::prelude::*;
use yew::{html, Html};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub skills: Vec<Skill>,
}

pub struct SkillsPanel {
    skills: Vec<Skill>,
}

impl Component for SkillsPanel {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            skills: _props.skills.to_vec(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let Self { skills } = self;
        html! {
            <ul class="item-list">
              { for skills.iter().map(|skill| html! {
                  <SkillDescription skill=skill.clone()/>
                })}
            </ul>
        }
    }
}
