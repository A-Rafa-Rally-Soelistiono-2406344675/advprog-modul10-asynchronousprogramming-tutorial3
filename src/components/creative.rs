use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Creative)]
pub fn creative() -> Html {
    html! {
        <div class="min-h-screen w-screen bg-[#f6f7f9] text-[#18181b]">
            <div class="mx-auto max-w-5xl px-6 py-8">
                <header class="flex items-center justify-between border-b border-[#d8dde5] pb-6">
                    <div>
                        <div class="text-xs font-bold uppercase tracking-wide text-[#2563eb]">{"Experiment 3.2"}</div>
                        <h1 class="mt-2 text-4xl font-black">{"Design system changes"}</h1>
                    </div>
                    <Link<Route> to={Route::Login}>
                        <button class="rounded-md bg-[#2563eb] px-4 py-2 text-sm font-bold text-white">{"Back"}</button>
                    </Link<Route>>
                </header>

                <main class="grid grid-cols-1 gap-6 py-8 lg:grid-cols-[18rem_1fr]">
                    <aside class="rounded-lg border border-[#d8dde5] bg-white p-5">
                        <div class="text-sm font-black">{"System goal"}</div>
                        <p class="mt-3 text-sm leading-6 text-[#4b5563]">
                            {"Membuat webclient terasa berbeda dengan sistem visual yang lebih sederhana, bukan dengan prompt atau template halaman yang ramai."}
                        </p>
                    </aside>

                    <section class="grid gap-4">
                        <DesignRow title="Color" body="Palet diubah menjadi netral terang dengan biru sebagai warna aksi utama." />
                        <DesignRow title="Shape" body="Radius dibuat kecil dan konsisten supaya komponen terasa lebih utilitarian." />
                        <DesignRow title="Layout" body="Login memakai struktur header, area konten, dan panel form yang jelas." />
                        <DesignRow title="Controls" body="Input dan button dibuat seragam agar alur masuk chat lebih mudah dipindai." />
                    </section>
                </main>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DesignRowProps {
    title: &'static str,
    body: &'static str,
}

#[function_component(DesignRow)]
fn design_row(props: &DesignRowProps) -> Html {
    html! {
        <div class="grid grid-cols-1 gap-3 rounded-lg border border-[#d8dde5] bg-white p-5 sm:grid-cols-[8rem_1fr]">
            <div class="font-black">{props.title}</div>
            <p class="text-sm leading-6 text-[#4b5563]">{props.body}</p>
        </div>
    }
}
