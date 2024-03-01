#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::dev::Service;
    use actix_web::http::Uri;
    use actix_web::HttpResponse;
    use actix_web::Responder;
    use actix_web::*;
    use kylerchinmusic::app::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use qstring::QString;

    use core::future::Future;
    use futures::future::ok;
    use futures::future::Either;
    use futures::future::Ready;
    use futures::FutureExt;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .service(ads)
            .service(ntpns)
            .service(octavehicleproxy)
            .wrap_fn(|service_request, service| {
                let uri_original = service_request.uri().clone();

                let mut uri_new = Uri::builder().scheme("https").authority("kylerchin.com");

                if let Some(p_and_q) = uri_original.path_and_query() {
                    uri_new = uri_new.path_and_query(p_and_q.as_str());
                }

                let uri = format!("{}", uri_new.build().unwrap());

                service.call(service_request).map(move |result| {
                    let languages = vec!["en", "ko", "zh", "zh-TW"];

                    let language_final_header = languages
                        .iter()
                        .map(|lang_code| {
                            format!(
                                "<{}>; rel=\"alternate\"; hreflang=\"{}\"",
                                uri,
                                lang_code
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(",");

                    let mut service_response = result.unwrap();

                    let header_value = http::header::HeaderValue::from_str(&language_final_header);

                    match header_value {
                        Ok(header_value) => {
                            service_response.headers_mut().insert(
                                http::header::HeaderName::from_static("link"),
                                header_value,
                            );
                        }
                        Err(err) => {
                            println!("{:#?}", err);
                        }
                    }

                    Ok(service_response)
                })
            })
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("ads.txt")]
async fn ads(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/ads.txt"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("octavehicleproxy")]
async fn octavehicleproxy(
   req: actix_web:: HttpRequest,
   leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> impl actix_web::Responder {

    let raw_data =
        reqwest::get("https://transitime-api.goswift.ly/api/v1/key/81YENWXv/agency/octa/command/gtfs-rt/vehiclePositions").await;

    match raw_data {
        Ok(raw_data) => {
            let text = raw_data.text().await.unwrap();

            actix_web::HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body(text)
        },
        Err(_) => {
            actix_web::HttpResponse::InternalServerError()
            .insert_header(("Content-Type", "text/plain"))
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body("Could not fetch Amtrak data")
        }
    }
}

#[cfg(feature = "ssr")]
#[actix_web::get("ntpns")]
async fn ntpns(
    req: actix_web::HttpRequest,
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> impl actix_web::Responder {
    let qs = qstring::QString::from(req.query_string());
    match qs.get("c") {
        Some(c) => match c.parse::<u128>() {
            Ok(c) => {
                let server_timestamp_ns = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();

                let server_client_req_diff_time = server_timestamp_ns - c;

                actix_web::HttpResponse::Ok()
                    .insert_header(("Content-Type", "text/plain"))
                    .insert_header(("Access-Control-Allow-Origin", "*"))
                    .body(format!(
                        "{}|{}",
                        server_timestamp_ns, server_client_req_diff_time
                    ))
            }
            Err(err) => actix_web::HttpResponse::BadRequest()
                .insert_header(("Content-Type", "text/plain"))
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .body("c param not a number"),
        },
        None => actix_web::HttpResponse::BadRequest()
            .insert_header(("Content-Type", "text/plain"))
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body("c param missing"),
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use kylerchinmusic::app::*;
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
