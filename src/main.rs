use yew::prelude::*;

#[function_component]
fn App() -> Html {


    html! {
         <div class={classes!("flex", "justify-center", "items-center", "h-screen", "bg-gradient-to-b", "from-gradient_start","via-gradient_middle","to-gradient_end")}>
             
             <img class={classes!("object-cover")} src="https://cdn.discordapp.com/attachments/461549842896781312/1079497406695092376/pet.gif" alt="to jsem ja" width=300/>
             <div class={classes!("py-12")}>
                <iframe src="https://discord.com/widget?id=900127793168392212&theme=dark" width="350" height="500" allowtransparency="true" frameborder="0" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe>
             </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}