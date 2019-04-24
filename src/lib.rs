extern crate chrono;
extern crate regex;

mod common_enum;
mod value;
mod configure;
mod player;
mod events;
mod util;

// use.
use wasm_bindgen::prelude::*;
use common_enum::{MediaDataType};
use value::Value;
use events::EventEmitter;
use events::Listener;


// types.
pub type Events<T> = EventEmitter<Listener<T>>;


// MediaDataSource.
#[derive(Serialize)]
struct MediaDataSource {
    Type: MediaDataType,  // 表示媒体类型，'flv'或'mp4'
    isLive: bool,  // 指示数据源是否为实时流
    cors: bool,  // 指示是否为http获取启用CORS
    withCredentials: bool, // 指示是否使用cookie进行http获取
    hasAudio: bool,  // 指示流是否具有音轨
    hasVideo: bool, // 指示流是否具有视频轨道
    duration: f64,  // 表示总媒体持续时间，以毫秒为单位
    filesize: f64,  // 表示媒体文件的总文件大小，以字节为单位
    url: String,  // 表示媒体URL，可以以'https(s)'或'ws(s)'（WebSocket）开头
    segments: Vec<MediaSegment>  // 多部分播放的可选字段
}


// configure.
#[derive(Serialize)]
struct Config<T> {
    enableWorker: bool,  // 启用分离线程进行传输
    enableStashBuffer: bool,  // 启用IO存储缓冲区
    stashInitialSize: f64,  // 表示IO存储缓冲区初始大小
    isLive: bool,
    lazyLoad: bool,  // 如果有足够的数据进行播放，则中止http连接
    lazyLoadMaxDuration: f64,  // 指示要保留的数据秒数
    lazyLoadRecoverDuration: f64,  // 以秒为单位指示恢复时间边界
    deferLoadAfterSourceOpen: bool,  // 在sourceopen触发MediaSource 事件后加载
    autoCleanupSourceBuffer: bool,  // 对SourceBuffer进行自动清理
    autoCleanupMaxBackwardDuration: f64,  // 当后向缓冲区持续时间超过此值（以秒为单位）时，请对SourceBuffer执行自动清理
    autoCleanupMinBackwardDuration: f64,  // 指示执行自动清理时为后向缓冲区保留的持续时间（以秒为单位）
    fixAudioTimestampGap: bool,  // 在检测到大的音频时间戳间隙时，填充静音音频帧以避免不同步
    accurateSeek: bool,  // 准确搜索任何帧，不限于视频IDR帧，但可能会慢一点
    seekType: String,  // 'range'使用范围请求来寻找或'param'将params添加到url中以指示请求范围
    seekParamStart: String,  // 表示查找起始参数名称
    seekParamEnd: String,  // 表示查找结束参数名称
    rangeLoadZeroStart: bool,  // 如果使用范围搜索，则首次发送加载
    customSeekHandler: Value<T>, // 表示自定义搜索处理程序
    reuseRedirectedURL: bool,  // 重新使用301/302重定向的URL以获取后续请求
    referrerPolicy: String,  // 使用FetchStreamLoader时指示Referrer策略
    headers: Value<T> // 将添加到请求的其他标头
}


// 创建播放器
// pub fn createPlayer (mediaDataSource: MediaDataSource, config: Config) {
    
// }