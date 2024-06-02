use chrono::*;
use ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use web_sys::js_sys::encode_uri_component;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <App/>
                <A href="https://github.com/chris13524/countdown">{"GitHub"}</A>
            </Router>
        }
    })
}

#[derive(Params, PartialEq)]
struct QueryParams {
    name: Option<String>,
    to: Option<DateTime<FixedOffset>>,
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

const DATETIME_LOCAL_FMT: &str = "%FT%R";

#[component]
fn Create() -> impl IntoView {
    let navigate = use_navigate();

    let (name, set_name) = create_signal("".to_owned());

    let default_datetime = Local::now() + TimeDelta::minutes(1);
    let (datetime, set_datetime) =
        create_signal(default_datetime.format(DATETIME_LOCAL_FMT).to_string());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = name.get();

        let datetime = {
            let datetime =
                NaiveDateTime::parse_from_str(&datetime.get(), DATETIME_LOCAL_FMT).unwrap();
            let datetime = Local.from_local_datetime(&datetime).unwrap();
            datetime.to_rfc3339()
        };

        let url = {
            let mut url = format!("/?to={}", encode_uri_component(&datetime));
            if !name.is_empty() {
                url.push_str(&format!("&name={}", encode_uri_component(&name)));
            }
            url
        };
        navigate(&url, Default::default());
    };

    view! {
        <h1>"Create countdown"</h1>
        <form on:submit=on_submit>
            <div>
                <label for="name">"Name: "</label>
                <input type="text" name="name"
                    prop:value=name
                    on:input=move |ev| set_name(event_target_value(&ev)) />
            </div>
            <div>
                <label for="to">"To: "</label>
                <input type="datetime-local" required
                    prop:value=datetime
                    on:input=move |ev| set_datetime(event_target_value(&ev)) />
            </div>
            <input type="submit" />
        </form>
    }
}

fn update_countdown(to: DateTime<FixedOffset>, set_time_remaining: WriteSignal<TimeDelta>) {
    let now = Local::now().with_timezone(to.offset());
    let delta = to - now.trunc_subsecs(0);
    set_time_remaining(delta);

    let next_update = now.with_nanosecond(0).unwrap() + TimeDelta::seconds(1);
    set_timeout(
        move || update_countdown(to, set_time_remaining),
        (next_update - now).to_std().unwrap(),
    );
}

#[component]
fn Countdown(to: DateTime<FixedOffset>, name: Option<String>) -> impl IntoView {
    let name = name
        .map(|name| format!("Countdown to {name}"))
        .unwrap_or_else(|| "Countdown".to_owned());

    let (time_remaining, set_time_remaining) = create_signal(TimeDelta::max_value());
    update_countdown(to, set_time_remaining);

    view! {
        <div>
            <h1>{name}</h1>
            <p>"Time: "{to.to_string()}</p>
            <p>"Seconds: "{move || time_remaining.get().num_seconds()}</p>
            <A href="/">{"Create another"}</A>
        </div>
    }
}
