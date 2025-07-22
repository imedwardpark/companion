use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

/// Message role for conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    role: MessageRole,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

impl Message {
    /// Create a new system message
    pub fn system(content: &str) -> Self {
        Self {
            role: MessageRole::System,
            content: content.to_string(),
            name: None,
            tool_calls: None,
        }
    }

    /// Create a new user message
    pub fn user(content: &str) -> Self {
        Self {
            role: MessageRole::User,
            content: content.to_string(),
            name: None,
            tool_calls: None,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: &str) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.to_string(),
            name: None,
            tool_calls: None,
        }
    }

    /// Create a new tool message
    pub fn tool(content: &str, name: &str) -> Self {
        Self {
            role: MessageRole::Tool,
            content: content.to_string(),
            name: Some(name.to_string()),
            tool_calls: None,
        }
    }
}

/// Tool call in a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    id: String,
    name: String,
    arguments: String,
}

/// Model request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    messages: Vec<Message>,
    model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    tools: Option<Vec<ToolDefinition>>,
    top_p: Option<f32>,
    stream: Option<bool>,
}

/// Tool definition for model request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    name: String,
    description: Option<String>,
    parameters: serde_json::Value,
}

/// Model response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    message: Message,
    model: String,
    usage: TokenUsage,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Trait for AI model providers
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// Get the provider name
    fn provider_name(&self) -> &str;
    
    /// Get available models
    async fn available_models(&self) -> Result<Vec<String>, Box<dyn Error>>;
    
    /// Generate a completion for the given request
    async fn generate(
        &self,
        request: ModelRequest,
    ) -> Result<ModelResponse, Box<dyn Error>>;
    
    /// Stream a completion for the given request
    async fn generate_stream(
        &self,
        request: ModelRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<ModelResponse, Box<dyn Error>>>, Box<dyn Error>>;
}

