use yew::{function_component, html, Html, Properties, classes};
use log::{info};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    // props go here
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let board = [
        [1,2,2,1,2,2,1],
        [2,3,1,3,1,3,2],
        [1,2,2,2,3,2,1],
        [2,1,2,3,2,1,2],
        [1,2,3,2,2,2,1],
        [2,3,1,3,1,3,2],
        [1,2,2,1,2,2,1],
    ];
    html! {
        <div class="board">
            {
                board.iter().enumerate().map(|(y,row)| {
                    row.iter().enumerate().map(|(x,cell)| {
                        html! {
                            // TODO: use a custom component for each cell, props should be
                            //      the cell value and the cell should be clickable
                            <div class={classes!(
                                "cell", 
                                (cell == &1).then(|| "white"), 
                                (cell == &2).then(|| "black"), 
                                (cell == &3).then(|| "pink"))}>
                            </div>
                        }
                      }).collect::<Html>()
                    }).collect::<Html>()
            }
        </div>
    }
}