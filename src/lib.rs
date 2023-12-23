//! 
//! 使用 Rust语言封装的 OpenAI-SDK 工具包
//! 

use std::time::Duration;
use anyhow::{Result, Ok};
use reqwest::{Client, RequestBuilder};

// 使用api模块，并且对外暴露
pub mod api;
use api::*;

///
/// 核心 SDK 结构体
/// 
#[derive(Debug, Clone)]
pub struct OpenaiSdk{
    /// 要使用的openai账号的 api key
    pub(crate) token: String,
    /// 网络请求客户端
    pub(crate) client: Client
}



// SDK 实现块
impl OpenaiSdk {
    
    ///
    /// 传入openai的apikey，并且初始化网络请求客户端
    /// 
    pub fn new(token: String) -> Self{
        Self { token: token, client: Client::new() }
    }

    ///
    /// 文字聊天类型 api请求发送
    /// 
    pub async fn chat_completion(&self, req: ChatCompletionRequest) -> Result<ChatCompletionResponse>{
        // 构建请求
        let req = self.prepare_request(req);
        // 发送请求
        let res = req.send().await?;
        Ok(res.json::<ChatCompletionResponse>().await?)
    }

    ///
    /// 生成图片 api 请求发送
    /// 
    pub async fn create_image(&self,req: CreateImageRequest) -> Result<CreateImageResponse>{
        let req = self.prepare_request(req);
        let res = req.send().await?;
        Ok(res.json::<CreateImageResponse>().await?)
    }

    /// 将IntoRequest实现的结构体，统一转为为 `reqwest::RequestBuilder`,并且设置通用参数: token、timeout等
    fn prepare_request(&self,req: impl IntoRequest) -> RequestBuilder{
        // 使用网络请求客户端Clinet，构建出一个网络请求
        let req = req.into_request(self.client.clone());
        // 设置令牌(api-key)
        let req = if self.token.is_empty(){
            req
        }else{
            req.bearer_auth(&self.token)
        };
        // 设置超时请求超时时间
        req.timeout(Duration::from_secs(30))
    }

}

/// 创建一个Request特征，让所有类型的自定义Request，都可以构建为RequestBuilder
pub trait IntoRequest {
    fn into_request(self,client: Client) -> RequestBuilder;
}