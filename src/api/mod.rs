//! OpenAi的各种交互类型的请求接口构建，最终都构建成一个请求对象`Reqwest::RequestBuilder`;
//! 
//! 

// 统一定义模块，并且对外公开
mod chat_completion;
mod create_image;
mod message;
pub use chat_completion::*;
pub use create_image::*;