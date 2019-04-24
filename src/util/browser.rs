use wasm_bindgen::prelude::*;
use woothee::parser::Parser;


#[wasm_bindgen(module="/script.js")]
extern "C" {
    fn ua_match(ua: &str) -> Vec<String>;
    fn platform_match(ua: &str) -> Vec<String>;
}


// 迭代器实例
pub struct Iters {
    pub context: Vec<String>,
    pub index: usize
}


/// # 检查指定索引是否存在
/// 
/// 
/// ## example
/// ```
/// indexof(vec![0, 1, 2, 3], 2);
/// ```
pub fn indexof (args: &Vec<String>, index: usize) -> bool {
    args.len() < index
}


/// # 短路求值
/// 返回最先匹配到的值
/// 
pub fn trinocular (iters: Vec<Iters>) -> Vec<String> {
    let mut match_values: Vec<String> = Vec::new();
    for value in iters.iter() {
        if indexof(&value.context, value.index) {
            if match_values.is_empty() {
                match_values.push(value.context[value.index]);
            }
        }
    }

    match_values
}


/// # 获取浏览器用户代理
/// 
/// 
/// ## example
/// ```
/// let ua: String = get_ua();
/// ```
fn get_ua () -> String {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let agent = navigator.user_agent().unwrap();
    agent
}


// 浏览器实例
pub struct Browser {
    pub name: String
}


impl Browser {
    pub fn new () -> Self {
        let ua = get_ua();
        let ua_handle = ua_match(ua.as_str());
        let platform_handle = platform_match(ua.as_str());
        let browser_match = trinocular(vec![
            Iters { context: ua_handle.to_vec(), index: 5 },
            Iters { context: ua_handle.to_vec(), index: 3 },
            Iters { context: ua_handle.to_vec(), index: 1 }
        ]);

        let mut browser = String::new();
        if browser_match.is_empty() {
            browser = browser_match[0];
        }

        Browser {
            name: browser
        }
    }
}