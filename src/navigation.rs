//! Retainable navigation over an I3S [`SceneLayerReader`].

use std::sync::Arc;

use ::hiera::{Cursor, Navigator};

use crate::{ContentRef, Error, NodeInfo, NodeRef, SceneLayerReader};

/// Immutable I3S facts retained by a [`SceneLayerCursor`].
///
/// The state preserves the loader's source-native [`NodeInfo`] rather than
/// projecting it into a generic spatial model.
#[derive(Clone)]
pub struct SceneLayerCursorState {
    info: NodeInfo,
}

/// A retainable, path-aware position in a loaded I3S scene-layer hierarchy.
#[derive(Clone)]
pub struct SceneLayerCursor {
    inner: Cursor<SceneLayerReader, SceneLayerCursorState>,
}

/// The shallow expansion of a [`SceneLayerCursor`].
pub struct SceneLayerExpansion {
    /// Immediate child positions.
    pub children: Vec<SceneLayerCursor>,
    /// Content references belonging to the expanded node.
    pub contents: Vec<ContentRef>,
}

/// Creates [`SceneLayerCursor`] values over a [`SceneLayerReader`].
///
/// The navigator owns no traversal policy. Callers retain cursors and choose
/// breadth-first, depth-first, priority, or application-specific traversal.
#[derive(Clone)]
pub struct SceneLayerNavigator {
    inner: Navigator<SceneLayerReader, SceneLayerCursorState>,
}

impl SceneLayerNavigator {
    /// Creates a navigator that owns `reader`.
    pub fn new(reader: SceneLayerReader) -> Self {
        Self::from_shared(Arc::new(reader))
    }

    /// Creates a navigator over a shared reader.
    pub fn from_shared(reader: Arc<SceneLayerReader>) -> Self {
        let inner = Navigator::from_shared(
            reader,
            |_loader, _item, info| {
                let state = SceneLayerCursorState { info: info.clone() };
                Box::pin(async move { Ok(state) })
            },
            |_loader, _parent, _item, info| {
                let state = SceneLayerCursorState { info: info.clone() };
                Box::pin(async move { Ok(state) })
            },
        );
        Self { inner }
    }

    /// Returns the reader serving this navigator.
    pub fn reader(&self) -> &SceneLayerReader {
        self.inner.loader()
    }

    /// Resolves and returns the root scene-layer cursor.
    pub async fn root(&self) -> Result<SceneLayerCursor, Error> {
        Ok(SceneLayerCursor {
            inner: self.inner.root().await?,
        })
    }
}

impl SceneLayerCursor {
    /// Returns the retained source-native node handle.
    pub fn node(&self) -> &NodeRef {
        self.inner.item()
    }

    /// Returns a stable source identity for this node.
    ///
    /// Use this to de-duplicate graph frontiers or associate application
    /// analysis state with a retained cursor.
    pub fn id(&self) -> String {
        self.inner.item_id()
    }

    /// Returns immutable source-native facts evaluated for this node.
    pub fn info(&self) -> &NodeInfo {
        &self.inner.state().info
    }

    /// Returns the cursor by which this position was reached, if any.
    pub fn parent(&self) -> Option<Self> {
        self.inner.parent().map(|inner| Self { inner })
    }

    /// Performs one shallow expansion of this cursor.
    pub async fn expand(&self) -> Result<SceneLayerExpansion, Error> {
        let expansion = self.inner.expand().await?;
        Ok(SceneLayerExpansion {
            children: expansion
                .children
                .into_iter()
                .map(|inner| Self { inner })
                .collect(),
            contents: expansion.contents,
        })
    }

    /// Returns immediate child cursors in source-defined order.
    pub async fn children(&self) -> Result<Vec<Self>, Error> {
        Ok(self.expand().await?.children)
    }

    /// Returns every child of this cursor's path parent except this cursor.
    pub async fn siblings(&self) -> Result<Vec<Self>, Error> {
        Ok(self
            .inner
            .siblings()
            .await?
            .into_iter()
            .map(|inner| Self { inner })
            .collect())
    }
}
