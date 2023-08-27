use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/another-page" view=AnotherPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let something = create_server_action::<SomeThing>();
    create_effect(move |_| {
        match something.value().get() {
            Some(Ok(success)) => {
                log::info!("Success: {}", success);
            }
            Some(Err(error)) => {
                log::error!("Error: {}", error);
            }
            _ => {}
        };
    });

    log::info!("Home Page");

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <ActionForm action=something>
            <input type="hidden" name="should_work" value="true" />
            <button type="submit">
                "Redirect"
            </button>
        </ActionForm>
    }
}

#[component]
fn AnotherPage() -> impl IntoView {
    log::info!("Another Page");

    view! {
        <h1>"Another Page"</h1>
    }
}

#[server(SomeThing, "/api")]
pub async fn something(should_work: bool) -> Result<bool, ServerFnError> {
    leptos_actix::redirect("/another-page");

    Ok(true)
}
