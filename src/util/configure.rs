use serde::{Serialize, Serializer};

/// Value作为JsValue的过渡,
/// 将Value转为JsValue用JsValue::from_serde(&Value)
/// 将JsValue转为Value用JsValue::into_serde()
#[derive(Serialize, Deserialize)]
pub struct Value<T>(T);

impl<T> Value<T> {
    pub fn new(content: T) -> Self {
        Value(content)
    }
}

/// 媒体数据类型
///
/// * flv
/// * mp4
pub enum MediaDataType {
    FLV,
    MP4,
}

impl Serialize for MediaDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        match &self {
            MediaDataType::FLV => serializer.serialize_str("flv"),
            MediaDataType::MP4 => serializer.serialize_str("mp4"),
        }
    }
}

// MediaSegment.
#[derive(Serialize)]
pub struct MediaSegment {
    pub duration: f64, // 表示段持续时间（以毫秒为单位）
    pub filesize: f64, // 表示以字节为单位的段文件大小
    pub url: String,   // 表示段文件URL
}

// MediaDataSource.
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct MediaDataSource {
    pub mediaType: MediaDataType,    // 表示媒体类型，'flv'或'mp4'
    pub isLive: bool,                // 指示数据源是否为实时流
    pub cors: bool,                  // 指示是否为http获取启用CORS
    pub withCredentials: bool,       // 指示是否使用cookie进行http获取
    pub hasAudio: bool,              // 指示流是否具有音轨
    pub hasVideo: bool,              // 指示流是否具有视频轨道
    pub duration: f64,               // 表示总媒体持续时间，以毫秒为单位
    pub filesize: f64,               // 表示媒体文件的总文件大小，以字节为单位
    pub url: String,                 // 表示媒体URL，可以以'https(s)'或'ws(s)'（WebSocket）开头
    pub segments: Vec<MediaSegment>, // 多部分播放的可选字段
}

// configure.
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Config<T> {
    pub enableWorker: bool,      // 启用分离线程进行传输
    pub enableStashBuffer: bool, // 启用IO存储缓冲区
    pub stashInitialSize: f64,   // 表示IO存储缓冲区初始大小
    pub isLive: bool,
    pub lazyLoad: bool,                 // 如果有足够的数据进行播放，则中止http连接
    pub lazyLoadMaxDuration: f64,       // 指示要保留的数据秒数
    pub lazyLoadRecoverDuration: f64,   // 以秒为单位指示恢复时间边界
    pub deferLoadAfterSourceOpen: bool, // 在sourceopen触发MediaSource 事件后加载
    pub autoCleanupSourceBuffer: bool,  // 对SourceBuffer进行自动清理
    pub autoCleanupMaxBackwardDuration: f64, // 当后向缓冲区持续时间超过此值（以秒为单位）时，请对SourceBuffer执行自动清理
    pub autoCleanupMinBackwardDuration: f64, // 指示执行自动清理时为后向缓冲区保留的持续时间（以秒为单位）
    pub fixAudioTimestampGap: bool, // 在检测到大的音频时间戳间隙时，填充静音音频帧以避免不同步
    pub accurateSeek: bool,         // 准确搜索任何帧，不限于视频IDR帧，但可能会慢一点
    pub seekType: String, // 'range'使用范围请求来寻找或'param'将params添加到url中以指示请求范围
    pub seekParamStart: String, // 表示查找起始参数名称
    pub seekParamEnd: String, // 表示查找结束参数名称
    pub rangeLoadZeroStart: bool, // 如果使用范围搜索，则首次发送加载
    pub customSeekHandler: Value<T>, // 表示自定义搜索处理程序
    pub reuseRedirectedURL: bool, // 重新使用301/302重定向的URL以获取后续请求
    pub referrerPolicy: String, // 使用FetchStreamLoader时指示Referrer策略
    pub headers: Value<T>, // 将添加到请求的其他标头
}
