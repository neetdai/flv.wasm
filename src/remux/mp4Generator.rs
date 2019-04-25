// use.
use crate::util::set_vec;

// 类型
pub struct Type {
    pub name: &'static str,
    pub list: Vec<u8>,
}

// Flag
pub struct Flag {
    pub is_leading: u64,
    pub depends_on: u64,
    pub is_depended_on: u64,
    pub has_redundancy: u64,
    pub is_non_sync: u64,
}

// Sample
pub struct Sample {
    pub duration: u64,
    pub size: u64,
    pub flags: Flag,
    pub cts: u64,
}

// 媒体信息
pub struct Meta {
    pub id: u64,
    pub codec: &'static str,
    pub types: &'static str,
    pub duration: u64,
    pub timescale: u64,
    pub width: u64,
    pub height: u64,
    pub channel_count: u8,
    pub audio_sample_rate: u64,
    pub configs: Vec<u8>,
    pub avcc: Vec<u8>,
}

// 轨道信息
pub struct Track {
    pub id: u64,
    pub sequence_number: i32,
    pub samples: Vec<Sample>,
}

// 常量
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
    pub VMHD: Vec<u8>,
}

// mp4.
pub struct MP4 {
    pub types: Vec<Type>,
    pub constants: Constants,
}

impl MP4 {
    /// # 构造函数
    ///
    /// ## example
    /// ```
    /// MP4::new();
    /// ```
    ///
    pub fn new() -> Self {
        MP4 {
            types: vec![
                Type {
                    name: "avc1",
                    list: Vec::new(),
                },
                Type {
                    name: "avcC",
                    list: Vec::new(),
                },
                Type {
                    name: "btrt",
                    list: Vec::new(),
                },
                Type {
                    name: "dinf",
                    list: Vec::new(),
                },
                Type {
                    name: "dref",
                    list: Vec::new(),
                },
                Type {
                    name: "esds",
                    list: Vec::new(),
                },
                Type {
                    name: "ftyp",
                    list: Vec::new(),
                },
                Type {
                    name: "hdlr",
                    list: Vec::new(),
                },
                Type {
                    name: "mdat",
                    list: Vec::new(),
                },
                Type {
                    name: "mdhd",
                    list: Vec::new(),
                },
                Type {
                    name: "mdia",
                    list: Vec::new(),
                },
                Type {
                    name: "mfhd",
                    list: Vec::new(),
                },
                Type {
                    name: "minf",
                    list: Vec::new(),
                },
                Type {
                    name: "moof",
                    list: Vec::new(),
                },
                Type {
                    name: "moov",
                    list: Vec::new(),
                },
                Type {
                    name: "mp4a",
                    list: Vec::new(),
                },
                Type {
                    name: "mvex",
                    list: Vec::new(),
                },
                Type {
                    name: "mvhd",
                    list: Vec::new(),
                },
                Type {
                    name: "sdtp",
                    list: Vec::new(),
                },
                Type {
                    name: "stbl",
                    list: Vec::new(),
                },
                Type {
                    name: "stco",
                    list: Vec::new(),
                },
                Type {
                    name: "stsc",
                    list: Vec::new(),
                },
                Type {
                    name: "stsd",
                    list: Vec::new(),
                },
                Type {
                    name: "stsz",
                    list: Vec::new(),
                },
                Type {
                    name: "stts",
                    list: Vec::new(),
                },
                Type {
                    name: "tfdt",
                    list: Vec::new(),
                },
                Type {
                    name: "tfhd",
                    list: Vec::new(),
                },
                Type {
                    name: "traf",
                    list: Vec::new(),
                },
                Type {
                    name: "trak",
                    list: Vec::new(),
                },
                Type {
                    name: "trun",
                    list: Vec::new(),
                },
                Type {
                    name: "trex",
                    list: Vec::new(),
                },
                Type {
                    name: "tkhd",
                    list: Vec::new(),
                },
                Type {
                    name: "vmhd",
                    list: Vec::new(),
                },
                Type {
                    name: "smhd",
                    list: Vec::new(),
                },
                Type {
                    name: ".mp3",
                    list: Vec::new(),
                },
            ],
            constants: Constants {
                FTYP: [
                    0x69, 0x73, 0x6F, 0x6D, // major_brand: isom
                    0x0, 0x0, 0x0, 0x1, // minor_version: 0x01
                    0x69, 0x73, 0x6F, 0x6D, // isom
                    0x61, 0x76, 0x63, 0x31, // avc1
                ]
                .to_vec(),
                STSD_PREFIX: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x01, // entry_count
                ]
                .to_vec(),
                STTS: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // entry_count
                ]
                .to_vec(),
                STCO: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // entry_count
                ]
                .to_vec(),
                STSC: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // entry_count
                ]
                .to_vec(),
                STSZ: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // sample_size
                    0x00, 0x00, 0x00, 0x00, // sample_count
                ]
                .to_vec(),
                HDLR_VIDEO: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // pre_defined
                    0x76, 0x69, 0x64, 0x65, // handler_type: 'vide'
                    0x00, 0x00, 0x00, 0x00, // reserved: 3 * 4 bytes
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x69, 0x64, 0x65, 0x6F,
                    0x48, 0x61, 0x6E, 0x64, 0x6C, 0x65, 0x72, 0x00, // name: VideoHandler
                ]
                .to_vec(),
                HDLR_AUDIO: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // pre_defined
                    0x73, 0x6F, 0x75, 0x6E, // handler_type: 'soun'
                    0x00, 0x00, 0x00, 0x00, // reserved: 3 * 4 bytes
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x53, 0x6F, 0x75, 0x6E, 0x64,
                    0x48, 0x61, 0x6E, 0x64, 0x6C, 0x65, 0x72, 0x00, // name: SoundHandler
                ]
                .to_vec(),
                DREF: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x01, // entry_count
                    0x00, 0x00, 0x00, 0x0C, // entry_size
                    0x75, 0x72, 0x6C, 0x20, // type 'url '
                    0x00, 0x00, 0x00, 0x01, // version(0) + flags
                ]
                .to_vec(),
                SMHD: [
                    0x00, 0x00, 0x00, 0x00, // version(0) + flags
                    0x00, 0x00, 0x00, 0x00, // balance(2) + reserved(2)
                ]
                .to_vec(),
                VMHD: [
                    0x00, 0x00, 0x00, 0x01, // version(0) + flags
                    0x00, 0x00, // graphicsmode: 2 bytes
                    0x00, 0x00, 0x00, 0x00, // opcolor: 3 * 2 bytes
                    0x00, 0x00,
                ]
                .to_vec(),
            },
        }
    }

    /// # Find types data
    ///
    fn find_type(&self, is_type: &str) -> Vec<u8> {
        let mut type_value = Vec::new();
        for value in self.types.iter() {
            if value.name == is_type {
                type_value = value.list.to_vec()
            }
        }

        type_value
    }

    /// # Create Box
    ///
    pub fn create_box(&self, is_type: Vec<u8>, data: Vec<Vec<u8>>) -> Vec<u8> {
        let mut size = 0;
        let mut offset = 8;
        let mut result: Vec<u8> = Vec::new();

        // count.
        for value in data.iter() {
            size += value.len();
        }

        // length.
        result.push(((size >> 24) & 0xFF) as u8);
        result.push(((size >> 16) & 0xFF) as u8);
        result.push(((size >> 8) & 0xFF) as u8);
        result.push(((size) & 0xFF) as u8);

        // type.
        for value in is_type.iter() {
            result.push(*value);
        }

        // data.
        for value in data.iter() {
            set_vec(&mut result, value, offset);
            // add global offset.
            offset += value.len();
        }

        result
    }

    /// # Generate Init Segment
    ///
    pub fn generate_init_segment(&self, meta: Meta) -> Vec<u8> {
        let moov = self.movie_metadata(meta);
        let ftyp = self.create_box(self.find_type("ftyp"), vec![self.constants.FTYP.to_vec()]);

        let mut data = Vec::new();
        set_vec(&mut data, &ftyp, 0);
        set_vec(&mut data, &moov, ftyp.len());
        data
    }

    /// # Movie metadata
    ///
    pub fn movie_metadata(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("moov"),
            vec![
                self.movie_header(meta.timescale, meta.duration),
                self.track(meta),
                self.movie_extends(meta),
            ],
        )
    }

    /// # Movie header
    ///
    pub fn movie_header(&self, timescale: u64, duration: u64) -> Vec<u8> {
        self.create_box(
            self.find_type("mvhd"),
            vec![[
                0x00,
                0x00,
                0x00,
                0x00, // version(0) + flags
                0x00,
                0x00,
                0x00,
                0x00, // creation_time
                0x00,
                0x00,
                0x00,
                0x00,                             // modification_time
                ((timescale >> 24) & 0xFF) as u8, // timescale: 4 bytes
                ((timescale >> 16) & 0xFF) as u8,
                ((timescale >> 8) & 0xFF) as u8,
                ((timescale) & 0xFF) as u8,
                ((duration >> 24) & 0xFF) as u8, // duration: 4 bytes
                ((duration >> 16) & 0xFF) as u8,
                ((duration >> 8) & 0xFF) as u8,
                ((duration) & 0xFF) as u8,
                0x00,
                0x01,
                0x00,
                0x00, // Preferred rate: 1.0
                0x01,
                0x00,
                0x00,
                0x00, // PreferredVolume(1.0, 2bytes) + reserved(2bytes)
                0x00,
                0x00,
                0x00,
                0x00, // reserved: 4 + 4 bytes
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x01,
                0x00,
                0x00, // ----begin composition matrix----
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x01,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x40,
                0x00,
                0x00,
                0x00, // ----end composition matrix----
                0x00,
                0x00,
                0x00,
                0x00, // ----begin pre_defined 6 * 4 bytes----
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00, // ----end pre_defined 6 * 4 bytes----
                0xFF,
                0xFF,
                0xFF,
                0xFF, // next_track_ID
            ]
            .to_vec()],
        )
    }

    /// # Track
    ///
    pub fn track(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("trak"),
            vec![self.track_header(meta), self.media(meta)],
        )
    }

    /// # Track header
    ///
    pub fn track_header(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("tkhd"),
            vec![[
                0x00,
                0x00,
                0x00,
                0x07, // version(0) + flags
                0x00,
                0x00,
                0x00,
                0x00, // creation_time
                0x00,
                0x00,
                0x00,
                0x00,                           // modification_time
                ((meta.id >> 24) & 0xFF) as u8, // track_ID: 4 bytes
                ((meta.id >> 16) & 0xFF) as u8,
                ((meta.id >> 8) & 0xFF) as u8,
                ((meta.id) & 0xFF) as u8,
                0x00,
                0x00,
                0x00,
                0x00,                                 // reserved: 4 bytes
                ((meta.duration >> 24) & 0xFF) as u8, // duration: 4 bytes
                ((meta.duration >> 16) & 0xFF) as u8,
                ((meta.duration >> 8) & 0xFF) as u8,
                ((meta.duration) & 0xFF) as u8,
                0x00,
                0x00,
                0x00,
                0x00, // reserved: 2 * 4 bytes
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00, // layer(2bytes) + alternate_group(2bytes)
                0x00,
                0x00,
                0x00,
                0x00, // volume(2bytes) + reserved(2bytes)
                0x00,
                0x01,
                0x00,
                0x00, // ----begin composition matrix----
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x01,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x40,
                0x00,
                0x00,
                0x00,                             // ----end composition matrix----
                ((meta.width >> 8) & 0xFF) as u8, // width and height
                ((meta.width) & 0xFF) as u8,
                0x00,
                0x00,
                ((meta.height >> 8) & 0xFF) as u8,
                ((meta.height) & 0xFF) as u8,
                0x00,
                0x00,
            ]
            .to_vec()],
        )
    }

    /// # Media
    ///
    pub fn media(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("mdia"),
            vec![
                self.media_hrader(meta),
                self.media_handler_reference(meta),
                self.media_infomation(meta),
            ],
        )
    }

    /// # Media header
    ///
    pub fn media_hrader(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("mdhd"),
            vec![[
                0x00,
                0x00,
                0x00,
                0x00, // version(0) + flags
                0x00,
                0x00,
                0x00,
                0x00, // creation_time
                0x00,
                0x00,
                0x00,
                0x00,                                  // modification_time
                ((meta.timescale >> 24) & 0xFF) as u8, // timescale: 4 bytes
                ((meta.timescale >> 16) & 0xFF) as u8,
                ((meta.timescale >> 8) & 0xFF) as u8,
                ((meta.timescale) & 0xFF) as u8,
                ((meta.duration >> 24) & 0xFF) as u8, // duration: 4 bytes
                ((meta.duration >> 16) & 0xFF) as u8,
                ((meta.duration >> 8) & 0xFF) as u8,
                ((meta.duration) & 0xFF) as u8,
                0x55,
                0xC4, // language: und (undetermined)
                0x00,
                0x00, // pre_defined = 0
            ]
            .to_vec()],
        )
    }

    /// # Media handler reference
    ///
    pub fn media_handler_reference(&self, meta: Meta) -> Vec<u8> {
        match meta.types {
            "audio" => self.create_box(
                self.find_type("hdlr"),
                vec![self.constants.HDLR_AUDIO.to_vec()],
            ),
            _ => self.create_box(
                self.find_type("hdlr"),
                vec![self.constants.HDLR_VIDEO.to_vec()],
            ),
        }
    }

    /// # Media infomation
    ///
    pub fn media_infomation(&self, meta: Meta) -> Vec<u8> {
        let boxs = match meta.types {
            "audio" => self.create_box(self.find_type("smhd"), vec![self.constants.SMHD.to_vec()]),
            _ => self.create_box(self.find_type("vmhd"), vec![self.constants.VMHD.to_vec()]),
        };

        self.create_box(
            self.find_type("minf"),
            vec![boxs, self.data_infomation(), self.sample_table(meta)],
        )
    }

    /// # 数据信息
    ///
    pub fn data_infomation(&self) -> Vec<u8> {
        self.create_box(
            self.find_type("dinf"),
            vec![self.create_box(self.find_type("dref"), vec![self.constants.DREF.to_vec()])],
        )
    }

    /// # 样本描述
    ///
    pub fn sample_table(&self, meta: Meta) -> Vec<u8> {
        match meta.types {
            "audio" => match meta.codec {
                "mp3" => self.create_box(
                    self.find_type("stsd"),
                    vec![self.constants.STSD_PREFIX.to_vec(), self.mp3(meta)],
                ),
                _ => self.create_box(
                    self.find_type("stsd"),
                    vec![self.constants.STSD_PREFIX.to_vec(), self.mp4a(meta)],
                ),
            },
            _ => self.create_box(
                self.find_type("stsd"),
                vec![self.constants.STSD_PREFIX.to_vec(), self.avc1(meta)],
            ),
        }
    }

    /// # MP3
    ///
    pub fn mp3(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type(".mp3"),
            vec![[
                0x00,
                0x00,
                0x00,
                0x00, // reserved(4)
                0x00,
                0x00,
                0x00,
                0x01, // reserved(2) + data_reference_index(2)
                0x00,
                0x00,
                0x00,
                0x00, // reserved: 2 * 4 bytes
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                meta.channel_count, // channelCount(2)
                0x00,
                0x10, // sampleSize(2)
                0x00,
                0x00,
                0x00,
                0x00,                                         // reserved(4)
                ((meta.audio_sample_rate >> 8) & 0xFF) as u8, // Audio sample rate
                ((meta.audio_sample_rate) & 0xFF) as u8,
                0x00,
                0x00,
            ]
            .to_vec()],
        )
    }

    /// # mp4a
    ///
    pub fn mp4a(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("mp4a"),
            vec![
                [
                    0x00,
                    0x00,
                    0x00,
                    0x00, // reserved(4)
                    0x00,
                    0x00,
                    0x00,
                    0x01, // reserved(2) + data_reference_index(2)
                    0x00,
                    0x00,
                    0x00,
                    0x00, // reserved: 2 * 4 bytes
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    meta.channel_count, // channelCount(2)
                    0x00,
                    0x10, // sampleSize(2)
                    0x00,
                    0x00,
                    0x00,
                    0x00,                                         // reserved(4)
                    ((meta.audio_sample_rate >> 8) & 0xFF) as u8, // Audio sample rate
                    ((meta.audio_sample_rate) & 0xFF) as u8,
                    0x00,
                    0x00,
                ]
                .to_vec(),
                self.esds(meta),
            ],
        )
    }

    /// # esds
    ///
    pub fn esds(&self, meta: Meta) -> Vec<u8> {
        let mut data: Vec<u8> = [
            0x00,
            0x00,
            0x00,
            0x00,                              // version 0 + flags
            0x03,                              // descriptor_type
            (0x17 + meta.configs.len()) as u8, // length3
            0x00,
            0x01,                              // es_id
            0x00,                              // stream_priority
            0x04,                              // descriptor_type
            (0x0F + meta.configs.len()) as u8, // length
            0x40,                              // codec: mpeg4_audio
            0x15,                              // stream_type: Audio
            0x00,
            0x00,
            0x00, // buffer_size
            0x00,
            0x00,
            0x00,
            0x00, // maxBitrate
            0x00,
            0x00,
            0x00,
            0x00, // avgBitrate
            0x05, // descriptor_type
            meta.configs.len() as u8,
        ]
        .to_vec();

        for value in meta.configs.iter() {
            data.push(*value);
        }

        self.create_box(self.find_type("esds"), vec![data])
    }

    /// # avc1
    ///
    pub fn avc1(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("avc1"),
            vec![
                [
                    0x00,
                    0x00,
                    0x00,
                    0x00, // reserved(4)
                    0x00,
                    0x00,
                    0x00,
                    0x01, // reserved(2) + data_reference_index(2)
                    0x00,
                    0x00,
                    0x00,
                    0x00, // pre_defined(2) + reserved(2)
                    0x00,
                    0x00,
                    0x00,
                    0x00, // pre_defined: 3 * 4 bytes
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    ((meta.width >> 8) & 0xFF) as u8, // width: 2 bytes
                    ((meta.width) & 0xFF) as u8,
                    ((meta.height >> 8) & 0xFF) as u8, // height: 2 bytes
                    ((meta.height) & 0xFF) as u8,
                    0x00,
                    0x48,
                    0x00,
                    0x00, // horizresolution: 4 bytes
                    0x00,
                    0x48,
                    0x00,
                    0x00, // vertresolution: 4 bytes
                    0x00,
                    0x00,
                    0x00,
                    0x00, // reserved: 4 bytes
                    0x00,
                    0x01, // frame_count
                    0x0A, // strlen
                    0x78,
                    0x71,
                    0x71,
                    0x2F, // compressorname: 32 bytes
                    0x66,
                    0x6C,
                    0x76,
                    0x2E,
                    0x6A,
                    0x73,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x18, // depth
                    0xFF,
                    0xFF, // pre_defined = -1
                ]
                .to_vec(),
                self.create_box(self.find_type("avcC"), vec![meta.avcc]),
            ],
        )
    }

    /// # Movie Extends
    ///
    pub fn movie_extends(&self, meta: Meta) -> Vec<u8> {
        self.create_box(self.find_type("mvex"), vec![self.track_extends(meta)])
    }

    /// # Track Extends
    ///
    pub fn track_extends(&self, meta: Meta) -> Vec<u8> {
        self.create_box(
            self.find_type("trex"),
            vec![[
                0x00,
                0x00,
                0x00,
                0x00,                           // version(0) + flags
                ((meta.id >> 24) & 0xFF) as u8, // track_ID
                ((meta.id >> 16) & 0xFF) as u8,
                ((meta.id >> 8) & 0xFF) as u8,
                ((meta.id) & 0xFF) as u8,
                0x00,
                0x00,
                0x00,
                0x01, // default_sample_description_index
                0x00,
                0x00,
                0x00,
                0x00, // default_sample_duration
                0x00,
                0x00,
                0x00,
                0x00, // default_sample_size
                0x00,
                0x01,
                0x00,
                0x01, // default_sample_flags
            ]
            .to_vec()],
        )
    }

    /// # Movie fragment
    ///
    pub fn movie_fragment(&self, track: Track, baseMediaDecodeTime: i32) -> Vec<u8> {
        self.create_box(
            self.find_type("moof"),
            vec![
                self.movie_fragment_hd(&track.sequence_number),
                self.track_fragment(&track, baseMediaDecodeTime),
            ],
        )
    }

    /// # 视频片段
    ///
    pub fn movie_fragment_hd(&self, sequenceNumber: &i32) -> Vec<u8> {
        self.create_box(
            self.find_type("mfhd"),
            vec![[
                0x00,
                0x00,
                0x00,
                0x00,
                ((sequenceNumber >> 24) & 0xFF) as u8, // sequence_number: int32
                ((sequenceNumber >> 16) & 0xFF) as u8,
                ((sequenceNumber >> 8) & 0xFF) as u8,
                ((sequenceNumber) & 0xFF) as u8,
            ]
            .to_vec()],
        )
    }

    /// # 轨道片段
    ///
    pub fn track_fragment(&self, track: &Track, baseMediaDecodeTime: i32) -> Vec<u8> {
        let sample_dependency_type = self.sample_dependency_type(track);
        self.create_box(
            self.find_type("traf"),
            vec![
                self.create_box(
                    self.find_type("tfhd"),
                    vec![[
                        0x00,
                        0x00,
                        0x00,
                        0x00,                            // version(0) & flags
                        ((track.id >> 24) & 0xFF) as u8, // track_ID
                        ((track.id >> 16) & 0xFF) as u8,
                        ((track.id >> 8) & 0xFF) as u8,
                        ((track.id) & 0xFF) as u8,
                    ]
                    .to_vec()],
                ),
                self.create_box(
                    self.find_type("tfdt"),
                    vec![[
                        0x00,
                        0x00,
                        0x00,
                        0x00,                                       // version(0) & flags
                        ((baseMediaDecodeTime >> 24) & 0xFF) as u8, // baseMediaDecodeTime: int32
                        ((baseMediaDecodeTime >> 16) & 0xFF) as u8,
                        ((baseMediaDecodeTime >> 8) & 0xFF) as u8,
                        ((baseMediaDecodeTime) & 0xFF) as u8,
                    ]
                    .to_vec()],
                ),
                self.track_fragment_run(&track, (sample_dependency_type.len() + 72) as u64),
                sample_dependency_type,
            ],
        )
    }

    /// # Sample Dependency Type box
    ///
    pub fn sample_dependency_type(&self, track: &Track) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(4 + track.samples.len());
        for (index, value) in track.samples.iter().enumerate() {
            let tag = (value.flags.is_leading << 6)    // is_leading: 2 (bit)
                    | (value.flags.depends_on << 4)    // sample_depends_on
                    | (value.flags.is_depended_on << 2) // sample_is_depended_on
                    | (value.flags.has_redundancy);
            data.insert(index + 4, tag as u8);
        }

        self.create_box(self.find_type("sdtp"), vec![data])
    }

    /// # Track fragment run box
    ///
    pub fn track_fragment_run(&self, track: &Track, offset: u64) -> Vec<u8> {
        let count = track.samples.len();
        let size = (count * 16 + 12) as u64;
        let mut data: Vec<u8> = Vec::new();
        let offset_copy = offset + 8 + size;
        let head = [
            0x00,
            0x00,
            0x0F,
            0x01,                         // version(0) & flags
            ((count >> 24) & 0xFF) as u8, // sample_count
            ((count >> 16) & 0xFF) as u8,
            ((count >> 8) & 0xFF) as u8,
            ((count) & 0xFF) as u8,
            ((offset_copy >> 24) & 0xFF) as u8, // data_offset
            ((offset_copy >> 16) & 0xFF) as u8,
            ((offset_copy >> 8) & 0xFF) as u8,
            ((offset_copy) & 0xFF) as u8,
        ]
        .to_vec();

        for value in head.iter() {
            data.push(*value);
        }

        for (index, value) in track.samples.iter().enumerate() {
            let flags = &value.flags;
            let buf = [
                ((value.duration >> 24) & 0xFF) as u8, // sample_duration
                ((value.duration >> 16) & 0xFF) as u8,
                ((value.duration >> 8) & 0xFF) as u8,
                ((value.duration) & 0xFF) as u8,
                ((value.size >> 24) & 0xFF) as u8, // sample_size
                ((value.size >> 16) & 0xFF) as u8,
                ((value.size >> 8) & 0xFF) as u8,
                ((value.size) & 0xFF) as u8,
                ((flags.is_leading << 2) | flags.depends_on) as u8, // sample_flags
                ((flags.is_depended_on << 6) | (flags.has_redundancy << 4) | flags.is_non_sync)
                    as u8,
                0x00,
                0x00,                             // sample_degradation_priority
                ((value.cts >> 24) & 0xFF) as u8, // sample_composition_time_offset
                ((value.cts >> 16) & 0xFF) as u8,
                ((value.cts >> 8) & 0xFF) as u8,
                ((value.cts) & 0xFF) as u8,
            ]
            .to_vec();
            set_vec(&mut data, &buf, index * 16 + 12);
        }

        self.create_box(self.find_type("trun"), vec![data])
    }

    /// # Movie data type
    ///
    pub fn movie_data_type(&self, data: Vec<u8>) -> Vec<u8> {
        self.create_box(self.find_type("mdat"), vec![data])
    }
}
