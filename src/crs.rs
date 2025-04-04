//! Coordinate Reference Systems.

use serde::{Deserialize, Serialize};

/// Global EPSG codes
pub(crate) const GLOBAL_EPSGS: &[i32] = &[4326, 4490];

/// Coordinate Reference System Mode
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Mode {
    Local = 0,
    Global = 1,
}

impl Mode {
    /// Determine the mode from an EPSG code.
    pub fn from_epsg(epsg: i32) -> Self {
        if GLOBAL_EPSGS.contains(&epsg) {
            Mode::Global
        } else {
            Mode::Local
        }
    }
}

/// Elevation Info
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElevationInfo {
    pub mode: String,
    pub offset: Option<f64>,
    pub unit: Option<String>,
}

/// Height Model Info
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeightModelInfo {
    #[serde(rename = "heightModel")]
    pub height_model: String,
    #[serde(rename = "vertCRS", default)]
    pub vertical_crs: Option<String>,
    #[serde(rename = "heightUnit")]
    pub height_unit: String,
}

/// Extent
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Extent {
    pub xmin: f64,
    pub ymin: f64,
    pub zmin: f64,
    pub xmax: f64,
    pub ymax: f64,
    pub zmax: f64,
    #[serde(rename = "spatialReference", default)]
    pub spatial_reference: Option<SpatialReference>,
}

/// Spatial Reference
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SpatialReference {
    pub wkid: i32,
    #[serde(rename = "latestWkid", default)]
    pub latest_wkid: Option<i32>,
    #[serde(rename = "vcsWkid", default)]
    pub vcs_wkid: Option<i32>,
    #[serde(rename = "latestVcsWkid", default)]
    pub latest_vcs_wkid: Option<i32>,
}

impl SpatialReference {
    /// Check if the spatial reference has a vertical coordinate system.
    pub fn has_z(&self) -> bool {
        self.vcs_wkid.is_some() || self.latest_vcs_wkid.is_some()
    }

    /// Get the vertical coordinate system WKID.
    pub fn z(&self) -> Option<i32> {
        self.vcs_wkid.or(self.latest_vcs_wkid)
    }

    /// Get the horizontal coordinate system WKID.
    pub fn xy(&self) -> i32 {
        self.wkid
    }

    /// Get the mode (Local or Global) based on the WKID.
    pub fn mode(&self) -> Mode {
        Mode::from_epsg(self.wkid)
    }
}
