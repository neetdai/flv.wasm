// 类型
pub struct Type {
    pub name: &'static str,
    pub list: Vec<u8>
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
    pub audio_sample_rate: u64
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
    pub VMHD: Vec<u8>
}


// mp4.
pub struct MP4 {
    pub types: Vec<Type>,
    pub constants: Constants
}


impl MP4 {

    /// # 构造函数
    /// 
    /// ## example
    /// ```
    /// MP4::new();
    /// ```
    /// 
    pub fn new () -> Self {
        MP4 { types: vec![
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
        ], constants: Constants {
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
        } }
    }

    /// # 查找类型数据
    /// 
    fn find_type (&self, is_type: &str) -> Vec<u8> {
        let mut type_value = Vec::new();
        for value in self.types.iter() {
            if value.name == is_type {
                type_value = value.list.to_vec()
            }
        }

        type_value
    }

    /// # 创建盒子
    ///
    pub fn create_box (&self, is_type: Vec<u8>, data: Vec<Vec<u8>>) -> Vec<u8> {
        let mut size = 0;
        let mut offset = 8;
        let mut result: Vec<u8> = Vec::new();
        
        // 计算总量
        for value in data.iter() {
            size += value.len();
        }

        // 长度
        result.push(((size >> 24) & 0xFF) as u8);
        result.push(((size >> 16) & 0xFF) as u8);
        result.push(((size >>  8) & 0xFF) as u8);
        result.push(((size) & 0xFF) as u8);

        // 类型
        for value in is_type.iter() {
            result.push(*value);
        }

        // 数据
        for value in data.iter() {
            let mut offset_copy = offset;
            for v in value.iter() {
                // 检查容量，避免非法写入
                // 如不满足写入条件，扩容
                if result.len() - 1 < offset_copy {
                    result.reserve(offset_copy + 1);
                }
                // 写入数据
                // 增加局部偏移
                result.insert(*v as usize, offset_copy as u8);
                offset_copy += 1;
            }
            // 增加全局偏移
            offset += value.len();
        }

        result
    }

