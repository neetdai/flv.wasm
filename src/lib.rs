mod player;
mod remux;
mod util;

// use.
use crate::util::events::EventEmitter;
use crate::util::events::Listener;
use wasm_bindgen::prelude::*;

// types.
pub type Events<T> = EventEmitter<Listener<T>>;

// 创建播放器
// pub fn createPlayer (mediaDataSource: MediaDataSource, config: Config) {

// }
