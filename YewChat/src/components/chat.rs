use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleTheme,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    is_dark: bool,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username),
            data_array: None,
        };

        let _ = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap());

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            is_dark: false,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        self.users = msg
                            .data_array
                            .unwrap_or_default()
                            .iter()
                            .map(|u| UserProfile {
                                name: u.clone(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                ),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        if let Some(data) = msg.data {
                            if let Ok(message_data) = serde_json::from_str::<MessageData>(&data) {
                                self.messages.push(message_data);
                                return true;
                            }
                        }
                        false
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let text = input.value();
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(text),
                        data_array: None,
                    };
                    let _ = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap());
                    input.set_value("");
                }
                false
            }
            Msg::ToggleTheme => {
                self.is_dark = !self.is_dark;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_theme = ctx.link().callback(|_| Msg::ToggleTheme);

        let theme_class = if self.is_dark { "bg-gray-900 text-white" } else { "bg-white text-black" };
        let sidebar_bg = if self.is_dark { "bg-gray-800" } else { "bg-gray-100" };
        let user_card_bg = if self.is_dark { "bg-gray-700" } else { "bg-white" };
        let message_bg = if self.is_dark { "bg-gray-700" } else { "bg-gray-100" };
        let input_bg = if self.is_dark { "bg-gray-800 text-white" } else { "bg-gray-100 text-black" };

        html! {
            <div class={classes!("flex", "w-screen", theme_class)}>
                <div class={classes!("flex-none", "w-56", "h-screen", sidebar_bg)}>
                    <div class="text-xl p-3">{"Users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html! {
                                <div class={classes!("flex", "m-3", user_card_bg.clone(), "rounded-lg", "p-2")}>
                                    <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar" />
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-gray-400">{"Hi there!"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-14 border-b-2 border-gray-300 flex justify-between items-center px-3">
                        <div class="text-xl">{"💬 Chat!"}</div>
                        <button onclick={toggle_theme} class="text-sm bg-gray-600 text-white px-3 py-1 rounded">
                            { if self.is_dark { "☀ Light Mode" } else { "🌙 Dark Mode" } }
                        </button>
                    </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300">
                        {
                            self.messages.iter().map(|m| {
                                let user_opt = self.users.iter().find(|u| u.name == m.from);
                                html! {
                                    <div class={classes!("flex", "items-end", "w-3/6", message_bg.clone(), "m-8", "rounded-tl-lg", "rounded-tr-lg", "rounded-br-lg")}>
                                        if let Some(user) = user_opt {
                                            <img class="w-8 h-8 rounded-full m-3" src={user.avatar.clone()} alt="avatar"/>
                                        }
                                        <div class="p-3">
                                            <div class="text-sm">{ &m.from }</div>
                                            <div class="text-xs text-gray-500">
                                                {
                                                    if m.message.ends_with(".gif") {
                                                        html! { <img class="mt-3" src={m.message.clone()} /> }
                                                    } else {
                                                        html! { &m.message }
                                                    }
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="w-full h-14 flex px-3 items-center">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" class={classes!("block", "w-full", "py-2", "pl-4", "mx-3", input_bg.clone(), "rounded-full", "outline-none", "focus:text-gray-700")} />
                        <button onclick={submit} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"/>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
