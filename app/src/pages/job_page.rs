use yew::prelude::*;
use yew::{html, Html};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub job_name: String,
}

#[derive(Debug)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: String,    
}

// Should this be the same job struct as in the sim?
#[derive(Debug)]
pub struct Job {
    pub name: String,
    pub skills: Vec<Skill>,
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
                    <ul class="item-list">
                      { for self.job.skills.iter().map(|skill| self.render_skill(skill))}
                    </ul>
                </div>

            </div>
        }
    }
}

impl JobPage {
    fn render_skill(&self, skill: &Skill) -> Html {
        html! {
            <li>
                { &skill.name } 
            </li>
        }
    }
}