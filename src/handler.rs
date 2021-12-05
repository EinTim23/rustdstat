use crate::{Result};
use warp::{http::StatusCode, Reply};

pub async fn reset() {
    unsafe {
        super::requests = 0;
    }
}
pub async fn index_handler() -> Result<impl Reply> {
    Ok(warp::reply::html(super::HTMLF.clone()).into_response())
}
pub async fn dstat_handler() -> Result<impl Reply> {
    unsafe {
        super::requests += 1;
    }
    Ok(StatusCode::OK)
}
pub async fn getc_handler() -> Result<impl Reply> {
    unsafe{
        Ok(warp::reply::html(super::requests.to_string()).into_response())
    }
}