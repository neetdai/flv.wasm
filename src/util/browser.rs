use woothee::parser::Parser;
use woothee::parser::WootheeResult;


///  TODO: 使用woothee解决
///      不再使用外部脚本获取
/// # 迭代器实例
/// 
///pub struct Iters {
///    pub context: Vec<String>,
///    pub index: usize
///}
///
/// 
/// # 检查指定索引是否存在
/// 
/// 
/// ## example
/// ```
/// indexof(vec![0, 1, 2, 3], 2);
/// ```
/// pub fn indexof (args: &Vec<String>, index: usize) -> bool {
///     args.len() < index
/// }
/// 
/// 
/// # 短路求值
/// 返回最先匹配到的值
/// 
/// pub fn trinocular (iters: Vec<Iters>) -> Vec<String> {
///     let mut match_values: Vec<String> = Vec::new();
///     for value in iters.iter() {
///         if indexof(&value.context, value.index) {
///             if match_values.is_empty() {
///                 match_values.push(value.context[value.index]);
///             }
///         }
///     }
/// 
///     match_values
/// }
/// 


/// # 获取浏览器用户代理
/// 
/// 
/// ## example
/// ```
/// match get_ua() {
///     None => None,
///     Some(context) => {
///         println!("{:?}", context);
///     }
/// }
/// ```
fn get_ua () -> Option<WootheeResult> {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let agent = navigator.user_agent().unwrap();
    let parser = Parser::new();
    let result = parser.parse(agent);
    result
}


// 浏览器实例
pub struct Browser {
    pub name: String
}