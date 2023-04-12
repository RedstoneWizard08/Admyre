use axum::{http::Request, middleware::Next, response::Response};

use axum::http::Method;
use chrono::Utc;

use crate::colors::{BackgroundColors, Colors, ForegroundColors};

pub async fn logging_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let time_start = Utc::now().time();

    let method = &req.method().clone();
    let uri = &req.uri().clone();

    let res = next.run(req).await;

    let now = Utc::now().time();

    let elapsed = now - time_start;

    let color: &str = match method.clone() {
        Method::GET => BackgroundColors::Green,
        Method::PUT => BackgroundColors::Light_Blue,
        Method::POST => BackgroundColors::Magenta,
        Method::PATCH => BackgroundColors::Light_Yellow,
        Method::DELETE => BackgroundColors::Light_Red,

        _ => BackgroundColors::Yellow,
    };

    // info!(
    //     "{}{}{}{} {} ({} MS)",
    //     ForegroundColors::Magenta,
    //     Colors::Bold,
    //     uri.path(),
    //     Colors::Reset,
    //     res.status(),
    //     elapsed.num_milliseconds()
    // );

    info!(
        "{}{}{} {} {} {}{}{}{} {} ({} MS){}",
        color,
        Colors::Bold,
        ForegroundColors::Black,
        method.as_str(),
        Colors::Reset,
        ForegroundColors::Magenta,
        Colors::Bold,
        uri.path(),
        Colors::Reset,
        res.status(),
        elapsed.num_milliseconds(),
        Colors::Reset,
    );

    return res;
}
