#[allow(unused_imports)]
use super::*;
///The 3D spatial extent of the object it describes in the given spatial reference. The coordinates of the extent can span across the antimeridian (180th meridian). For example, scene layers in a geographic coordinate system covering New Zealand may have a larger xmin value than xmax value. The fullExtent is used by clients to zoom to a scene layer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullExtent {
    ///An object containing the WKID or WKT identifying the spatial reference of the layer's geometry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_reference: Option<SpatialReference>,
    ///The most west x coordinate.
    pub xmax: f64,
    ///The most east x coordinate.
    pub xmin: f64,
    ///The most north y coordinate.
    pub ymax: f64,
    ///The most south y coordinate.
    pub ymin: f64,
    ///The maximum height z coordinate.
    pub zmax: f64,
    ///The minimum height z coordinate.
    pub zmin: f64,
}
