

use derive_builder::Builder;
use serde::{Serialize,Deserialize};
use reqwest::{Client,RequestBuilder};
use crate::IntoRequest;

use super::message::{ChatMessage, ToolType, AssistantMessage};


///
/// 创建聊天对话API-请求体
/// 
#[derive(Debug,Clone,Serialize,Builder)]
pub struct ChatCompletionRequest{
    /// 该次对话的所有消息列表。
    #[builder(setter(into))]
    messages: Vec<ChatMessage>,
    
    /// 要使用的模型ID枚举
    #[builder(default)]
    model: Model,

    /// 控制模型生成文本时避免重复词汇或短语出现的频率。
    /// 当你使用这个属性时，你可以调整模型生成文本时对重复词汇的惩罚程度。
    /// 这样可以帮助你控制模型输出的多样性，防止模型重复使用相同的词语或短语来生成文本。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    /// 聊天完成时生成的最大令牌数。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,

    /// 为每条输入消息生成多少个聊天完成选项。请注意，您将根据所有选项生成的代币数量付费。将 n 保留为 1 以最大限度地降低成本。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<usize>,

    /// 用于控制模型在生成文本时避免包含特定词汇或短语的程度。
    /// 设置 presence_penalty 允许你规定模型生成的文本中不应该包含某些词汇或主题，
    /// 这样可以帮助你控制输出的内容，使其不倾向于包含你指定的特定词语或主题。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,

    /// 指定模型响应的数据格式，Json or Text
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ChatResponseFormatObject>,

    /// seed 属性用于确定模型生成文本的随机种子。
    /// 设置 seed 允许你指定一个整数值，以在模型生成文本时固定随机性，确保相同的输入和参数条件下得到相同的输出。
    /// 这在需要复现特定结果或调试时非常有用，因为相同的种子值将产生相同的文本输出，使得结果可预测和可重现。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<usize>,

    /// 用于指定模型生成文本的停止标记。当模型遇到 stop 中指定的词语或短语时，会停止生成文本并返回结果。
    /// 这个属性允许你控制模型生成文本的长度或确保生成的文本在某个特定点结束，
    /// 例如，当模型生成了特定的句子或段落后立即停止。这有助于限制模型输出的长度或确保生成的文本在特定条件下结束。
    /// TODO: make this as an enum
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,

    /// 设置返回数据是否以流式方式;
    /// 当设置 stream 为 false 时，API 将等待所有结果都准备就绪后一次性返回给用户。
    /// 而当 stream 设置为 true 时，API 将会以流式方式返回结果，意味着每当生成的文本准备好时就会立即返回给用户，而不必等待其他文本生成完成。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,

    /// temperature 属性是在生成文本时用来控制模型创造性和多样性的一个重要参数。它影响模型生成下一个词或字符时对概率分布进行的抽样过程。
    /// 设置较高的 temperature 值会增加模型对不同词汇的随机性，从而产生更多样化和更富创造性的输出。这意味着模型更可能选择概率较低的词汇或字符作为下一个生成的内容，增加了多样性，但也可能导致输出的不确定性增加。
    /// 相反，设置较低的 temperature 值会减少模型的随机性，使其更倾向于选择概率较高的词汇或字符作为下一个生成的内容。这样可以使得输出更加可预测，但可能会减少生成文本的多样性。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    /// top_p 是 OpenAI GPT模型（也在其他一些模型中）中的一个参数，用于控制生成文本时的采样策略。它使用一个累积概率的阈值来动态地截断模型预测的词汇或token的候选集。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    /// 模型可能调用的工具列表;
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Tool>,

    /// 控制模型调用哪个函数（如果有）。
    /// none 表示模型不会调用函数，而是生成消息。 
    /// auto 表示模型可以在生成消息或调用函数之间进行选择。
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,

    /// 代表您的最终用户的唯一标识符
    #[builder(default,setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>
}

// CreateImageRequest 实现 IntoRequest 特征，返回对应的网络请求构建器;
impl IntoRequest for ChatCompletionRequest{
    
    /// 构建post请求，指定目标url
    /// 将自身序列化为json格式，作为请求体
    fn into_request(self,client: Client) -> RequestBuilder {
        client.post("https://api.openai.com/v1/chat/completions")
        .json(&self)
    }
}


/// 工具选择枚举
#[derive(Debug,Clone,Default,PartialEq, Eq,Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice{
    /// 不调用函数
    #[default]
    None,
    /// 自动选择
    Auto,
    /// TODO: 
    Function{
        /// 要调用的函数名称
        name: String
    }
}

/// 工具实体
#[derive(Debug,Clone,Serialize)]
pub struct Tool{
    /// 工具的类型,目前仅支持 function。
    r#type: ToolType,
    /// 工具对应的函数信息
    function: FunctionInfo,
}
/// 工具函数信息实体
#[derive(Debug,Clone,Serialize)]
pub struct FunctionInfo{
    /// 工具函数功能的描述，模型使用它来选择何时以及如何调用该函数。
    description: String,
    /// 要调用的函数的名称。必须是 a-z、A-Z、0-9，或包含下划线和破折号，最大长度为 64;
    name: String,
    /// 函数的所有参数，以Json格式描述
    parameters: serde_json::Value,
}


/// 模型响应格式对象
#[derive(Debug,Clone,Serialize)]
pub struct ChatResponseFormatObject{
    /// 响应格式类型
    r#type: ChatResponseFormat
}
/// 响应格式枚举
#[derive(Debug,Clone,Copy,Default,PartialEq, Eq,Serialize)]
#[serde(rename_all = "snake_case")]
pub enum  ChatResponseFormat{
    /// 文本格式
    Text,
    /// Json格式
    #[default]
    JSON,
}


