#[macro_use] extern crate serde_derive;
extern crate web_sys;
extern crate wasm_bindgen;
extern crate chrono;
extern crate regex;
extern crate woothee;

mod player;
mod util;
mod remux;

// use.
use wasm_bindgen::prelude::*;
use util::events::EventEmitter;
use util::events::Listener;


// types.
pub type Events<T> = EventEmitter<Listener<T>>;


// 创建播放器
// pub fn createPlayer (mediaDataSource: MediaDataSource, config: Config) {
    
// }