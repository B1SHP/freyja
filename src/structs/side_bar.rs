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

    type Message = SideBarMessage;

    type Properties = SideBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_expanded: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            SideBarMessage::MouseClick => {
                self.is_expanded = !self.is_expanded;
                true
            }
            SideBarMessage::MouseLeave => {
                self.is_expanded = false;
                true
            }
        }
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html!{
            <>
                <div onclick={ctx.link().callback(|_| SideBarMessage::MouseClick)} 
                     onmouseleave={ctx.link().callback(|_| SideBarMessage::MouseLeave)}
                    class={ "duration-300 bg-cyan-950 h-screen pl-8 -mt-3.5 -ml-8 pt-5 ".to_string() + (if self.is_expanded { "w-72" } else { "w-24" })}>
                    <SideBarButtonComponent is_over=false />
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
