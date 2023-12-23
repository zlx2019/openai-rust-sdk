use serde::{Serialize, Deserialize};

///
/// 各种类型的对话消息实体
/// 


/// 聊天消息类型枚举
/// 消息的类型分为很多种，不同的消息类型所持有的的属性也不同，所以使用enum;
/// 指定 tag 为 role，表示将枚举本身序列化后作为`role`属性的值
#[derive(Debug,Clone,Serialize)]
#[serde(rename_all = "snake_case", tag = "role")]
pub enum ChatMessage {
    /// 系统消息
    System(SystemMessage),
    /// 用户消息
    User(UserMessage),
    /// 辅助消息
    Assistant(AssistantMessage),
    /// 工具消息
    Tool(ToolMessage),
}

/// 系统消息，一般指模型系统对用户的响应信息;
#[derive(Debug,Clone,Serialize)]
pub struct SystemMessage{
    /// 系统消息的内容。
    content: String,
    /// 参与者的可选名称。提供模型信息以区分相同角色的参与者。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}

/// 用户消息，一般指用户向模型系统发送的消息;
#[derive(Debug,Clone,Serialize)]
pub struct UserMessage{
    /// 用户消息的内容。
    content: String,
    /// 参与者的可选名称。提供模型信息以区分相同角色的参与者。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}


/// 辅助消息，同时可以作为系统返回时的消息体
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AssistantMessage{
    /// 消息的内容。
    pub content: String,
    /// 参与者的可选名称。提供模型信息以区分相同角色的参与者。
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// 模型生成的工具调用信息，例如函数调用。
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tool_calls: Vec<ToolCall>,
}

/// 工具消息
#[derive(Debug,Clone,Serialize)]
pub struct ToolMessage{
    /// 工具消息的内容。
    content: String,
    /// 此消息正在响应的工具调用。
    tool_call_id: String,
}


impl ChatMessage {
    /// 创建系统消息
    pub fn new_system(content: impl Into<String>, name: &str) -> ChatMessage{
        ChatMessage::System(SystemMessage { 
            content: content.into(), 
            name: Self::get_name(name)
        })
    }

    /// 创建用户消息
    pub fn new_user(content: impl Into<String>, name: &str) -> ChatMessage{
        ChatMessage::User(UserMessage { 
            content: content.into(), 
            name: Self::get_name(name) 
        })
    }


    /// 获取name
    fn get_name(name: &str) -> Option<String>{
         if name.is_empty(){
            None
        }else{
            Some(name.into())
        }
    }
}


/// 辅助工具信息
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ToolCall{
    /// 工具的ID
    pub id: String,
    /// 工具的类型，目前仅支持`function`
    pub r#type: ToolType,
    /// 工具函数信息
    pub function: CallFunction,
}

/// 工具函数信息
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct CallFunction{
    /// 调用的函数名
    pub name: String,
    /// 函数的所有参数(Json格式)
    pub arguments: String

}

/// 工具类型枚举，目前仅支持 function 类型
#[derive(Debug,Clone,Copy,Default,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    Function
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn works(){
        let message: ChatMessage =  ChatMessage::User(
            UserMessage {
                 content: "user send content.".to_string(), 
                 name: Some("zero9501".to_string()) 
            }
        );
        let json = serde_json::to_value(&message).unwrap();
        println!("{}",json);
    }
}