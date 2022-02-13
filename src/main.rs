mod snippets;

use rand::seq::SliceRandom;
use yew::prelude::*;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <center>
                        <small>
                            { "PEP Talk Generator" }
                        </small>
                    </center>
                </header>
                <main class="container">
                    <h1>
                        {
                            format!("{} {} {} {}", 
                                snippets::FIRST.choose(&mut rand::thread_rng()).unwrap(),
                                snippets::SECOND.choose(&mut rand::thread_rng()).unwrap(),
                                snippets::THIRD.choose(&mut rand::thread_rng()).unwrap(),
                                snippets::FOURTH.choose(&mut rand::thread_rng()).unwrap()
                            )
                        }
                    </h1>
                </main>
                <footer>
                    <center>
                        <small>
                            <a href="">{ "Again!" }</a>
                        </small>
                    </center>
                </footer>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}