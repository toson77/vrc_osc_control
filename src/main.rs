use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="whole ">
                <div class="direction-area justify-center">
                    <header class="header py-2 md:py-4 border-b-2 mb-4">
                        <h1 class="text-2xl font-bold">{"VRC-OSC-ControlPanel"}</h1>
                    </header>
                    <div class="btn-wrapper h-80 grid grid-cols-3 grid-rows-3 flex justify-center max-w-screen-md ">
                        <div class="p-1 justify-self-center"></div>
                            <button class="px-2 py-1 m-1 bg-blue-400 text-white sm:text-lg font-semibold rounded hover:bg-blue-500">{"STRAIGHT"}</button>
                        <div class="p-1 justify-self-center"></div>
                            <button class=" px-2 py-1 m-1 bg-blue-400 text-white sm:text-lg font-semibold rounded hover:bg-blue-500">{"LEFT"}</button>
                            <button class="px-2 py-1 m-1 bg-blue-400 text-white sm:text-lg font-semibold rounded hover:bg-blue-500">{"JUMP"}</button>
                            <button class="px-2 py-1 m-1 bg-blue-400 text-white sm:text-lg font-semibold rounded hover:bg-blue-500">{"RIGHT"}</button>
                        <div class="p-1 justify-self-center"></div>
                            <button class="px-2 py-1 m-1 bg-blue-400 text-white sm:text-lg font-semibold rounded hover:bg-blue-500">{"BACK"}</button>
                        <div class="p-1 justify-self-center"></div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
