use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all plugins, 2 chars, match `DeviceNamespace` in manifest.json
pub const DEVICE_NAMESPACE: &str = "d7";

pub const ROW_COUNT: usize = 4;
pub const COL_COUNT: usize = 3;
pub const KEY_COUNT: usize = ROW_COUNT * COL_COUNT;
pub const ENCODER_COUNT: usize = 0;

#[derive(Debug, Clone)]
pub enum Kind {
    DUO87,
}

pub const WOMIER_VID: u16 = 0x0600;
pub const DUO87_PID: u16 = 0x3001;

pub const DUO87_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, WOMIER_VID, DUO87_PID);

pub const QUERIES: [DeviceQuery; 1] = [DUO87_QUERY];

pub fn get_image_format_for_key(_kind: &Kind, _key: u8) -> ImageFormat {
    ImageFormat {
        mode: ImageMode::JPEG,
        size: (80, 80),
        rotation: ImageRotation::Rot180,
        mirror: ImageMirroring::None,
    }
}

impl Kind {
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match (vid, pid) {
            (WOMIER_VID, DUO87_PID) => Some(Kind::DUO87),
            _ => None,
        }
    }

    // Sets mirajazz's packet size to 1024 bytes, matching DUO87's real transfers.
    pub fn protocol_version(&self) -> usize {
        2
    }

    pub fn human_name(&self) -> String {
        "Womier DUO87".to_string()
    }

    pub fn id_suffix(&self) -> String {
        "DUO87".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}