use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

/// Tool capability for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Tool parameters schema
    pub parameters: serde_json::Value,
    /// Required permissions
    pub required_permissions: Vec<String>,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Tool name
    pub name: String,
    /// Execution status
    pub status: ToolStatus,
    /// Result data
    pub data: serde_json::Value,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Tool execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolStatus {
    Success,
    Error,
    Pending,
}

/// Tool trait for implementing tools
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get tool name
    fn name(&self) -> &str;
    
    /// Get tool description
    fn description(&self) -> &str;
    
    /// Get tool parameters schema
    fn parameters_schema(&self) -> serde_json::Value;
    
    /// Get required permissions
    fn required_permissions(&self) -> Vec<String>;
    
    /// Execute tool with parameters
    async fn execute(&self, parameters: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>>;
}

/// Tool registry for managing tools
pub struct ToolRegistry {
    tools: Arc<Mutex<HashMap<String, Box<dyn Tool>>>>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Register a new tool
    pub async fn register_tool<T: Tool + 'static>(&self, tool: T) -> Result<(), Box<dyn Error>> {
        let mut tools = self.tools.lock().await;
        tools.insert(tool.name().to_string(), Box::new(tool));
        Ok(())
    }
    
    /// Get a tool by name
    pub async fn get_tool(&self, name: &str) -> Option<ToolCapability> {
        let tools = self.tools.lock().await;
        tools.get(name).map(|tool| ToolCapability {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters: tool.parameters_schema(),
            required_permissions: tool.required_permissions(),
        })
    }
    
    /// Execute a tool
    pub async fn execute_tool(&self, name: &str, parameters: serde_json::Value) -> Result<ToolResult, Box<dyn Error>> {
        let tools = self.tools.lock().await;
        let tool = tools.get(name).ok_or_else(|| format!("Tool not found: {}", name))?;
        
        match tool.execute(parameters.clone()).await {
            Ok(data) => Ok(ToolResult {
                name: name.to_string(),
                status: ToolStatus::Success,
                data,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                name: name.to_string(),
                status: ToolStatus::Error,
                data: serde_json::json!(null),
                error: Some(e.to_string()),
            }),
        }
    }
    
    /// List all available tools
    pub async fn list_tools(&self) -> Vec<ToolCapability> {
        let tools = self.tools.lock().await;
        tools.values().map(|tool| ToolCapability {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters: tool.parameters_schema(),
            required_permissions: tool.required_permissions(),
        }).collect()
    }
}

/// Knowledge base search tool
pub struct KnowledgeBaseSearchTool {
    name: String,
    description: String,
    parameters_schema: serde_json::Value,
    permissions: Vec<String>,
}

impl KnowledgeBaseSearchTool {
    /// Create a new knowledge base search tool
    pub fn new() -> Self {
        Self {
            name: "search_knowledge_base".to_string(),
            description: "Search the agent's knowledge base for information".to_string(),
            parameters_schema: serde_json::json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results to return",
                        "default": 5
                    }
                }
            }),
            permissions: vec!["knowledge_base:read".to_string()],
        }
    }
}

#[async_trait]
impl Tool for KnowledgeBaseSearchTool {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        self.parameters_schema.clone()
    }
    
    fn required_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }
    
    async fn execute(&self, parameters: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        // In a real implementation, this would search the knowledge base
        
        let query = parameters.get("query")
            .and_then(|q| q.as_str())
            .ok_or("Missing query parameter")?;
            
        let limit = parameters.get("limit")
            .and_then(|l| l.as_u64())
            .unwrap_or(5);
            
        // Mock search results
        let results = vec![
            serde_json::json!({
                "title": "About Bea Bot",
                "content": "Bea Bot is a platform for deploying AI agents, tokens, and sites.",
                "relevance": 0.95
            }),
            serde_json::json!({
                "title": "Deployment Guide",
                "content": "Learn how to deploy your agent to production.",
                "relevance": 0.87
            }),
            serde_json::json!({
                "title": "Token Management",
                "content": "Secure token management for your agents.",
                "relevance": 0.82
            }),
        ];
        
        Ok(serde_json::json!({
            "query": query,
            "results": results.into_iter().take(limit as usize).collect::<Vec<_>>()
        }))
    }
}

