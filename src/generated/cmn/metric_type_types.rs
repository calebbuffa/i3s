#[allow(unused_imports)]
use super::*;
///Possible values are: `maxScreenThreshold`: A per-node value for the maximum pixel size as measured in screen pixels. This value indicates the upper limit for the screen size of the diameter of the node's minimum bounding sphere (MBS). In other words, the content referenced by this node will qualify to be rendered only when the screen size is below the maximum screen threshold value. Used with the mesh pyramid profile. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LoD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`screenSpaceRelative`: The scale of the node's minimum bounding volume. Used by the point profile. `distanceRangeFromDefaultCamera`: The distance from the surface of the node's minimum bounding volume to the camera. Used by the point profile. `effectiveDensity`: Estimation of the point density covered by the node. Used by the point cloud profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "maxScreenThreshold")]
    MaxScreenThreshold,
    #[serde(rename = "maxScreenThresholdSQ")]
    MaxScreenThresholdSq,
    #[serde(rename = "screenSpaceRelative")]
    ScreenSpaceRelative,
    #[serde(rename = "distanceRangeFromDefaultCamera")]
    DistanceRangeFromDefaultCamera,
    #[serde(rename = "effectiveDensity")]
    EffectiveDensity,
}
impl Default for MetricType {
    fn default() -> Self {
        Self::MaxScreenThreshold
    }
}
