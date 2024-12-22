use leptos::{
    ev, html,
    prelude::*,
    tachys::{dom::window, renderer::dom::Dom},
};
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use web_sys::HtmlImageElement;

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
fn DesktopHomePage() -> impl IntoView {
    use std::cmp::Ordering;

    let rustacean = NodeRef::<html::Div>::new();
    let left_eye = NodeRef::new();
    let right_eye = NodeRef::new();

    // set position of center of eye hole relative to parent
    let eyes = [(right_eye, (260.0, 218.0)), (left_eye, (386.0, 218.0))];

    // on mousemove
    let mousemove_handle = window_event_listener(ev::mousemove, move |ev| {
        let px = f64::from(ev.page_x());
        let py = f64::from(ev.page_y());

        let rect = rustacean
            .get()
            .expect("rustacean <div> to exist")
            .get_bounding_client_rect();

        let parent_x = rect.x();
        let parent_y = rect.y();

        eyes.iter()
            .filter_map(|(eye, positions)| eye.get().map(|eye| (eye, positions)))
            .for_each(|(eye, positions): (HtmlImageElement, &(f64, f64))| {
                let dx = px - (positions.0 + parent_x);
                let dy = py - (positions.1 + parent_y);
                let r = dx.hypot(dy);
                let sin = dx.abs() / r;
                let cos = dy.abs() / r;
                let A = 35.0;

                let mut fx = positions.0;
                let mut fy = positions.1;

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

                Dom::set_attribute(&eye, "style", &format!("left:{fx}px;top:{fy}px;"));
            });
    });

    on_cleanup(move || mousemove_handle.remove());

    view! {
        <div class="relative w-[749px] h-[435px] m-auto translate-x-6" node_ref=rustacean>
            <img src="rustacean-no-eyes.png" class="z-10 absolute"/>
            <img src="rustacean-eye.png" class="z-0 absolute" style="left:260px;top:218px;" node_ref=right_eye/>
            <img src="rustacean-eye.png" class="z-0 absolute" style="left:386px;top:218px;" node_ref=left_eye/>
        </div>
    }
}

#[component]
fn MobileHomePage() -> impl IntoView {
    view! {
        <img src="rustacean.png" class="mx-auto"/>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let small = RwSignal::new(false);

    Effect::new(move || {
        let Some(width) = window()
            .inner_width()
            .expect("window to have a width")
            .as_f64()
        else {
            return;
        };

        // rustacean width plus the translation of 6.0
        if width < 755.0 {
            small.set(true);
        }
    });

    view! {
        <div class="bg-black min-h-screen">
            <h1 class="text-6xl text-center text-white py-24 sm:text-8xl">ozpv</h1>
            <Show when=move || small.get() fallback=DesktopHomePage>
                <MobileHomePage />
            </Show>
            <div class="flex justify-center gap-5 pt-14">
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
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ozpv.css"/>
        <Title text="ozpv"/>

        <Router>
            <main>
                <Routes fallback=ErrorPage>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
