use yew::prelude::*;

pub struct About;
impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h2 class="title is-1">{ "Developers" }</h2> 
                    </div>
                    <div class="hero-body container pb-0">
                        <p>{ "Made by eatmorehufu@ and capoferro@ on GitHub." }</p> 
                    </div>
                    <div class="hero-body container pb-0">
                        <h2 class="title is-1">{ "Features" }</h2> 
                    </div>
                    <div class="hero-body container pb-0">
                        <p>{ "Placeholder for feature list." }</p> 
                    </div>
                </div>

            </div>
        }
    }
}