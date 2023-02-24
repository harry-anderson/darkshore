use dominator::routing;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use uuid::Uuid;
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    View(Uuid),
    Login,
    Error,
}

const HOME_ROUTE: &str = "#/";
const LOGIN_ROUTE: &str = "#/login";
const ERROR_ROUTE: &str = "#/error";
const VIEW_BOARD_ROUTE: &str = "#/view/b#";

fn parse_tag<'a>(i: &'a str) -> nom::IResult<&'a str, &'a str> {
    //Matched first to last
    alt((
        tag(VIEW_BOARD_ROUTE),
        tag(LOGIN_ROUTE),
        tag(ERROR_ROUTE),
        tag(HOME_ROUTE),
    ))(i)
}

impl Route {
    // This could use more advanced URL parsing, but it isn't needed
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        let hash = url.hash();

        match parse_tag(&hash) {
            IResult::Ok((ptr, tag)) => match tag {
                // Add raw string to enum mapping for routes
                HOME_ROUTE => Route::Home,
                LOGIN_ROUTE => Route::Login,
                VIEW_BOARD_ROUTE => match Uuid::parse_str(ptr) {
                    Ok(n) => Route::View(n),
                    Err(err) => Route::Error,
                },
                ERROR_ROUTE => Route::Error,
                _ => Route::Error,
            },
            Err(err) => {
                tracing::error!("expand_err={}", err);
                Route::Home
            }
        }
    }

    // pub fn as_url(&self) -> &'static str {
    //     match self {
    //         Route::Error => "#/error",
    //         Route::Login => "#/login",
    //         Route::Verify => "#/verify",
    //         Route::Home => "#/",
    //     }
    // }
}

impl Default for Route {
    fn default() -> Self {
        // Create the Route based on the current URL
        Self::from_url(&routing::url().lock_ref())
    }
}
