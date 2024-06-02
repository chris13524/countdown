use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone};
use leptos::*;
use leptos_router::*;
use web_sys::js_sys::encode_uri_component;

#[derive(Params, PartialEq)]
struct QueryParams {
    name: Option<String>,
    to: Option<DateTime<FixedOffset>>,
}

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <App/>
            </Router>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let query: Memo<Result<QueryParams, ParamsError>> = use_query::<QueryParams>();

    move || {
        query.with(|params| {
            params
                .as_ref()
                .map(|params| {
                    params
                        .to
                        .map(|to| {
                            view! {
                                <Countdown to name=params.name.clone()/>
                            }
                            .into_view()
                        })
                        .unwrap_or_else(|| {
                            view! {
                                <Create />
                            }
                            .into_view()
                        })
                })
                .unwrap_or_else(|e| {
                    view! {
                        <p>{format!("{e:?}")}</p>
                    }
                    .into_view()
                })
        })
    }
}

#[component]
fn Create() -> impl IntoView {
    let navigate = use_navigate();
    let (datetime, set_datetime) = create_signal("".to_owned());
    let (name, set_name) = create_signal("".to_owned());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let datetime = NaiveDateTime::parse_from_str(&datetime.get(), "%FT%R").unwrap();
        let datetime = Local.from_local_datetime(&datetime).unwrap();
        let datetime = datetime.to_rfc3339();

        let mut url = format!("/?to={}", encode_uri_component(&datetime));
        let name = name.get();
        if !name.is_empty() {
            url.push_str(&format!("&name={}", encode_uri_component(&name)));
        }
        navigate(&url, Default::default());
    };

    view! {
        <h1>"Create countdown"</h1>
        <form on:submit=on_submit>
            <div>
                <label for="name">"Name: "</label>
                <input type="text" name="name" prop:value=name on:input=move |ev| set_name(event_target_value(&ev)) />
            </div>
            <div>
                <label for="to">"To: "</label>
                <input type="datetime-local" required prop:value=datetime on:input=move |ev| set_datetime(event_target_value(&ev)) />
            </div>
            <input type="submit" />
        </form>
    }
}

#[component]
fn Countdown(to: DateTime<FixedOffset>, name: Option<String>) -> impl IntoView {
    let name = name
        .map(|name| format!("Countdown to {name}"))
        .unwrap_or_else(|| "Countdown".to_owned());

    view! {
        <div>
            <h1>{name}</h1>
            <p>"Time: "{to.to_string()}</p>
            <A href="/">{"Create another"}</A>
        </div>
    }
}
