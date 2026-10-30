#[allow(unused_imports)]
use super::*;
///The cachedDrawingInfo object indicates if the *drawingInfo* object is captured as part of the binary scene layer representation. This object is used for the 3D Object and Integrated Mesh scene layer if no [drawingInfo](drawingInfo.cmn.md) is defined.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedDrawingInfo {
    ///If true, the drawingInfo is captured as part of the binary scene layer representation.
    pub color: bool,
}
