mod player;
mod util;
mod remux;

// use.
use wasm_bindgen::prelude::*;
use crate::util::events::EventEmitter;
use crate::util::events::Listener;


// types.
pub type Events<T> = EventEmitter<Listener<T>>;


// 创建播放器
// pub fn createPlayer (mediaDataSource: MediaDataSource, config: Config) {
    
// }