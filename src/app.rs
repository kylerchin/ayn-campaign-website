use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.

    view! {
           // <!-- Google tag (gtag.js) -->
    <Script async_="true" src="https://www.googletagmanager.com/gtag/js?id=G-NJ3G60BR1Z"/>
    <Script>
"window.dataLayer = window.dataLayer || [];
function gtag(){dataLayer.push(arguments);}
gtag('js', new Date());

gtag('config', 'G-NJ3G60BR1Z');"
    </Script>
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/kylerchinmusic.css"/>
            <Stylesheet href="https://use.typekit.net/ffc3xaq.css"/>

            <Script async_="true">
            "
        (function(d) {
          var config = {
            kitId: 'wfo4vht',
            scriptTimeout: 3000,
            async: true
          },
          h=d.documentElement,t=setTimeout(function(){h.className=h.className.replace(/\\bwf-loading\\b/g,"")+\" wf-inactive\";},config.scriptTimeout),tk=d.createElement(\"script\"),f=false,s=d.getElementsByTagName(\"script\")[0],a;h.className+=\" wf-loading\";tk.src='https://use.typekit.net/'+config.kitId+'.js';tk.async=true;tk.onload=tk.onreadystatechange=function(){a=this.readyState;if(f||a&&a!=\"complete\"&&a!=\"loaded\")
          return;f=true;clearTimeout(t);try{Typekit.load(config)}catch(e){}};s.parentNode.insertBefore(tk,s)
        })(document);"
          </Script>

            // content for this welcome page
            <Router>
                <main class="flex flex-col h-full">
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

#[derive(Clone)]
struct VideoData {
    link: String,
    thumbnail_url: String,
}

#[derive(Clone)]
struct KylersVideoData {
    videos: Vec<VideoData>,
}

fn data() -> KylersVideoData {
    KylersVideoData {
        videos: vec![VideoData {
            link: String::from("https://www.youtube.com/watch?v=4ZvBIdLGkw0"),
            thumbnail_url: String::from("https://i3.ytimg.com/vi/4ZvBIdLGkw0/maxresdefault.jpg"),
        }],
    }
}

#[component]
fn TransparentNav() -> impl IntoView {
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    view! {
        <div class="flex flex-row">
            <div>"Kyler Chin"</div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    leptos_meta::provide_meta_context();
    provide_i18n_context();
    let i18n = use_i18n();

    view! {
        <div class="w-full  bg-[#101010] text-white px-8 py-4 text-sm  mt-auto">


       <div class="container mx-full"> <p class="underline"><a href="https://computing.kylerchin.com">"Computing"</a></p>
       <p>{t!(i18n, email)} ": kyler@yk3music.com"</p>
       <br/>

       <p>"© Yk3 Music 2024"</p></div>
    </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    let kylers_video_data = data();

    view! {

        <Title text={t!(i18n, title)}/>
        // sets the document title
        <div
        class=" gaze1 bg-no-repeat bg-cover bg-right md:bg-center"
        >

        <div class="w-full">
        <div class="container mx-auto ">
        <Navbar
        white_text={true}
        />
        </div>
        <div class="container mx-auto py-48 md:py-96 min-[20000px]:py-[40em]">
            <div class="mx-6 fadeintop md:mx-12 lg:mx-24">
            <h1 class="text-white drop-shadow amster text-3xl md:text-4xl font-bold">"Kyler Chin"</h1>
            <p class="text-white drop-shadow">{t!(i18n, shorttitle)}</p>
            <div class="flex flex-row mt-3 gap-x-3">
                <div>
                <a href="https://youtube.com/kylerchin" target="_blank"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="h-8 text-white"><path fill="currentColor" d="M10,15L15.19,12L10,9V15M21.56,7.17C21.69,7.64 21.78,8.27 21.84,9.07C21.91,9.87 21.94,10.56 21.94,11.16L22,12C22,14.19 21.84,15.8 21.56,16.83C21.31,17.73 20.73,18.31 19.83,18.56C19.36,18.69 18.5,18.78 17.18,18.84C15.88,18.91 14.69,18.94 13.59,18.94L12,19C7.81,19 5.2,18.84 4.17,18.56C3.27,18.31 2.69,17.73 2.44,16.83C2.31,16.36 2.22,15.73 2.16,14.93C2.09,14.13 2.06,13.44 2.06,12.84L2,12C2,9.81 2.16,8.2 2.44,7.17C2.69,6.27 3.27,5.69 4.17,5.44C4.64,5.31 5.5,5.22 6.82,5.16C8.12,5.09 9.31,5.06 10.41,5.06L12,5C16.19,5 18.8,5.16 19.83,5.44C20.73,5.69 21.31,6.27 21.56,7.17Z" /></svg>
                </a></div>
                <div>
                <a href="https://instagram.com/kyler.chin" target="_blank"> <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="h-8 text-white">
                <path fill="currentColor"
                d="M7.8,2H16.2C19.4,2 22,4.6 22,7.8V16.2A5.8,5.8 0 0,1 16.2,22H7.8C4.6,22 2,19.4 2,16.2V7.8A5.8,5.8 0 0,1 7.8,2M7.6,4A3.6,3.6 0 0,0 4,7.6V16.4C4,18.39 5.61,20 7.6,20H16.4A3.6,3.6 0 0,0 20,16.4V7.6C20,5.61 18.39,4 16.4,4H7.6M17.25,5.5A1.25,1.25 0 0,1 18.5,6.75A1.25,1.25 0 0,1 17.25,8A1.25,1.25 0 0,1 16,6.75A1.25,1.25 0 0,1 17.25,5.5M12,7A5,5 0 0,1 17,12A5,5 0 0,1 12,17A5,5 0 0,1 7,12A5,5 0 0,1 12,7M12,9A3,3 0 0,0 9,12A3,3 0 0,0 12,15A3,3 0 0,0 15,12A3,3 0 0,0 12,9Z" /></svg>
                </a></div>
            </div>
            </div>
        </div>
        </div>
        </div>

        <div class="w-full">
        //info box
        <div class="container mx-auto pt-10 ">
        <div class="mx-4 md:mx-12 lg:mx-24">
        <h2 class="text-3xl font-semibold amster">{t!(i18n,profile)}</h2>
        <p><span class="font-bold">{t!(i18n, blood_type)}</span>" "<span>"AB+"</span></p>
        <p><span class="font-bold">{t!(i18n, height)}</span>" "<span>"173 cm"</span></p>
        <p><span class="font-bold">{t!(i18n, weight)}</span>" "<span>"59 kg"</span></p>
        <p><span class="font-bold">{t!(i18n, birthday)}</span>" "<span>{t!(i18n,kylers_birthday)}</span></p>

        <br/>

        <h2 class="text-3xl font-semibold amster">{t!(i18n, discography)}</h2>
        <p>{t!(i18n, comingsoon)}</p>
        </div>
        <br/>
        <br/>



        </div>

        </div>
        <Footer/>
    }
}

fn invert_bool(n: &mut bool) {
    if *n == false {
        *n = true;
    } else {
        *n = false;
    }
}

#[component]
fn PickTranslation(set_opened: WriteSignal<bool>) -> impl IntoView {
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    leptos_i18n::load_locales!();

    let avaliable_lang = vec![(Locale::en, "English"), (Locale::ko, "한국어")];

    view! {
        {
            avaliable_lang.into_iter().map(move |lang_pair| {
                view! {

                    <div on:click=move |_| {i18n.set_locale(lang_pair.0); set_opened.update(|n| *n = false);}>
                        {lang_pair.1}
                    </div>
                }
            })
            .collect_view()
        }
    }
}

#[component]
fn Navbar(white_text: bool) -> impl IntoView {
    let text_colour = match white_text {
        true => "text-white",
        false => "text-black",
    };

    let (opened, set_opened) = create_signal(false);

    view! {
            <div class="flex flex-row py-4 mx-4">
                <div class="ml-auto">
                <div class="relative inline-block text-left">

                <button on:click=move |_| set_opened.update(|n| invert_bool(n))>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class={format!("w-6 h-6 {}", text_colour)}>
      <path fill-rule="evenodd" d="M9 2.25a.75.75 0 0 1 .75.75v1.506a49.384 49.384 0 0 1 5.343.371.75.75 0 1 1-.186 1.489c-.66-.083-1.323-.151-1.99-.206a18.67 18.67 0 0 1-2.97 6.323c.318.384.65.753 1 1.107a.75.75 0 0 1-1.07 1.052A18.902 18.902 0 0 1 9 13.687a18.823 18.823 0 0 1-5.656 4.482.75.75 0 0 1-.688-1.333 17.323 17.323 0 0 0 5.396-4.353A18.72 18.72 0 0 1 5.89 8.598a.75.75 0 0 1 1.388-.568A17.21 17.21 0 0 0 9 11.224a17.168 17.168 0 0 0 2.391-5.165 48.04 48.04 0 0 0-8.298.307.75.75 0 0 1-.186-1.489 49.159 49.159 0 0 1 5.343-.371V3A.75.75 0 0 1 9 2.25ZM15.75 9a.75.75 0 0 1 .68.433l5.25 11.25a.75.75 0 1 1-1.36.634l-1.198-2.567h-6.744l-1.198 2.567a.75.75 0 0 1-1.36-.634l5.25-11.25A.75.75 0 0 1 15.75 9Zm-2.672 8.25h5.344l-2.672-5.726-2.672 5.726Z" clip-rule="evenodd" />
    </svg>
    </button>
    <Show

    when=move || { opened() == true }
    >
    <div
    class=" absolute right-0 z-10 mt-2 w-36 origin-top-right rounded-md bg-sky-100 bg-opacity-80 backdrop-filter backdrop-blur-xl  shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
    <div class="py-1 font-semibold px-2" role="none">
      <PickTranslation
      set_opened={set_opened}
      />
    </div>
            </div>
            </Show>
    </div></div>
            </div>
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
