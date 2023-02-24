use std::sync::Arc;

use dominator::{class, html, link, Dom};
use once_cell::sync::Lazy;

use crate::spa::Spa;

pub struct LoginPage {
    pub login_google_uri: String,
    pub login_gitub_uri: String
}

static GOOGLE_BASE_URI: &str = "https://accounts.google.com/o/oauth2/v2/auth";
static GOOGLE_CLIENT_ID: &str = "";
static GOOGLE_REDIR_URI: &str = "http://localhost:10001/verify.html?source=google";
static GOOGLE_SCOPES: &str =   "https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email";

static GITHUB_BASE_URI: &str = "https://github.com/login/oauth/authorize";
static GITHUB_CLIENT_ID: &str = "";

impl LoginPage {
    pub fn new() -> Arc<Self> {
        // TODO fetch client_ids dynamically
        Arc::new(Self {
            login_google_uri: format!( "{}?response_type=code&client_id={}&scope={}&redirect_uri={}", GOOGLE_BASE_URI, GOOGLE_CLIENT_ID, GOOGLE_SCOPES, GOOGLE_REDIR_URI),
            login_gitub_uri: format!( "{}?client_id={}", GITHUB_BASE_URI,  GITHUB_CLIENT_ID)
        })
    }

    pub fn render_header(_spa_state: Arc<Spa>) -> Dom {
        static HEADER_CLASS: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
            }
        });

        html!("header", {
        .class(&*HEADER_CLASS)
        .children(&mut [
            html!("h1", {
                .text("Login")
            }),
        ])})
    }

    pub fn render_footer(_spa_state: Arc<Spa>) -> Dom {
        static FOOTER_CLASS: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
            }
        });

        html!("footer", {
        .class(&*FOOTER_CLASS)
        .children(&mut [
        ])})
    }

    pub fn render_main(spa_state: Arc<Spa>) -> Dom {
        static ROOT_CLASS: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
            }
        });

        html!("main", {
            .class([&*ROOT_CLASS])
            .children(&mut [
                html!("a", {
                    .prop("href", &spa_state.login_page.login_gitub_uri)
                    .text("Login with github")
                }),
                html!("a", {
                    .prop("href", &spa_state.login_page.login_google_uri)
                    .text("Login with google")
                }),
            ] )
        })
    }

    pub fn render(spa_state: Arc<Spa>) -> Option<Dom> {
        static ROOT_CLASS: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
            }
        });

        Some(html!("div", {
            .class([&*ROOT_CLASS,"container-fluid"])
            .children(&mut [
                Self::render_header(spa_state.clone()),
                Self::render_main(spa_state.clone()),
                Self::render_footer(spa_state),
            ] )
        }))
    }
}
