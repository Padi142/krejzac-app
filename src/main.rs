use yew::prelude::*;

#[function_component]
fn App() -> Html {


    html! {
         <div class={classes!("flex", "justify-center", "items-center", "h-screen", "bg-gradient-to-b", "from-gradient_start","via-gradient_middle","to-gradient_end")}>
             <img class={classes!("object-cover")} src="https://cdn.discordapp.com/attachments/461549842896781312/1079497406695092376/pet.gif" alt="to jsem ja" width=300/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}