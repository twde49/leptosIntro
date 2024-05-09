use leptos::*;
use std::thread::*;
use std::time::Duration;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App()->impl IntoView{
    let (count,set_count) = create_signal(0);
    let double_count = move || count()*2;
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n +=1)
            }
            class:red=move || count() % 2==1
            style="position: absolute"
                //style:left=move || format!("{}px", count() + 100)
                //style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
                //style:min-width="400px"
                //style=("--columns", count)
        >
            "Click me: "
            {count}
        </button>
        <ProgressBar max=10 progress=count />x
        <ProgressBar max=10 progress=double_count />x
    }
}

#[component]
fn ProgressBar<F:Fn() -> i32 + 'static,>(
    #[prop(default=100)]
    max: usize,
    progress: F
)->impl IntoView {
    view! {
        <progress
            class="middle"
            max=max
            value=progress
        />
    }
}