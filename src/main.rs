use chrono::*;
use ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::js_sys::encode_uri_component;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <main>
            <Router>
                <Main />
                <A href="https://github.com/chris13524/countdown">{"GitHub"}</A>
            </Router>
        </main>
    }
}

#[derive(Params, PartialEq)]
struct QueryParams {
    name: Option<String>,
    to: Option<DateTime<FixedOffset>>,
}

#[component]
fn Main() -> impl IntoView {
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
                        <p>"Error: "{format!("{e:?}")}</p>
                    }
                    .into_view()
                })
        })
    }
}

const INPUT_DATETIME_LOCAL_FMT: &str = "%FT%R";

#[component]
fn Create() -> impl IntoView {
    let navigate = use_navigate();

    let (name, set_name) = create_signal("".to_owned());

    let default_datetime = Local::now() + TimeDelta::minutes(1);
    let (datetime, set_datetime) = create_signal(
        default_datetime
            .format(INPUT_DATETIME_LOCAL_FMT)
            .to_string(),
    );

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = name.get();

        let datetime = {
            let datetime =
                NaiveDateTime::parse_from_str(&datetime.get(), INPUT_DATETIME_LOCAL_FMT).unwrap();
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
        <Title text="Create countdown" />
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
        .map(|name| format!("{name} countdown"))
        .unwrap_or_else(|| "Countdown".to_owned());

    let (time_remaining, set_time_remaining) = create_signal(TimeDelta::max_value());
    update_countdown(to, set_time_remaining);

    let styler_class = stylers::style! { "Countdown",
        .remaining {
            display: "grid";
            grid-template-columns: "repeat(4, min-content auto) min-content";
            grid-template-rows: "auto auto";
            text-align: center;
            justify-content: center;
        }

        .big {
            font-size: 85px;
        }
    };

    view! { class = styler_class,
        <Title text=name.clone() />
        <div>
            <h1>{name}</h1>
            <p>"Time: "{to.to_string()}</p>
            <div class="remaining">
                <div>
                    <div class="big">{move || format!("{:02}", time_remaining.get().num_weeks())}</div>
                    <div>"Weeks"</div>
                </div>
                <div class="big">:</div>
                <div>
                    <div class="big">{move || format!("{:02}", time_remaining.get().num_days() % 7)}</div>
                    <div>"Days"</div>
                </div>
                <div class="big">:</div>
                <div>
                    <div class="big">{move || format!("{:02}", time_remaining.get().num_hours() % 24)}</div>
                    <div>"Hours"</div>
                </div>
                <div class="big">:</div>
                <div>
                    <div class="big">{move || format!("{:02}", time_remaining.get().num_minutes() % 60)}</div>
                    <div>"Minutes"</div>
                </div>
                <div class="big">:</div>
                <div>
                    <div class="big">{move || format!("{:02}", time_remaining.get().num_seconds() % 60)}</div>
                    <div>"Seconds"</div>
                </div>
            </div>
            <A href="/">{"Create another"}</A>
        </div>
    }
}