    /// # 视频头部
    ///
    pub fn movie_header (&self, timescale: u64, duration: u64) -> Vec<u8> {
        self.create_box(self.find_type("mvhd"), vec![[
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

    /// # 轨道头部
    /// 
    pub fn track_header (&self, meta: Meta) -> Vec<u8> {
        self.create_box(self.find_type("tkhd"), vec![[
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

    /// # 媒体头部
    /// 
    pub fn media_hrader (&self, meta: Meta) -> Vec<u8> {
        self.create_box(self.find_type("mdhd"), vec![[
            0x00, 0x00, 0x00, 0x00,  // version(0) + flags
            0x00, 0x00, 0x00, 0x00,  // creation_time
            0x00, 0x00, 0x00, 0x00,  // modification_time
            ((meta.timescale >> 24) & 0xFF) as u8,  // timescale: 4 bytes
            ((meta.timescale >> 16) & 0xFF) as u8,
            ((meta.timescale >>  8) & 0xFF) as u8,
            ((meta.timescale) & 0xFF) as u8,
            ((meta.duration >> 24) & 0xFF) as u8,   // duration: 4 bytes
            ((meta.duration >> 16) & 0xFF) as u8,
            ((meta.duration >>  8) & 0xFF) as u8,
            ((meta.duration) & 0xFF) as u8,
            0x55, 0xC4,             // language: und (undetermined)
            0x00, 0x00              // pre_defined = 0
        ].to_vec()])
    }

    /// # 媒体处理引用
    /// 
    pub fn media_handler_reference (&self, meta: Meta) -> Vec<u8> {
        match meta.types {
            "audio" => self.create_box(self.find_type("hdlr"), vec![ 
                self.constants.HDLR_AUDIO.to_vec() 
            ]),
            _ => self.create_box(self.find_type("hdlr"), vec![ 
                self.constants.HDLR_VIDEO.to_vec()
            ])
        }
    }

    /// # 媒体信息
    /// 
    pub fn media_infomation (&self, meta: Meta) -> Vec<u8> {
       let boxs = match meta.types {
            "audio" => self.create_box(self.find_type("smhd"), vec![ 
                self.constants.SMHD.to_vec() 
            ]),
            _ => self.create_box(self.find_type("vmhd"), vec![ 
                self.constants.VMHD.to_vec()
            ])
        };

        self.create_box(self.find_type("minf"), vec![ 
            boxs, 
            self.data_infomation(),
            self.sample_table(meta)
        ]) 
    }

    /// # 数据信息
    /// 
    pub fn data_infomation (&self) -> Vec<u8> {
        self.create_box(self.find_type("dinf"), vec![
            self.create_box(self.find_type("dref"), vec![
                self.constants.DREF.to_vec()
            ])
        ])
    }

    /// # 样本描述
    /// 
    pub fn sample_table (&self, meta: Meta) -> Vec<u8> {
        match meta.types {
            "audio" => match meta.codec {
                "mp3" => self.create_box(self.find_type("stsd"), vec![
                    self.constants.STSD_PREFIX.to_vec(),
                    self.mp3(meta)
                ]),
                _ => self.create_box(self.find_type("stsd"), vec![
                    self.constants.STSD_PREFIX.to_vec(),
                    self.mp4a(meta)
                ])
            },
            _ => self.create_box(self.find_type("stsd"), vec![
                self.constants.STSD_PREFIX.to_vec(),
                self.avc1(meta)
            ])
        }
    }

    /// # MP3
    /// 
    pub fn mp3 (&self, meta: Meta) -> Vec<u8> {
        self.create_box(self.find_type(".mp3"), vec![[
            0x00, 0x00, 0x00, 0x00,  // reserved(4)
            0x00, 0x00, 0x00, 0x01,  // reserved(2) + data_reference_index(2)
            0x00, 0x00, 0x00, 0x00,  // reserved: 2 * 4 bytes
            0x00, 0x00, 0x00, 0x00,
            0x00, meta.channel_count,      // channelCount(2)
            0x00, 0x10,              // sampleSize(2)
            0x00, 0x00, 0x00, 0x00,  // reserved(4)
            ((meta.audio_sample_rate >> 8) & 0xFF) as u8,  // Audio sample rate
            ((meta.audio_sample_rate) & 0xFF) as u8,
            0x00, 0x00
        ].to_vec()])
    }

    /// # mp4a
    /// 
    pub fn mp4a (&self, meta: Meta) -> Vec<u8> {
        self.create_box(self.find_type("mp4a"), vec![[
            0x00, 0x00, 0x00, 0x00,  // reserved(4)
            0x00, 0x00, 0x00, 0x01,  // reserved(2) + data_reference_index(2)
            0x00, 0x00, 0x00, 0x00,  // reserved: 2 * 4 bytes
            0x00, 0x00, 0x00, 0x00,
            0x00, meta.channel_count,      // channelCount(2)
            0x00, 0x10,              // sampleSize(2)
            0x00, 0x00, 0x00, 0x00,  // reserved(4)
            ((meta.audio_sample_rate >> 8) & 0xFF) as u8,  // Audio sample rate
            ((meta.audio_sample_rate) & 0xFF) as u8,
            0x00, 0x00
        ].to_vec(), self.esds(meta)])
    }

    pub fn esds (&self, meta: Meta) -> Vec<u8> {
        self.create_box(self.find_type("esds"), vec![[[
            0x00, 0x00, 0x00, 0x00,  // version 0 + flags

            0x03,                    // descriptor_type
            0x17 + configSize,       // length3
            0x00, 0x01,              // es_id
            0x00,                    // stream_priority

            0x04,                    // descriptor_type
            0x0F + configSize,       // length
            0x40,                    // codec: mpeg4_audio
            0x15,                    // stream_type: Audio
            0x00, 0x00, 0x00,        // buffer_size
            0x00, 0x00, 0x00, 0x00,  // maxBitrate
            0x00, 0x00, 0x00, 0x00,  // avgBitrate

            0x05                     // descriptor_type
        ],to_vec()]])
    }
}