/// Anthropic Claude model provider
pub struct AnthropicProvider {
    api_key: String,
    client: reqwest::Client,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider with the given API key
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    /// Convert our message format to Anthropic's message format
    fn convert_messages(&self, messages: Vec<Message>) -> Vec<serde_json::Value> {
        messages
            .into_iter()
            .map(|msg| {
                let role = match msg.role {
                    MessageRole::System => "system",
                    MessageRole::User => "user",
                    MessageRole::Assistant => "assistant",
                    MessageRole::Tool => "tool",
                };
                
                let mut message = serde_json::json!({
                    "role": role,
                    "content": msg.content,
                });
                
                if let Some(name) = msg.name {
                    message["name"] = serde_json::json!(name);
                }
                
                if let Some(tool_calls) = msg.tool_calls {
                    message["tool_calls"] = serde_json::json!(tool_calls);
                }
                
                message
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for AnthropicProvider {
    fn provider_name(&self) -> &str {
        "anthropic"
    }
    
    async fn available_models(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // In a real implementation, this would query the Anthropic API
        Ok(vec![
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
            "claude-3.5-sonnet-20240613".to_string(),
            "claude-3.7-sonnet-20240519".to_string(),
        ])
    }
    
    async fn generate(
        &self,
        request: ModelRequest,
    ) -> Result<ModelResponse, Box<dyn Error>> {
        let anthropic_messages = self.convert_messages(request.messages);
        
        let mut payload = serde_json::json!({
            "model": request.model,
            "messages": anthropic_messages,
        });
        
        if let Some(temperature) = request.temperature {
            payload["temperature"] = serde_json::json!(temperature);
        }
        
        if let Some(max_tokens) = request.max_tokens {
            payload["max_tokens"] = serde_json::json!(max_tokens);
        }
        
        if let Some(tools) = request.tools {
            payload["tools"] = serde_json::json!(tools);
        }
        
        // In a real implementation, this would call the Anthropic API
        // and parse the response
        
        // Mock response for demonstration purposes
        let response = ModelResponse {
            message: Message::assistant("This is a response from Claude."),
            model: request.model,
            usage: TokenUsage {
                prompt_tokens: 128,
                completion_tokens: 64,
                total_tokens: 192,
            },
        };
        
        Ok(response)
    }
    
    async fn generate_stream(
        &self,
        request: ModelRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<ModelResponse, Box<dyn Error>>>, Box<dyn Error>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        // In a real implementation, this would stream responses from the Anthropic API
        
        // For demonstration, we'll just send a single response
        let cloned_request = request.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let response = ModelResponse {
                message: Message::assistant("This is a streamed response from Claude."),
                model: cloned_request.model,
                usage: TokenUsage {
                    prompt_tokens: 128,
                    completion_tokens: 64,
                    total_tokens: 192,
                },
            };
            
            let _ = tx.send(Ok(response)).await;
        });
        
        Ok(rx)
    }
}

/// OpenAI model provider
pub struct OpenAIProvider {
    api_key: String,
    organization: Option<String>,
    client: reqwest::Client,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider with the given API key
    pub fn new(api_key: &str, organization: Option<&str>) -> Self {
        Self {
            api_key: api_key.to_string(),
            organization: organization.map(|s| s.to_string()),
            client: reqwest::Client::new(),
        }
    }
    
    /// Convert our message format to OpenAI's message format
    fn convert_messages(&self, messages: Vec<Message>) -> Vec<serde_json::Value> {
        messages
            .into_iter()
            .map(|msg| {
                let role = match msg.role {
                    MessageRole::System => "system",
                    MessageRole::User => "user",
                    MessageRole::Assistant => "assistant",
                    MessageRole::Tool => "tool",
                };
                
                let mut message = serde_json::json!({
                    "role": role,
                    "content": msg.content,
                });
                
                if let Some(name) = msg.name {
                    message["name"] = serde_json::json!(name);
                }
                
                if let Some(tool_calls) = msg.tool_calls {
                    message["tool_calls"] = serde_json::json!(tool_calls);
                }
                
                message
            })
            .collect()
    }
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    fn provider_name(&self) -> &str {
        "openai"
    }
    
    async fn available_models(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // In a real implementation, this would query the OpenAI API
        Ok(vec![
            "gpt-4o".to_string(),
            "gpt-4-turbo".to_string(),
            "gpt-3.5-turbo".to_string(),
        ])
    }
    
    async fn generate(
        &self,
        request: ModelRequest,
    ) -> Result<ModelResponse, Box<dyn Error>> {
        let openai_messages = self.convert_messages(request.messages);
        
        let mut payload = serde_json::json!({
            "model": request.model,
            "messages": openai_messages,
        });
        
        if let Some(temperature) = request.temperature {
            payload["temperature"] = serde_json::json!(temperature);
        }
        
        if let Some(max_tokens) = request.max_tokens {
            payload["max_tokens"] = serde_json::json!(max_tokens);
        }
        
        if let Some(tools) = request.tools {
            payload["tools"] = serde_json::json!(tools);
        }
        
        // In a real implementation, this would call the OpenAI API
        // and parse the response
        
        // Mock response for demonstration purposes
        let response = ModelResponse {
            message: Message::assistant("This is a response from GPT."),
            model: request.model,
            usage: TokenUsage {
                prompt_tokens: 128,
                completion_tokens: 64,
                total_tokens: 192,
            },
        };
        
        Ok(response)
    }
    
    async fn generate_stream(
        &self,
        request: ModelRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<ModelResponse, Box<dyn Error>>>, Box<dyn Error>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        // In a real implementation, this would stream responses from the OpenAI API
        
        // For demonstration, we'll just send a single response
        let cloned_request = request.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let response = ModelResponse {
                message: Message::assistant("This is a streamed response from GPT."),
                model: cloned_request.model,
                usage: TokenUsage {
                    prompt_tokens: 128,
                    completion_tokens: 64,
                    total_tokens: 192,
                },
            };
            
            let _ = tx.send(Ok(response)).await;
        });
        
        Ok(rx)
    }
}

/// Model registry for managing providers
pub struct ModelRegistry {
    providers: Arc<Mutex<HashMap<String, Box<dyn ModelProvider>>>>,
}

impl ModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        Self {
            providers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Register a new provider
    pub async fn register_provider<P: ModelProvider + 'static>(&self, provider: P) -> Result<(), Box<dyn Error>> {
        let mut providers = self.providers.lock().await;
        providers.insert(provider.provider_name().to_string(), Box::new(provider));
        Ok(())
    }
    
    /// Get a provider by name
    pub async fn get_provider(&self, provider_name: &str) -> Option<Box<dyn ModelProvider>> {
        let providers = self.providers.lock().await;
        providers.get(provider_name).map(|p| dyn_clone::clone_box(&**p))
    }
    
    /// Generate a completion using the appropriate provider
    pub async fn generate(&self, request: ModelRequest) -> Result<ModelResponse, Box<dyn Error>> {
        let provider_name = request.model.split('/').next().unwrap_or("anthropic");
        
        let provider = self.get_provider(provider_name).await
            .ok_or_else(|| format!("Provider not found: {}", provider_name))?;
        
        provider.generate(request).await
    }
}
