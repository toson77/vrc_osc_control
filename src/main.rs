use anyhow::Error;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

enum Msg {
    StButtonDown,
    StButtonUp,
    LeftButtonDown,
    LeftButtonUp,
    RightButtonDown,
    RightButtonUp,
    BackButtonDown,
    BackButtonUp,
    JumpButtonDown,
    JumpButtonUp,
}

struct Model {
    ws_read: SplitStream<WebSocket>,
    ws_write: SplitSink<WebSocket, Message>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut ws = WebSocket::open("ws://localhost:8000").unwrap();
        let (mut write, mut read) = ws.split();
        Self {
            ws_read: read,
            ws_write: write,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StButtonDown => {
                // the value has changed so we need to
                // re-render for it to appear on the page
                log::info!("isUp:true");
                spawn_local(async move {
                    self.ws_write
                        .send(Message::Text(String::from(
                            r#"{"addr":/input/Vertical, "args":1}"#,
                        )))
                        .await
                        .unwrap();
                });
                spawn_local(async move {
                    while let Some(msg) = &self.ws_read.next().await {
                        log::info!("{}", format!("1. {:?}", msg));
                    }
                    log::info!("WebSocket closed");
                });

                true
            }
            Msg::StButtonUp => {
                log::info!("isUp:false");
                true
            }
            Msg::BackButtonUp => true,
            Msg::BackButtonDown => true,
            Msg::RightButtonUp => true,
            Msg::RightButtonDown => true,
            Msg::LeftButtonUp => true,
            Msg::LeftButtonDown => true,
            Msg::JumpButtonUp => true,
            Msg::JumpButtonDown => true,
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
                            <button class="px-2 py-1 m-1 bg-blue-400 text-white sm:text-lg font-semibold rounded hover:bg-blue-500" onmousedown={link.callback(|_| Msg::StButtonDown)} onmouseup={link.callback(|_| Msg::StButtonUp)}>{"STRAIGHT"}</button>
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
