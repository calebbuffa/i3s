//! Caller-owned byte transport for I3S resources.
//!
//! I3S core resolves format-native names and parses typed control documents;
//! applications own all filesystem, archive, HTTP, cache, authentication,
//! retry, decompression, and execution policy.

use std::{future::Future, sync::Arc};

use hiera::{BoxFuture, Bytes, Fetch, FetchRequest};

use crate::{Error, ResourcePath};

/// Application-provided access to logical I3S resource bytes.
///
/// Implementors decide how resource bytes are obtained and prepared. This
/// includes eSLPK directories, SLPK ZIP archives, HTTP, object stores, cache
/// behavior, package gzip decoding, and which executor performs blocking or
/// CPU work. The core crate never opens files, scans directories, reads ZIP
/// archives, or creates an executor.
pub trait SceneLayerStore: Send + Sync {
    /// Reads one resolved logical resource name.
    fn read(&self, name: String) -> BoxFuture<Bytes, Error>;

    /// Lists logical package-relative entries when the application transport
    /// can enumerate them.
    ///
    /// Return `None` for transports without trustworthy enumeration, such as
    /// ordinary REST or static HTTP endpoints.
    fn entries(&self) -> BoxFuture<Option<Vec<ResourcePath>>, Error> {
        Box::pin(async { Ok(None) })
    }
}

/// Adapts Hiera's caller-provided transport callback to an I3S store.
///
/// This adapter performs no I/O itself; it only forwards the resolved request
/// to the application-owned [`Fetch`] implementation.
#[derive(Clone)]
pub struct FetchStore {
    fetch: Fetch,
}

impl FetchStore {
    /// Creates a store backed by caller-provided asynchronous transport.
    pub fn new<F, Fut>(fetch: F) -> Self
    where
        F: Fn(FetchRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<hiera::FetchResponse, hiera::FetchError>> + Send + 'static,
    {
        Self {
            fetch: Arc::new(move |request| Box::pin(fetch(request))),
        }
    }
}

impl SceneLayerStore for FetchStore {
    fn read(&self, name: String) -> BoxFuture<Bytes, Error> {
        let fetch = Arc::clone(&self.fetch);
        Box::pin(async move {
            let response = fetch(FetchRequest {
                uri: name.clone(),
                range: None,
            })
            .await
            .map_err(|source| Error::fetch(name, source))?;
            Ok(response.bytes)
        })
    }
}
