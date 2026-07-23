use std::future::Future;
use std::pin::Pin;

use async_stream::stream;
use futures_core::Stream;

use crate::apis::Error;
use crate::models::common::ApiResponse;

/// Paginate through cursor-based API responses.
///
/// Each page is requested with the `next_cursor` from the previous response.
/// The closure `f` receives an optional cursor (None for the first page) and
/// must return a future yielding `Result<ApiResponse<T>, Error<E>>`.
///
/// Returns a `Stream` of items that yields each item as soon as it's available.
pub fn paginate_cursor<F, Fut, T, E>(f: F) -> impl Stream<Item = Result<T, Error<E>>>
where
    F: Fn(Option<String>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<ApiResponse<Vec<T>>, Error<E>>> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    stream! {
        let mut cursor: Option<String> = None;

        loop {
            match f(cursor.take()).await {
                Ok(response) => {
                    let has_next = response.next_token.is_some();
                    cursor = response.next_token;
                    for item in response.payload {
                        yield Ok(item);
                    }

                    if !has_next {
                        break;
                    }
                }
                Err(e) => {
                    yield Err(e);
                    break;
                }
            }
        }
    }
}

/// Paginate through cursor-based API responses and collect all items.
pub async fn paginate_all_cursor<F, Fut, T, E>(f: F) -> Result<Vec<T>, Error<E>>
where
    F: Fn(Option<String>) -> Fut + Send,
    Fut: Future<Output = Result<ApiResponse<Vec<T>>, Error<E>>> + Send,
{
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let response = f(cursor.take()).await?;
        let has_next = response.next_token.is_some();
        cursor = response.next_token;
        all_items.extend(response.payload);

        if !has_next {
            break;
        }
    }

    Ok(all_items)
}

/// Box a future for type-erased pagination.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
