use yew::prelude::{Component, Context, html, Html, Properties };

use crate::structs::side_bar_button::SideBarButtonComponent;

pub enum SideBarMessage{
    MouseClick,
    MouseLeave,
}

#[derive(Properties, PartialEq)]
pub struct SideBarProps;

pub struct SideBarComponent{
    pub is_expanded: bool,
}

impl Component for SideBarComponent {

    type Message = ();

    type Properties = SideBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_expanded: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
           html!{
            <>
                <div class="absolute duration-300 bg-cyan-950 h-screen -mt-6 -ml-6 p-5 pt-8">
                    <SideBarButtonComponent is_over=false/>
                </div>
            </>
        }
    }

}

//'absolute': true,
//'duration-300': true,
//'bg-cyan-950': true,
//'h-screen': true,
//'-ml-6': true,
//'2xl:mt-16 notebook:mt-14': true,
//'p-5': true,
//'pt-8': true,
//'w-72': this.isSidebarExpanded,
//'w-20': !this.isSidebarExpanded/
