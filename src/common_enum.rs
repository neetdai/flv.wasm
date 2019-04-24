use serde::{Serialize, Serializer};

pub(crate) enum MediaDataType {
    FLV,
    MP4,
}

impl Serialize for MediaDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match &self {
            MediaDataType::FLV => {
                serializer.serialize_str("flv")
            },
            MediaDataType::MP4 => {
                serializer.serialize_str("mp4")
            }
        }
    }
}
