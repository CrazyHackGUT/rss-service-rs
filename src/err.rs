use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("rss error")]
    RssError(#[from] rss::Error)
}