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
            match f(cursor.clone()).await {
                Ok(response) => {
                    cursor = response.next_token.clone();
                    for item in response.payload {
                        yield Ok(item);
                    }

                    // No more pages — exit
                    if cursor.is_none() {
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
        let response = f(cursor.clone()).await?;
        cursor = response.next_token.clone();
        all_items.extend(response.payload);

        if cursor.is_none() {
            break;
        }
    }

    Ok(all_items)
}

/// Box a future for type-erased pagination.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
