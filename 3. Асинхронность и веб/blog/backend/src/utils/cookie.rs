use axum::http::{HeaderMap, HeaderValue, Response, header};
use cookie::Cookie;

/// Получение cookie
pub fn extract_cookie(headers: &HeaderMap<HeaderValue>, key: &str) -> Option<String> {
    let header = headers.get(axum::http::header::COOKIE)?;
    let header = header.to_str().ok()?;

    header
        .split(';')
        .filter_map(|c| Cookie::parse(c.trim()).ok())
        .find(|c| c.name() == key)
        .map(|c| c.value().to_string())
}

/// Установка cookie
pub fn set_cookie<B>(res: &mut Response<B>, cookie: Cookie) {
    res.headers_mut()
        .append(header::SET_COOKIE, cookie.to_string().parse().unwrap());
}
