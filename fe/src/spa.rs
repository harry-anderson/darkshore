use dominator::{class, clone, html, routing, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::sync::Arc;

use crate::{
    model::User,
    pages::{error::ErrorPage, home::HomePage, login::LoginPage, verify::VerifyPage},
    router::Route,
    util::AsyncLoader,
};

pub struct Spa {
    pub route: Mutable<Route>,
    pub user: Mutable<Option<User>>,
    pub input: Mutable<String>,
    pub loader: AsyncLoader,
    pub home_page: Arc<HomePage>,
    pub login_page: Arc<LoginPage>,
    pub error_page: Arc<ErrorPage>,
}

impl Spa {
    pub fn new(name: &str, user: Option<User>) -> Arc<Self> {
        Arc::new(Self {
            route: Mutable::new(Route::default()),
            user: Mutable::new(user),
            input: Mutable::new(name.to_string()),
            loader: AsyncLoader::new(),
            home_page: HomePage::new(),
            login_page: LoginPage::new(),
            error_page: ErrorPage::new(),
        })
    }

    pub fn render(spa: Arc<Self>) -> Dom {
        html!("section", {
             // Update the Route when the URL changes
            .future(routing::url()
                .signal_ref(|url| Route::from_url(url))
                .for_each(clone!(spa => move |route| {
                    spa.route.set_neq(route);
                    async {}
            })))

            .child_signal(spa.route.signal_ref(clone!(spa => move |route| {
                match route {
                    Route::Home => HomePage::render(spa.clone()),
                    Route::Login => LoginPage::render(spa.clone()),
                    Route::Verify => VerifyPage::render(spa.clone()),
                    Route::View(_) => todo!(),
                    Route::Error => ErrorPage::render(spa.clone()),
                }
            })))
        })
    }
}
