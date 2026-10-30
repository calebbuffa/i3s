#[allow(unused_imports)]
use super::*;
///Defines the meaning of `nodes[].lodThreshold` for this layer. Possible values are: `maxScreenThreshold`: A per-node value for the maximum area of the projected bounding volume on screen in pixel. `maxScreenThresholdSQ`: A per-node value for the maximum area of the projected bounding volume on screen in pixel squared. 3D Viewers may implement **look-angle dependent** node switching by comparing this metric with the area of the 2D outline of the oriented-bounding box (OBB) on screen. ( see [_"Fast Projected Area Computation for Three-Dimensional Bounding Boxes", Dieter Schmalstieg and Robert F. Tobler_](https://pdfs.semanticscholar.org/1f59/8266e387cf367702d16acf5a4e02cc72cb99.pdf) for an efficient algorithm). If a **look-angle independent** LOD switching is desired, viewers may use the area of minimum bounding-sphere (MBS) of the node if available or the MBS of the OBB otherwise. Note: `maxScreenThresholdSQ` may be related to `maxScreenThreshold` as follow: `maxScreenThresholdSQ = PI * 0.25 * maxScreenThreshold * maxScreenThreshold`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodePageDefinitionLodSelectionMetricType {
    #[serde(rename = "maxScreenThreshold")]
    MaxScreenThreshold,
    #[serde(rename = "maxScreenThresholdSQ")]
    MaxScreenThresholdSq,
}
impl Default for NodePageDefinitionLodSelectionMetricType {
    fn default() -> Self {
        Self::MaxScreenThreshold
    }
}
