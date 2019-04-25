use crate::util::configure::Config;
use crate::util::configure::MediaDataSource;
use crate::util::events::EventEmitter;
use crate::Events;
use chrono::offset::Local;
use chrono::DateTime;

// 播放器实例
pub struct FlvPlayer<T> {
    pub tag: String,
    pub types: String,
    pub emitter: Events<T>,
    pub config: Config<T>,
    pub now: DateTime<Local>,
    pub pending_seek_time: Option<T>,
    pub request_set_time: bool,
    pub seekpoint_record: Option<T>,
    pub progress_checker: Option<T>,
    pub media_data_source: MediaDataSource,
    pub media_element: Option<T>,
    pub msectl: Option<T>,
    pub transmuxer: Option<T>,
    pub mse_source_opened: bool,
    pub has_pending_load: bool,
    pub received_can_play: bool,
    pub media_info: Option<T>,
    pub statistics_info: Option<T>,
    pub always_seek_key_frame: bool,
}

impl<T> FlvPlayer<T> {
    // 创建实例.
    pub fn new(mediaDataSource: MediaDataSource, config: Config<T>) -> Self {
        FlvPlayer {
            tag: String::from("FlvPlayer"),
            types: String::from("FlvPlayer"),
            emitter: EventEmitter::new(),
            now: Local::now(),
            pending_seek_time: None,
            request_set_time: false,
            seekpoint_record: None,
            progress_checker: None,
            media_data_source: mediaDataSource,
            media_element: None,
            msectl: None,
            transmuxer: None,
            mse_source_opened: false,
            has_pending_load: false,
            received_can_play: false,
            media_info: None,
            statistics_info: None,

            // TODO: 需要浏览器UA信息
            always_seek_key_frame: false,
            config,
        }
    }

    // 销毁实例
    pub fn destroy(&mut self) {
        if self.progress_checker.is_some() {
            // window.clearInterval(this._progressChecker);
            // TODO: 需要关闭定时器
            //      可以使用Rust实现
            self.progress_checker = None;
        }

        if self.transmuxer.is_some() {
            self.unload();
        }

        if self.media_element.is_some() {
            self.detach_media_element();
        }
    }

    // 重新加载
    pub fn unload(&self) {}

    // 分离媒体元素
    pub fn detach_media_element(&self) {}
}
