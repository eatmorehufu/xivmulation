use yew::prelude::*;
use yew::{html, Html};
use crate::models::{
    skill::Skill, job::Job,
};
use super::skills_panel::SkillsPanel;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub job_name: String,
}

pub struct JobPage {    
    job: Job,    
}

impl Component for JobPage {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // TODO: Load skills for the job by name.
        Self {
            job: Job {
                name: _props.job_name,
                skills: vec![
                    Skill {
                        id: "1".to_string(),
                        name: "Fast Blade".to_string(),
                        description: "it's fast".to_string(),
                        icon_url: "some_url".to_string(),
                    }
                ]
            }
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let Self { job } = self;
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Job Page" }</h1>
                        <h2 class="subtitle">{ &job.name }</h2>
                    </div>
                    <SkillsPanel skills=job.skills.clone()/>
                </div>
            </div>
        }
    }
}
