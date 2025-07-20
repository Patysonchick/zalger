use crate::smbls::Input;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let output = move || {
        let mut text = Input::new(input.get());
        text.smlr_depth();
        text.obfs()
    };

    view! {
        <div class="flex flex-col justify-center items-center">
            <div class="bg-white text-black rounded-lg p-1">Введи текст</div>
            <input type="text"
                id="text"
                placeholder="Ввод здесь"
                class="border-1 rounded-xl p-2 my-2"
                on:input:target=move |ev| {
                    set_input.set(ev.target().value());
                }
                prop:value=input
            />
        </div>
        <div class="flex flex-col justify-center items-center my-8">
            <div class="bg-white text-black rounded-lg p-2">Вывод</div>
            <textarea id="output" readonly class="border-2 rounded-lg p-2 my-5" prop:value=output>{output}</textarea>
            <button
                class="bg-white text-black rounded-2xl p-3"
                on:click=move |_| {
                let text = output();
                let _ = window().navigator().clipboard().write_text(&text);
            }>
                Скопировать
            </button>
        </div>
    }
}
