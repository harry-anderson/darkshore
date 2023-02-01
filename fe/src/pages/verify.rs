
use std::sync::Arc;

use dominator::{class, html, link, Dom};
use once_cell::sync::Lazy;

use crate::spa::Spa;


pub struct VerifyPage {}

impl VerifyPage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
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
                .text("Verify")
            }),
            link!("#/", { .text("Home")}),
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

    pub fn render_main(_spa_state: Arc<Spa>) -> Dom {
        static ROOT_CLASS: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "grid")
            }
        });

        html!("main", {
            .class([&*ROOT_CLASS])
            .children(&mut [

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
