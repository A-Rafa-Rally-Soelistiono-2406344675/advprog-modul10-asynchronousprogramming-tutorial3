use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen w-screen bg-[#f6f7f9] text-[#18181b]">
            <div class="mx-auto flex min-h-screen max-w-6xl flex-col px-6">
                <header class="flex h-20 items-center justify-between border-b border-[#d8dde5]">
                    <div class="flex items-center gap-3">
                        <div class="grid h-9 w-9 place-items-center rounded-md bg-[#2563eb] text-sm font-black text-white">{"YC"}</div>
                        <div>
                            <div class="text-sm font-black tracking-wide">{"YewChat"}</div>
                            <div class="text-xs text-[#6b7280]">{"simple client system"}</div>
                        </div>
                    </div>
                </header>

                <main class="grid flex-1 grid-cols-1 items-center gap-8 py-10 lg:grid-cols-[1fr_24rem]">
                    <section>
                        <div class="max-w-2xl">
                            <div class="mb-5 inline-flex rounded-md border border-[#bfd3ff] bg-[#edf4ff] px-3 py-2 text-xs font-bold uppercase tracking-wide text-[#1d4ed8]">
                                {"Experiment 3.2"}
                            </div>
                            <h1 class="text-5xl font-black leading-tight tracking-normal lg:text-6xl">
                                {"System Chat Client Sederhana"}
                            </h1>
                            <p class="mt-5 max-w-xl text-base leading-7 text-[#4b5563]">
                                {"Client chat dibuat lebih sederhana supaya user bisa langsung masuk, mengetik pesan, dan melihat ruang percakapan tanpa elemen yang mengganggu."}
                            </p>
                        </div>
                    </section>

                    <section class="rounded-lg border border-[#d8dde5] bg-white p-5 shadow-sm">
                        <div class="mb-5 flex items-center justify-between">
                            <div>
                                <div class="text-xl font-black">{"Enter chat"}</div>
                                <div class="text-sm text-[#6b7280]">{"Use a username to join."}</div>
                            </div>
                            <img class="h-12 w-12 rounded-md border border-[#d8dde5]" src="https://api.dicebear.com/7.x/shapes/svg?seed=yewchat-system" alt="avatar" />
                        </div>

                        <form class="grid gap-3">
                            <input {oninput} class="w-full rounded-md border border-[#c8ced8] bg-[#f9fafb] px-4 py-3 text-[#18181b] outline-none focus:border-[#2563eb] focus:bg-white focus:ring-2 focus:ring-[#bfd3ff]" placeholder="Username" />
                            <Link<Route> to={Route::Chat}>
                                <button {onclick} disabled={username.len()<1} class="w-full rounded-md bg-[#2563eb] px-4 py-3 font-bold text-white disabled:bg-[#d1d5db] disabled:text-[#6b7280]">
                                    {"Start chat"}
                                </button>
                            </Link<Route>>
                        </form>
                    </section>
                </main>
            </div>
        </div>
    }
}
