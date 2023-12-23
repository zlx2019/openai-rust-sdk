
use  serde::{Serialize,Deserialize};
use reqwest::{Client,RequestBuilder};
use crate::IntoRequest;
use derive_builder::Builder;


/// 用于生成图像的API构建
/// 输入要生成的图像描述，让模型生成新的图像并且返回;


///
/// 图像生成API-请求体
/// 
#[derive(Debug,Clone,Serialize,Builder)]
#[builder(pattern = "mutable")]
pub struct CreateImageRequest{
    /// 要生成的图像文本描述。dall-e-2 的最大长度为 1000 个字符， dall-e-3 的最大长度为 4000 个字符。
    #[builder(setter(into))]
    pub prompt: String,
    
    /// 要使用的模型。默认为 dall-e-2，建议使用dall-e-3，质量更好;
    #[builder(default)]
    pub model: ImageModel,
    
    /// default 表示使用默认值
    /// setter 表示允许通过 .n = 10 的方式赋值
    /// 并且如果n为None,则序列化时忽略
    
    /// 生成的图像数量,只能介于0~10之间。dall-e-3 模型只能填1;
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,
    
    /// 生成的图像质量;
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    
    /// 返回生成的图像的格式
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// 生成的图像分辨率大小
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// 生成的图像风格
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,

    /// 代表您的最终用户的唯一标识符
    #[builder(default,setter(strip_option,into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
// CreateImageRequest 构造方法
impl CreateImageRequest{
    pub fn new(prompt: impl Into<String>) -> Self {
        // 通过构建器创建实例，未传入的属性都会使用默认值;
        CreateImageRequestBuilder::default()
        .prompt(prompt)
        .build()
        .unwrap()
    }
}

// CreateImageRequest 实现 IntoRequest 特征，返回对应的网络请求构建器;
impl IntoRequest for CreateImageRequest{
    /// 构建post请求，指定目标url
    /// 将自身序列化为json格式，作为请求体
    fn into_request(self,client: Client) -> RequestBuilder {
        client.post("https://api.openai.com/v1/images/generations").json(&self)
    }
}



///
/// 图片聊天API-响应体
/// 
#[derive(Debug,Clone,Deserialize)]
pub struct CreateImageResponse{
    /// 图像创建时间戳
    pub created: u64,
    /// 生成的图像对象列表
    pub data: Vec<ImageObject>
}

/// 单张图像的实体
#[derive(Debug,Clone,Deserialize)]
pub struct ImageObject{
    /// 生成的图像(base64)
    /// 当请求参数 response_format 为 b64_json 时返回该值;
    pub b64_json: Option<String>,

    /// 生成的图像地址
    /// 当请求参数 response_format 为 url 时返回该值;
    pub url: Option<String>,

    /// 用于生成图像的提示（如果提示有任何修订）。
    pub revised_prompt: Option<String>
}



/// 可以使用的模型枚举
/// dall-e-3  更加强大
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
pub enum ImageModel {
    #[serde(rename = "dall-e-2")]
    DallE2,
    #[serde(rename = "dall-e-3")]
    #[default]
    DallE3,
}


/// 图像生成质量枚举
/// hd`具有更高质量，但是仅 dall-e-3 模型支持此参数。
/// 枚举值直接序列化为 首字母小写
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageQuality{
    Standard,
    #[default]
    Hd
}


/// 返回生成的图像的格式枚举。
/// 必须是 url 或 b64_json 之一。
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    /// 返回图像的url
    #[default]
    Url,
    /// 将图像编码为base64格式直接返回
    B64Json,
}


/// 生成的图像分辨率大小
/// 对于 dall-e-3 模型，必须是 1024x1024 、 1792x1024 或 1024x1792 之一。
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
pub enum ImageSize {
    #[serde(rename = "1024x1024")]
    #[default]
    Large,
    #[serde(rename = "1792x1024")]
    LargeWide,
    #[serde(rename = "1024x1792")]
    LargeTall,
}

/// 生成的图像风格
/// 必须是 vivid 或 natural 之一。生动使模型倾向于生成超真实和戏剧性的图像。
/// 自然使模型生成更自然、不太真实的图像。仅 dall-e-3 支持此参数。
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageStyle {
    /// 生动风格(默认)
    #[default]
    Vivid, // 生动
    /// 自然
    Natural
}



/// 单元测试
#[cfg(test)]
mod test{
    use std::fs;
    use crate::OpenaiSdk;
    use super::*;
    use anyhow::{Result, Ok};
    use serde_json::json;


    ///
    /// 将 CreateImageRequest 转换为 json
    /// 
    #[test]
    fn create_image_request_should_serialize() -> Result<()>{
        let  req =  CreateImageRequest::new("hello world!");
        let req_json = serde_json::to_value(&req)?;
        println!("{}",req_json);
        assert_eq!(
            req_json,
            json!({
                "prompt": "hello world!",
                "model" : "dall-e-3"
            })
        );
        Ok(())
    }
    #[test]
    fn create_image_request_custom_should_serialize() -> Result<()>{
        let  req =  CreateImageRequestBuilder::default()
        .prompt("hello world!")
        .style(ImageStyle::Natural)
        .quality(ImageQuality::Hd)
        .build()?;
        let req_json = serde_json::to_value(&req)?;
        assert_eq!(
            req_json,
            json!({
                "prompt": "hello world!",
                "model" : "dall-e-3",
                "style": "natural",
                "quality": "hd",
            })
        );
        Ok(())
    }


    /// 单元测试: 发送请求，生成图像，并且下载到本地
    #[tokio::test]
    #[ignore] // 跳过单元测试
    async fn create_image_should_work() -> Result<()>{
        // 获取环境变量中的openai api key
        let api_key = std::env::var("OPENAI_API_KEYs")?;
        // 构建sdk
        let sdk = OpenaiSdk::new(api_key);
        // 构建创建图像请求
        let img_req = CreateImageRequest::new("");
        // 发送请求
        let res = sdk.create_image(img_req).await?;
        assert_eq!(res.data.len(), 1);
        // 获取生成的图像信息
        let img = &res.data[0];
        assert!(img.url.is_some());
        // 根据生成url，下载图片
        let content = reqwest::get(img.url.as_ref().unwrap()).await?.bytes().await?;
        // 将图片保存到本地
        fs::write("/Users/zero/Desktop/dog1.png", content)?;
        println!("图片地址: {}",&img.url.clone().unwrap());
        Ok(())
    }
}