extern crate web_sys;
extern crate wasm_bindgen;
extern crate chrono;
extern crate regex;
extern crate woothee;


// mod.
mod configure;
mod player;
mod events;
mod util;
mod remux;


// use.
use wasm_bindgen::prelude::*;
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