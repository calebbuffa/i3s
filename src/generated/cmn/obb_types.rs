#[allow(unused_imports)]
use super::*;
///An Oriented Bounding Box (OBB) is a compact bounding volume representation, tightly fitting the geometries it represents. An OBBs' invariance to translation and rotation, makes it ideal as the optimal and default bounding volume representation in I3S. When constructing an OBB for I3S use, there are two considerations an implementer needs to be make based on the Coordinate Reference System (CRS) of the layer:
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Obb {
    ///The center point of the oriented bounding box. For a global scene, such as the XY coordinate system in WGS1984, the center is specified in latitude/longitude in decimal degrees, elevation (Z) in meters.
    pub center: [f64; 3],
    ///Half size of the oriented bounding box in units of the CRS. For a global scene, such as the XY coordinate system in WGS1984, the center is specified in latitude/longitude in decimal degrees, elevation (Z) in meters.
    pub half_size: [f64; 3],
    ///Orientation of the oriented bounding box as a 4-component quaternion. For a global scene, the quaternion is in an Earth-Centric-Earth-Fixed (ECEF) Cartesian space. ( Z+ : North, Y+ : East, X+: lon=lat=0.0). Note: A quaternion is a four-element vector that can be used to encode any rotation in a 3D coordinate system. The quaternion components are in the order x, y, z, w.
    pub quaternion: [f64; 4],
}
