// MediaSegment.
pub struct MediaSegment {
    pub duration: f64,  // 表示段持续时间（以毫秒为单位）
    pub filesize: f64,  // 表示以字节为单位的段文件大小
    pub url: String  // 表示段文件URL
}


// MediaDataSource.
pub struct MediaDataSource {
    pub types: String,  // 表示媒体类型，'flv'或'mp4'
    pub is_live: bool,  // 指示数据源是否为实时流
    pub cors: bool,  // 指示是否为http获取启用CORS
    pub with_credentials: bool, // 指示是否使用cookie进行http获取
    pub has_audio: bool,  // 指示流是否具有音轨
    pub has_video: bool, // 指示流是否具有视频轨道
    pub duration: f64,  // 表示总媒体持续时间，以毫秒为单位
    pub filesize: f64,  // 表示媒体文件的总文件大小，以字节为单位
    pub url: String,  // 表示媒体URL，可以以'https(s)'或'ws(s)'（WebSocket）开头
    pub segments: Vec<MediaSegment>  // 多部分播放的可选字段
}


// configure.
pub struct Config {
    pub enable_worker: bool,  // 启用分离线程进行传输
    pub enable_stash_buffer: bool,  // 启用IO存储缓冲区
    pub stash_initial_size: f64,  // 表示IO存储缓冲区初始大小
    pub is_live: bool,
    pub lazy_load: bool,  // 如果有足够的数据进行播放，则中止http连接
    pub lazy_load_max_duration: f64,  // 指示要保留的数据秒数
    pub lazy_load_recover_duration: f64,  // 以秒为单位指示恢复时间边界
    pub defer_load_after_source_open: bool,  // 在sourceopen触发MediaSource 事件后加载
    pub auto_cleanup_source_buffer: bool,  // 对SourceBuffer进行自动清理
    pub auto_cleanup_max_backward_duration: f64,  // 当后向缓冲区持续时间超过此值（以秒为单位）时，请对SourceBuffer执行自动清理
    pub auto_cleanup_min_backward_duration: f64,  // 指示执行自动清理时为后向缓冲区保留的持续时间（以秒为单位）
    pub fix_audio_timestamp_gap: bool,  // 在检测到大的音频时间戳间隙时，填充静音音频帧以避免不同步
    pub accurate_seek: bool,  // 准确搜索任何帧，不限于视频IDR帧，但可能会慢一点
    pub seek_type: String,  // 'range'使用范围请求来寻找或'param'将params添加到url中以指示请求范围
    pub seek_param_start: String,  // 表示查找起始参数名称
    pub seek_param_end: String,  // 表示查找结束参数名称
    pub range_load_zero_start: bool,  // 如果使用范围搜索，则首次发送加载
    pub reuse_redirected_url: bool,  // 重新使用301/302重定向的URL以获取后续请求
    pub referrer_policy: String  // 使用FetchStreamLoader时指示Referrer策略
}