pub struct Type {
    pub name: &'static str,
    pub list: Vec<u8>
}


pub struct Meta {
    pub id: u64,
    pub duration: u64,
    pub width: u64,
    pub height: u64
}


pub struct Constants {
    pub FTYP: Vec<u8>,
    pub STSD_PREFIX: Vec<u8>,
    pub STTS: Vec<u8>,
    pub STSC: Vec<u8>,
    pub STCO: Vec<u8>,
    pub STSZ: Vec<u8>,
    pub HDLR_VIDEO: Vec<u8>,
    pub HDLR_AUDIO: Vec<u8>,
    pub DREF: Vec<u8>,
    pub SMHD: Vec<u8>,
    pub VMHD: Vec<u8>
}


pub struct MP4 {
    pub types: Vec<Type>,
    pub constants: Constants
}


impl MP4 {
    pub fn new () -> Self {
        MP4 {
            types: vec![
                Type { name: "avc1", list: Vec::new() },
                Type { name: "avcC", list: Vec::new() },
                Type { name: "btrt", list: Vec::new() },
                Type { name: "dinf", list: Vec::new() },
                Type { name: "dref", list: Vec::new() },
                Type { name: "esds", list: Vec::new() },
                Type { name: "ftyp", list: Vec::new() },
                Type { name: "hdlr", list: Vec::new() },
                Type { name: "mdat", list: Vec::new() },
                Type { name: "mdhd", list: Vec::new() },
                Type { name: "mdia", list: Vec::new() },
                Type { name: "mfhd", list: Vec::new() },
                Type { name: "minf", list: Vec::new() },
                Type { name: "moof", list: Vec::new() },
                Type { name: "moov", list: Vec::new() },
                Type { name: "mp4a", list: Vec::new() },
                Type { name: "mvex", list: Vec::new() },
                Type { name: "mvhd", list: Vec::new() },
                Type { name: "sdtp", list: Vec::new() },
                Type { name: "stbl", list: Vec::new() },
                Type { name: "stco", list: Vec::new() },
                Type { name: "stsc", list: Vec::new() },
                Type { name: "stsd", list: Vec::new() },
                Type { name: "stsz", list: Vec::new() },
                Type { name: "stts", list: Vec::new() },
                Type { name: "tfdt", list: Vec::new() },
                Type { name: "tfhd", list: Vec::new() },
                Type { name: "traf", list: Vec::new() },
                Type { name: "trak", list: Vec::new() },
                Type { name: "trun", list: Vec::new() },
                Type { name: "trex", list: Vec::new() },
                Type { name: "tkhd", list: Vec::new() },
                Type { name: "vmhd", list: Vec::new() },
                Type { name: "smhd", list: Vec::new() },
                Type { name: ".mp3", list: Vec::new() }
            ],
            constants: Constants {
                FTYP: [
                    0x69, 0x73, 0x6F, 0x6D,  // major_brand: isom
                    0x0,  0x0,  0x0,  0x1,   // minor_version: 0x01
                    0x69, 0x73, 0x6F, 0x6D,  // isom
                    0x61, 0x76, 0x63, 0x31   // avc1
                ].to_vec(),
                STSD_PREFIX: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x01   // entry_count
                ].to_vec(),
                STTS: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00   // entry_count
                ].to_vec(),
                STCO: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00   // entry_count
                ].to_vec(),
                STSC: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00   // entry_count
                ].to_vec(),
                STSZ: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00,  // sample_size
                    0x00, 0x00, 0x00, 0x00   // sample_count
                ].to_vec(),
                HDLR_VIDEO: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00,  // pre_defined
                    0x76, 0x69, 0x64, 0x65,  // handler_type: 'vide'
                    0x00, 0x00, 0x00, 0x00,  // reserved: 3 * 4 bytes
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00,
                    0x56, 0x69, 0x64, 0x65,
                    0x6F, 0x48, 0x61, 0x6E,
                    0x64, 0x6C, 0x65, 0x72, 0x00  // name: VideoHandler
                ].to_vec(),
                HDLR_AUDIO: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00,  // pre_defined
                    0x73, 0x6F, 0x75, 0x6E,  // handler_type: 'soun'
                    0x00, 0x00, 0x00, 0x00,  // reserved: 3 * 4 bytes
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00,
                    0x53, 0x6F, 0x75, 0x6E,
                    0x64, 0x48, 0x61, 0x6E,
                    0x64, 0x6C, 0x65, 0x72, 0x00  // name: SoundHandler
                ].to_vec(),
                DREF: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x01,  // entry_count
                    0x00, 0x00, 0x00, 0x0C,  // entry_size
                    0x75, 0x72, 0x6C, 0x20,  // type 'url '
                    0x00, 0x00, 0x00, 0x01   // version(0) + flags
                ].to_vec(),
                SMHD: [
                    0x00, 0x00, 0x00, 0x00,  // version(0) + flags
                    0x00, 0x00, 0x00, 0x00   // balance(2) + reserved(2)
                ].to_vec(),
                VMHD: [
                    0x00, 0x00, 0x00, 0x01,  // version(0) + flags
                    0x00, 0x00,              // graphicsmode: 2 bytes
                    0x00, 0x00, 0x00, 0x00,  // opcolor: 3 * 2 bytes
                    0x00, 0x00
                ].to_vec()
            }
        }
    }

    pub fn r#box (&self, r#type: Vec<u8>, data: Vec<Vec<u8>>) -> Vec<u8> {
        let mut size = 0;
        let mut offset = 8;
        let mut result: Vec<u8> = Vec::new();

        for value in data.iter() {
            size += value.len();
        }

        result.push(((size >> 24) & 0xFF) as u8);
        result.push(((size >> 16) & 0xFF) as u8);
        result.push(((size >>  8) & 0xFF) as u8);
        result.push(((size) & 0xFF) as u8);
        
        for value in r#type.iter() {
            result.push(*value);
        }

        for value in data.iter() {
            for v in value.iter() {
                result.push(*v);
            }

            offset += value.len();
        }

        result
    }

    pub fn mvhd (&self, timescale: u64, duration: u64) -> Vec<u8> {
        let mut mvhd = Vec::new();
        for value in self.types.iter() {
            if value.name == "mvhd" {
                mvhd = value.list.to_vec()
            }
        }

        self.r#box(mvhd, vec![[
            0x00, 0x00, 0x00, 0x00,  // version(0) + flags
            0x00, 0x00, 0x00, 0x00,  // creation_time
            0x00, 0x00, 0x00, 0x00,  // modification_time
            ((timescale >> 24) & 0xFF) as u8,  // timescale: 4 bytes
            ((timescale >> 16) & 0xFF) as u8,
            ((timescale >>  8) & 0xFF) as u8,
            ((timescale) & 0xFF) as u8,
            ((duration >> 24) & 0xFF) as u8,   // duration: 4 bytes
            ((duration >> 16) & 0xFF) as u8,
            ((duration >>  8) & 0xFF) as u8,
            ((duration) & 0xFF) as u8,
            0x00, 0x01, 0x00, 0x00,  // Preferred rate: 1.0
            0x01, 0x00, 0x00, 0x00,  // PreferredVolume(1.0, 2bytes) + reserved(2bytes)
            0x00, 0x00, 0x00, 0x00,  // reserved: 4 + 4 bytes
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00,  // ----begin composition matrix----
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x40, 0x00, 0x00, 0x00,  // ----end composition matrix----
            0x00, 0x00, 0x00, 0x00,  // ----begin pre_defined 6 * 4 bytes----
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,  // ----end pre_defined 6 * 4 bytes----
            0xFF, 0xFF, 0xFF, 0xFF   // next_track_ID
        ].to_vec()])
    }

    pub fn tkhd (&self, meta: Meta) -> Vec<u8> {
        let mut tkhd = Vec::new();
        for value in self.types.iter() {
            if value.name == "tkhd" {
                tkhd = value.list.to_vec()
            }
        }

        self.r#box(tkhd, vec![[
            0x00, 0x00, 0x00, 0x07,  // version(0) + flags
            0x00, 0x00, 0x00, 0x00,  // creation_time
            0x00, 0x00, 0x00, 0x00,  // modification_time
            ((meta.id >> 24) & 0xFF) as u8,  // track_ID: 4 bytes
            ((meta.id >> 16) & 0xFF) as u8,
            ((meta.id >>  8) & 0xFF) as u8,
            ((meta.id) & 0xFF) as u8,
            0x00, 0x00, 0x00, 0x00,  // reserved: 4 bytes
            ((meta.duration >> 24) & 0xFF) as u8, // duration: 4 bytes
            ((meta.duration >> 16) & 0xFF) as u8,
            ((meta.duration >>  8) & 0xFF) as u8,
            ((meta.duration) & 0xFF) as u8,
            0x00, 0x00, 0x00, 0x00,  // reserved: 2 * 4 bytes
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,  // layer(2bytes) + alternate_group(2bytes)
            0x00, 0x00, 0x00, 0x00,  // volume(2bytes) + reserved(2bytes)
            0x00, 0x01, 0x00, 0x00,  // ----begin composition matrix----
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x40, 0x00, 0x00, 0x00,  // ----end composition matrix----
            ((meta.width >> 8) & 0xFF) as u8,    // width and height
            ((meta.width) & 0xFF) as u8,
            0x00, 0x00,
            ((meta.height >> 8) & 0xFF) as u8,
            ((meta.height) & 0xFF) as u8,
            0x00, 0x00
        ].to_vec()])
    }
}