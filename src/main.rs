use yew::prelude::*;

use crate::structs::side_bar::SideBarComponent;

mod structs {
    pub mod side_bar;
    pub mod side_bar_button;
}

#[function_component(Site)]
fn site() -> Html {

    html! {
        <>
        <header class="shadow-sm bg-gradient-to-r from-slate-900 via-slate-700 to-slate-800">
            <nav class="container mx-auto 2xl:p-5 notebook:p-3">
                <div class="flex justify-around">
                    <div class="fixed left-4 top-3.5" >
                        <SideBarComponent /> 
                    </div>

                    <div class="">
                        <h1 class="bg-slate-500 rounded-md text-white text-3xl py-1 px-10">{ "Freyja" }</h1>
                    </div>

                    <div class="fixed right-5">
                    </div>
                </div>
            </nav>
        </header>

        <div class="ml-24 mr-5 pt-4">

        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<Site>::new().render();
}
