#[allow(unused_imports)]
use super::*;
///An array of four doubles, corresponding to x, y, z and radius of the minimum bounding sphere of a node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mbs {
    ///The center point of the minimum bounding sphere. An array of four doubles, corresponding to x, y, z and radius of the minimum bounding sphere of a node. For a global scene, i.e. XY coordinate system in WGS1984, the values of the array correspond to longitude in decimal degrees, latitude of in decimal degrees, elevation in meters and radius in meters. For all other CRS, the values of x,y,z and r are in the same unit.
    pub mbs: [f64; 4],
}
