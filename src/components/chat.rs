use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
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
            data: Some(username.to_string()),
            data_array: None,
        };

        if wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
            .is_ok()
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: avatar_for(u),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    self.send_chat_message(input.value());
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen bg-[#f6f7f9] text-[#18181b]">
                <div class="flex-none w-64 h-screen bg-white border-r border-[#d8dde5]">
                    <div class="p-5 border-b border-[#d8dde5]">
                        <div class="text-xs uppercase tracking-wide text-[#2563eb] font-bold">{"YewChat"}</div>
                        <div class="text-2xl font-black">{"Chat"}</div>
                    </div>
                    <div class="text-sm font-bold p-4 text-[#6b7280] uppercase">{"Active users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html!{
                                <div class="flex mx-3 mb-3 bg-[#f9fafb] border border-[#d8dde5] rounded-md p-2">
                                    <img class="w-12 h-12 rounded-md" src={u.avatar.clone()} alt="avatar"/>
                                    <div class="flex-grow p-3">
                                        <div class="text-sm font-bold">{u.name.clone()}</div>
                                        <div class="text-xs text-[#6b7280]">{"Online"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    <div class="m-4 rounded-md bg-[#edf4ff] border border-[#bfd3ff] p-4 text-sm text-[#1d4ed8]">
                        <div class="font-bold mb-1">{"Design system"}</div>
                        <div>{"Compact panels, clear borders, and one primary action color."}</div>
                    </div>
                </div>

                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-16 border-b border-[#d8dde5] bg-white flex items-center px-6">
                        <div>
                            <div class="text-lg font-black">{"Chat workspace"}</div>
                            <div class="text-xs text-[#6b7280]">{"WebSocket client at ws://127.0.0.1:8080"}</div>
                        </div>
                    </div>

                    <div class="w-full grow overflow-auto border-b border-[#d8dde5] bg-[#f6f7f9]">
                        if self.messages.is_empty() {
                            <div class="m-8 rounded-lg border border-dashed border-[#c8ced8] bg-white p-8 text-center">
                                <div class="text-xl font-black text-[#18181b]">{"No messages yet"}</div>
                                <div class="mt-2 text-sm text-[#6b7280]">{"Send a message from the composer."}</div>
                            </div>
                        }
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).cloned().unwrap_or(UserProfile {
                                    name: m.from.clone(),
                                    avatar: avatar_for(&m.from),
                                });
                                html!{
                                    <div class="flex items-end max-w-2xl bg-white border border-[#d8dde5] m-8 rounded-lg shadow-sm">
                                        <img class="w-8 h-8 rounded-md m-3" src={user.avatar.clone()} alt="avatar"/>
                                        <div class="p-3">
                                            <div class="text-sm font-bold">{m.from.clone()}</div>
                                            <div class="text-sm text-[#4b5563]">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3 max-w-sm rounded-lg" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    <div class="w-full bg-white p-4">
                        <div class="flex items-center">
                            <input ref={self.chat_input.clone()} type="text" placeholder="Message" class="block w-full py-3 pl-4 mr-3 bg-[#f9fafb] border border-[#c8ced8] rounded-md outline-none focus:ring-2 focus:ring-[#bfd3ff] focus:border-[#2563eb] focus:text-[#18181b]" name="message" required=true />
                            <button onclick={submit} class="p-3 shadow-sm bg-[#2563eb] w-11 h-11 rounded-md flex justify-center items-center color-white">
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                    <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl Chat {
    fn send_chat_message(&mut self, content: String) {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return;
        }

        let message = WebSocketMessage {
            message_type: MsgTypes::Message,
            data: Some(trimmed.to_string()),
            data_array: None,
        };
        if let Err(e) = self
            .wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("error sending to channel: {:?}", e);
        }
    }
}

fn avatar_for(seed: &str) -> String {
    format!(
        "https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}",
        seed
    )
}
