extern crate chrono;
extern crate regex;


// mod.
mod configure;
mod player;
mod events;
mod util;


// use.
use events::EventEmitter;
use events::Listener;


// types.
pub type Events<T> = EventEmitter<Listener<T>>;


// use.
// use configure::MediaDataSource;
// use configure::Config;


// 创建播放器
// pub fn createPlayer (mediaDataSource: MediaDataSource, config: Config) {
    
// }