//! Oriented Bounding Box.

use crate::crs::Mode;
use nalgebra::{Matrix3, Quaternion, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

/// Compute an oriented bounding box from center, half size, and quaternion.
///
/// # Parameters
/// - `center`: The (x, y, z) center of the OBB.
/// - `half_size`: The (x, y, z) half size of the OBB.
/// - `quaternion`: The (x, y, z, w) quaternion representing the rotation.
///
/// # Returns
/// A vector of 8 corners representing the oriented bounding box.
pub fn compute_obb(
    center: Vector3<f64>,
    half_size: Vector3<f64>,
    quaternion: Quaternion<f64>,
) -> Vec<Vector3<f64>> {
    // Convert quaternion to a rotation matrix
    let rotation: Matrix3<f64> = UnitQuaternion::from_quaternion(quaternion).to_rotation_matrix().into_inner();

    // Define the corners of the bounding box
    let corners = vec![
        Vector3::new(-half_size.x, -half_size.y, -half_size.z), // Corner 0: min corner
        Vector3::new(half_size.x, -half_size.y, -half_size.z),  // Corner 1
        Vector3::new(half_size.x, half_size.y, -half_size.z),   // Corner 2
        Vector3::new(-half_size.x, half_size.y, -half_size.z),  // Corner 3
        Vector3::new(-half_size.x, -half_size.y, half_size.z),  // Corner 4
        Vector3::new(half_size.x, -half_size.y, half_size.z),   // Corner 5
        Vector3::new(half_size.x, half_size.y, half_size.z),    // Corner 6
        Vector3::new(-half_size.x, half_size.y, half_size.z),   // Corner 7
    ];

    // Rotate and translate corners
    corners
        .into_iter()
        .map(|corner| rotation * corner + center)
        .collect()
}

/// Oriented Bounding Box
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrientedBoundingBox {
    pub center: Vec<f64>,
    #[serde(rename = "halfSize")]
    pub half_size: Vec<f64>,
    pub quaternion: Vec<f64>,
}

impl OrientedBoundingBox {
    /// Compute the vertices of the Oriented Bounding Box.
    ///
    /// # Parameters
    /// - `mode`: The mode of the scene (Local or Global).
    ///
    /// # Returns
    /// A vector of 8 corners representing the oriented bounding box.
    pub fn vertices(&self, mode: Mode) -> Result<Vec<Vector3<f64>>, String> {
        if mode == Mode::Global {
            return Err("Global mode not yet supported".to_string());
        }

        let center = Vector3::new(self.center[0], self.center[1], self.center[2]);
        let half_size = Vector3::new(self.half_size[0], self.half_size[1], self.half_size[2]);
        let quaternion = Quaternion::new(
            self.quaternion[3], // w
            self.quaternion[0], // x
            self.quaternion[1], // y
            self.quaternion[2], // z
        );

        Ok(compute_obb(center, half_size, quaternion))
    }
}
