#[allow(unused_imports)]
use super::*;
///Object to provide time stamp when the I3S service or the source of the service was created or updated.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpdateTimeStamp {
    ///Specifies the Unix epoch counting from 1 January 1970 in milliseconds. Time stamp is created when the I3S service was created or updated.
    pub last_update: f64,
}
