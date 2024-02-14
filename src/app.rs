use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kylerchinmusic.css"/>
        <Stylesheet href="https://use.typekit.net/ffc3xaq.css"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage
                    ssr=SsrMode::Async
                    />
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    view! {
        
        <Title text="Kyler Chin - Singer & Producer"/>
        // sets the document title
        <div 
        class=" gaze1 bg-no-repeat bg-cover bg-right md:bg-center" class="py-96 min-[2000px]:py-[40em]"
        >
        
        <div class="w-full">
        <div class="container mx-auto">
            <div class="md:mx-12 lg:mx-24">
            <h1 class="text-white amster text-4xl font-bold">"Kyler Chin"</h1>
            </div>
        </div>
        </div>
        </div>
        
        <div class="w-full">
        //info box
        <div class="container mx-auto pt-10 ">
        <div class="md:mx-12 lg:mx-24">
        <h2 class="text-3xl font-semibold amster">"Profile"</h2>
        <p><span class="font-bold">{t!(i18n, blood_type)}</span>" "<span>"AB+"</span></p>
        <p><span class="font-bold">{t!(i18n, height)}</span>" "<span>"173 cm"</span></p>
        <p><span class="font-bold">{t!(i18n, weight)}</span>" "<span>"60 kg"</span></p>
        <p><span class="font-bold">{t!(i18n, birthday)}</span>" "<span>{t!(i18n,kylers_birthday)}</span></p>
        </div>

        </div>
        </div>
    }
}

#[component]
fn SwitchLang() -> impl IntoView {
    view! {
        
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
