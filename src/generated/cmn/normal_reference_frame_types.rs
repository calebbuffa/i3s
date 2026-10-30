#[allow(unused_imports)]
use super::*;
///Describes the coordinate reference frame used for storing normals. Although not required, it is recommended to re-compute the normal component of the binary geometry buffer if this property is not present. Possible values are: `east-north-up`: Normals are stored in a node local reference frame defined by the easting, northing and up directions at the MBS center. It is only valid for geographic (WGS84) vertexCRS.`earth-centered`: Normals are stored in a global earth-centered, earth-fixed ([ECEF](https://en.wikipedia.org/wiki/ECEF)) reference frame. It is only valid for geographic vertexCRS. Earth centered can be directly used for global view rendering, thus is an optimal option for meshes in non-projected geographic coordinate system. `vertex-reference-frame`: Normals are stored in the same reference frame as vertices. It is only valid for projected vertexCRS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NormalReferenceFrame {
    #[serde(rename = "east-north-up")]
    EastNorthUp,
    #[serde(rename = "earth-centered")]
    EarthCentered,
    #[serde(rename = "vertex-reference-frame")]
    VertexReferenceFrame,
}
impl Default for NormalReferenceFrame {
    fn default() -> Self {
        Self::EastNorthUp
    }
}