/// Web search tool
pub struct WebSearchTool {
    name: String,
    description: String,
    parameters_schema: serde_json::Value,
    permissions: Vec<String>,
}

impl WebSearchTool {
    /// Create a new web search tool
    pub fn new() -> Self {
        Self {
            name: "web_search".to_string(),
            description: "Search the web for information".to_string(),
            parameters_schema: serde_json::json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results to return",
                        "default": 5
                    }
                }
            }),
            permissions: vec!["web:search".to_string()],
        }
    }
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        self.parameters_schema.clone()
    }
    
    fn required_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }
    
    async fn execute(&self, parameters: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        // In a real implementation, this would search the web
        
        let query = parameters.get("query")
            .and_then(|q| q.as_str())
            .ok_or("Missing query parameter")?;
            
        let limit = parameters.get("limit")
            .and_then(|l| l.as_u64())
            .unwrap_or(5);
            
        // Mock search results
        let results = vec![
            serde_json::json!({
                "title": "Rust Programming Language",
                "url": "https://www.rust-lang.org/",
                "snippet": "A language empowering everyone to build reliable and efficient software.",
                "relevance": 0.95
            }),
            serde_json::json!({
                "title": "Anthropic",
                "url": "https://www.anthropic.com/",
                "snippet": "AI research and deployment company, focused on building AI systems that are safe, honest, and helpful.",
                "relevance": 0.92
            }),
            serde_json::json!({
                "title": "GitHub: Build software better, together",
                "url": "https://github.com/",
                "snippet": "GitHub is where over 100 million developers shape the future of software, together.",
                "relevance": 0.88
            }),
        ];
        
        Ok(serde_json::json!({
            "query": query,
            "results": results.into_iter().take(limit as usize).collect::<Vec<_>>()
        }))
    }
}

/// Create ticket tool
pub struct CreateTicketTool {
    name: String,
    description: String,
    parameters_schema: serde_json::Value,
    permissions: Vec<String>,
}

impl CreateTicketTool {
    /// Create a new ticket creation tool
    pub fn new() -> Self {
        Self {
            name: "create_ticket".to_string(),
            description: "Create a support ticket or issue".to_string(),
            parameters_schema: serde_json::json!({
                "type": "object",
                "required": ["title", "description"],
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "Title of the ticket"
                    },
                    "description": {
                        "type": "string",
                        "description": "Detailed description of the issue"
                    },
                    "priority": {
                        "type": "string",
                        "description": "Priority level",
                        "enum": ["low", "medium", "high", "critical"],
                        "default": "medium"
                    },
                    "assignee": {
                        "type": "string",
                        "description": "User ID of the assignee (optional)"
                    }
                }
            }),
            permissions: vec!["tickets:write".to_string()],
        }
    }
}

#[async_trait]
impl Tool for CreateTicketTool {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        self.parameters_schema.clone()
    }
    
    fn required_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }
    
    async fn execute(&self, parameters: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        // In a real implementation, this would create a ticket in the ticketing system
        
        let title = parameters.get("title")
            .and_then(|t| t.as_str())
            .ok_or("Missing title parameter")?;
            
        let description = parameters.get("description")
            .and_then(|d| d.as_str())
            .ok_or("Missing description parameter")?;
            
        let priority = parameters.get("priority")
            .and_then(|p| p.as_str())
            .unwrap_or("medium");
            
        let assignee = parameters.get("assignee")
            .and_then(|a| a.as_str());
            
        // Generate ticket ID
        let ticket_id = format!("TICKET-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap());
        
        Ok(serde_json::json!({
            "ticket_id": ticket_id,
            "title": title,
            "description": description,
            "priority": priority,
            "assignee": assignee,
            "status": "open",
            "created_at": chrono::Utc::now().to_rfc3339()
        }))
    }
}
