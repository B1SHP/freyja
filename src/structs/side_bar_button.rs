use yew::prelude::*;

pub enum SideBarButtonMsg {
    MouseEnter,
    MouseLeave,
    MouseClick,
}

#[derive(Properties, PartialEq)]
pub struct SideBarButtonProps {
    pub is_over: bool,
}

pub struct SideBarButtonComponent {
    pub is_over: bool,
    pub rotate: bool,
}

impl Component for SideBarButtonComponent {

    type Message = SideBarButtonMsg;

    type Properties = SideBarButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            rotate: false,
            is_over: false,
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {

        match _msg {

            SideBarButtonMsg::MouseLeave => {
                self.is_over = false;
                self.rotate = false;
                true
            }
            SideBarButtonMsg::MouseEnter => {
                self.is_over = true;
                true
            }
            SideBarButtonMsg::MouseClick => {
                self.rotate = !self.rotate;
                true
            }
        } 
    
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let circles: Html = html!{{for ["10", "24", "38"].iter().map(|i| {
            html! {
                <>
                    <circle cx={i.to_string()} cy="10" r="4" fill={if self.is_over { "black" } else { "white" }} />
                    <circle cx={i.to_string()} cy="24" r="4" fill={if self.is_over { "black" } else { "white" }} />
                    <circle cx={i.to_string()} cy="38" r="4" fill={if self.is_over { "black" } else { "white" }} />
                </>
            }
        })}} ;

        html!{
            <>
                <div class="absolute flex justify-center" 
                    onmouseleave={_ctx.link().callback(|_| SideBarButtonMsg::MouseLeave)} 
                    onmouseenter={_ctx.link().callback(|_| SideBarButtonMsg::MouseEnter)}
                    onclick={_ctx.link().callback(|_| SideBarButtonMsg::MouseClick)}>
                    
                    <svg class={ (if self.is_over { "bg-slate-100" } else { "bg-slate-900" }).to_string() + " rounded-md w-12 h-12 duration-300 " + (if self.rotate && self.is_over { "rotate-90" } else { "" }) }>            
                        {  circles  }
                    </svg>
                
                </div>
            </>
        }

    }

}
