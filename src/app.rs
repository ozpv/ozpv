use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::UiEvent;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    view! {
        <div class="bg-black min-h-screen pt-24">
            <h2 class="text-4xl text-white text-center pb-4">"Page not found."</h2>
            <div class="flex justify-center">
                <a href="/" class="flex justify-center bg-slate-800 text-white text-center rounded-full py-5 px-10 hover:bg-slate-600">
                    "Return home"
                </a>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky top-full bg-black py-9">
            <span class="block py-1.5 justify-center text-center text-xs text-white font-sans">"Copyleft (ɔ) 2024-2025 "
                <a href="/" class="hover:underline hover:text-cyan-300">"ozpv"</a>
                ". All Wrongs Reserved."
            </span>
        </footer>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    Effect::new(|| {
        use std::cmp::Ordering;
        use std::rc::Rc;
        use std::sync::Mutex;

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // the <img> elements
        let left_eye = document.get_element_by_id("left-eye").unwrap();
        let right_eye = document.get_element_by_id("right-eye").unwrap();

        // The tuple values must match the ones in the style attribute
        // at least initially
        let eyes = Rc::new(Mutex::new([
            (right_eye, (260.0, 218.0)),
            (left_eye, (386.0, 218.0)),
        ]));

        // on mousemove
        let binding = Rc::clone(&eyes);
        let mousemove_callback = Closure::<dyn FnMut(UiEvent)>::new(move |ev: UiEvent| {
            let px = ev.page_x() as f64;
            let py = ev.page_y() as f64;
            binding.lock().unwrap().iter().for_each(|(eye, (x, y))| {
                let dx = px - x;
                let dy = py - y;
                let r = dx.hypot(dy);
                let sin = dx.abs() / r;
                let cos = dy.abs() / r;
                let A = 35.0;

                let mut fx = *x;
                let mut fy = *y;

                match (dx.total_cmp(&0.0), dy.total_cmp(&0.0)) {
                    (Ordering::Greater, Ordering::Greater) | (Ordering::Equal, Ordering::Equal) => {
                        fx += A * sin;
                        fy += A * cos;
                    }
                    (Ordering::Less, Ordering::Less) => {
                        fx += -A * sin;
                        fy += -A * cos;
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        fx += A * sin;
                        fy += -A * cos;
                    }
                    _ => {
                        fx += -A * sin;
                        fy += A * cos;
                    }
                }

                _ = eye.set_attribute("style", &format!("left:{fx}px;top:{fy}px;"));
            });
        });

        window
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_callback.as_ref().unchecked_ref(),
            )
            .unwrap();

        // on resize
        let binding = Rc::clone(&eyes);
        let resize_callback = Closure::<dyn Fn()>::new(move || {
            binding.lock().unwrap().iter().for_each(|_eye| {});
        });

        window
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
            .unwrap();

        // otherwise they will leak
        resize_callback.forget();
        mousemove_callback.forget();
    });

    view! {
        <div class="bg-black min-h-screen">
            <h1 class="text-8xl text-center text-white py-24">ozpv</h1>
            <div class="relative w-[749px] h-[435px]" id="rustacean-image">
                <img src="rustacean-no-eyes.png" id="rustacean" class="z-10 absolute"/>
                <img src="rustacean-eye.png" id="right-eye" class="z-0 absolute" style="left:260px;top:218px;"/>
                <img src="rustacean-eye.png" id="left-eye" class="z-0 absolute" style="left:386px;top:218px;"/>
            </div>
            <div class="flex justify-center gap-5 pt-5">
                <a href="https://github.com/ozpv" class="p-2 rounded-sm transition-all ease-in duration-150 hover:-translate-y-px hover:bg-slate-800">
                    <Icon icon={icondata::SiGithub} width="32" height="32" {..} class="text-white" />
                </a>
                <a href="https://github.com/ozpv/ozpv" class="p-2 rounded-sm transition-all ease-in duration-150 hover:-translate-y-px hover:bg-slate-800">
                    <Icon icon={icondata::SiGit} width="32" height="32" {..} class="text-white" />
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ozpv.css"/>
        <Title text="ozpv"/>

        <Router>
            <main>
                <Routes fallback=|| view! { <ErrorPage /> }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