/// 可以使用的模型ID枚举，不同的模型价格不同
/// 具体模型种类可参考: https://openai.com/pricing
#[derive(Debug,Clone,Copy,Default,PartialEq, Eq,Serialize,Deserialize)]
pub enum Model{
    // GPT3相关模型;
    #[default]
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3Turbo,
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    Gpt3TurboInstruct,

    // GPT4相关模型 
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-1106-vision-preview")]
    Gpt4TurboVision,
}



///
/// 创建聊天对话API-响应体
/// 
#[derive(Debug,Clone,Deserialize)]
pub struct ChatCompletionResponse{
    /// 聊天完成的唯一标识
    pub id: String,
    
    /// 返回的响应消息列表(可能会有多个)
    /// 聊天完成选项列表，如果`n`大于1，则返回多个;
    pub choices: Vec<ChatCompletionChoice>,

    /// 创建聊天完成时的 Unix 时间戳（以秒为单位）。
    pub created: usize,

    /// 使用的模型ID
    pub model: Model,

    /// 该指纹代表模型运行时使用的后端配置。
    pub system_fingerprint: String,

    /// 对象类型，始终为 chat.completion;
    pub object: String,

    /// 完成请求的使用统计。
    pub usage: ChatCompleteUsage,
}


// token指的是文本数据的最小单元。在自然语言处理中，一个 token 可以是一个单词、一个标点符号或者一个字符，这取决于你处理的文本单位。
// 在机器学习和深度学习中，文本通常会被转换成 token 的序列来进行处理，这样模型可以更好地理解和处理文本数据。

/// 请求统计信息
#[derive(Debug,Clone,Deserialize)]
pub struct ChatCompleteUsage{
    /// 生成完成的响应中使用的 token 数量
    /// 当你向 OpenAI 的模型发送请求并收到响应时，响应中包含了生成的文本或预测的 token 数量。
    pub completion_tokens: usize,

    /// 这代表你发送的请求中的 token 数量。
    /// 在你向模型提供输入时，输入的文本被转换成 token 的形式以便模型理解
    pub prompt_tokens: usize,

    /// 这是指完成请求时总共处理的 token 数量，即 prompt_tokens 加上 completion_tokens 的总和。
    /// 这个指标考虑了你发送的输入文本以及模型生成的输出文本所涉及的所有 token 数量。
    pub total_tokens: usize
}

/// 聊天完成选项
#[derive(Debug,Clone,Deserialize)]
pub struct ChatCompletionChoice{
    /// 聊天回复的停止原因标识;
    /// 如果是响应内容自然结束或者到达请求时提供的停止标识符(stop),则返回`stop`;
    /// 如果达到请求中指定的最大令牌数，则为 `length`;
    pub finish_reason: FinishReason,

    /// 当前选项在选项列表中的索引;
    pub index: usize,

    /// 选项的消息内容;
    pub message: AssistantMessage

}

///
/// 回复结束的原因标识
#[derive(Debug,Clone,Copy,Default,PartialEq, Eq,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// 响应内容自然结束或者到达请求时提供的停止标识符(stop);
    #[default]
    Stop,
    /// 达到请求中指定的最大令牌数;
    Length,
    /// 由于内容过滤器中的标志而省略内容
    ContentFilter,
    ///
    ToolCalls
}




/// 单元测试
#[cfg(test)]
mod tests{
    use anyhow::{Result, Ok};

    use crate::{api::message::ChatMessage, OpenaiSdk};
    use super::*;

    #[test]
    fn chat_completion_request_serialize_should_work(){
        // 构建消息列表
        let messages = vec![
            ChatMessage::new_system("这是系统消息", ""),
            ChatMessage::new_user("这是用户消息", "")
        ];

        let req = ChatCompletionRequestBuilder::default()
            .tool_choice(ToolChoice::Auto)
            .messages(messages)
            .build()
            .unwrap();
        // 序列化为 Json Value
        let json_value = serde_json::to_value(req).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "tool_choice": "auto",
                "messages": [
                    {
                        "role": "system",
                        "content": "这是系统消息"
                    },
                    {
                        "role": "user",
                        "content": "这是用户消息"
                    }
                ]
            })
        )
    }


    /// 测试chat请求
    #[tokio::test]
    async fn simple_chat_completion_should_work() -> Result<()>{
        // 获取环境变量中的openai api key
        let api_key = std::env::var("OPENAI_API_KEY")?;
        // 构建sdk
        let sdk = OpenaiSdk::new(api_key);
        // 构建请求
        let req = get_simple_chat_completion_request();
        // 发送请求
        let res = sdk.chat_completion(req).await?;
        assert_eq!(res.choices.len(),1);
        assert_eq!(res.model, Model::Gpt3Turbo);
        assert_eq!(res.choices[0].finish_reason,FinishReason::Stop);
        println!("res: {:?}", res);
        Ok(())
    }


    fn get_simple_chat_completion_request()-> ChatCompletionRequest{
        // 构建消息列表
        let messages = vec![
            ChatMessage::new_system("I can answer any question you ask me.", ""),
            ChatMessage::new_user("What is human life expectancy in the world?", "")
        ];
        ChatCompletionRequestBuilder::default()
        .messages(messages)
        .build()
        .unwrap()
    }

}