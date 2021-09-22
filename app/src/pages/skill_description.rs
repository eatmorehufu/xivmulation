use yew::prelude::*;
use yew::{html, Html};
use crate::models::{
    skill::Skill,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub skill: Skill,
}

pub struct SkillDescription {    
    skill: Skill,
}

impl Component for SkillDescription {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {        
        Self {
            skill: _props.skill.clone()
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let Self { skill } = self;
        html! {
            html! {
                <li>
                    { &skill.name } 
                </li>
            }
        }
    }
}