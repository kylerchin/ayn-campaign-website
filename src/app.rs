use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.

    view! {
           // <!-- Google tag (gtag.js) -->
    <Script async_="true" src="https://www.googletagmanager.com/gtag/js?id=G-N17QFSGLLK"/>
    <Script>
"window.dataLayer = window.dataLayer || [];
function gtag(){dataLayer.push(arguments);}
gtag('js', new Date());

gtag('config', 'G-N17QFSGLLK');"
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
                       // <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </Router>
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


      // <div class="container mx-full"> <p class="underline"><a href="https://computing.kylerchin.com">"Computing"</a></p>
      // <p>{t!(i18n, email)} ": kyler@yk3music.com"</p>
       <br/>

       <p>"© Yk3 Music 2024"</p></div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    leptos_meta::provide_meta_context();

    provide_i18n_context();

    let i18n = use_i18n();

    view! {

        <Title text={"Ayn Craciun for City Council"}/>
        // sets the document title
        
    }
}