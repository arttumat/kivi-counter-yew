use yew::prelude::*;
use crate::Board;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{"Kivi-laskuri"}</h1>
            <p>{"Naputtele laudalle yhden pelaajan kivet kerrallaan saadaksesi tuloksen."}</p>
            <Board />
        </main>
    }
}