use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod config;
pub mod models;
pub mod tokens;
pub mod sites;
pub mod deploy;
pub mod tools;

/// Represents an AI agent with configurable parameters
#[derive(Debug, Clone)]
pub struct Agent {
    name: String,
    model: String,
    memory_enabled: bool,
    context: String,
    tools: Vec<Tool>,
    dataset_path: Option<String>,
    performance_tier: String,
    parameters: HashMap<String, String>,
}

impl Agent {
    /// Create a new agent with the specified name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            model: "anthropic/claude-3-haiku".to_string(),
            memory_enabled: false,
            context: "You are a helpful assistant.".to_string(),
            tools: Vec::new(),
            dataset_path: None,
            performance_tier: "standard".to_string(),
            parameters: HashMap::new(),
        }
    }

    /// Set the AI model for this agent
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    /// Enable or disable agent memory
    pub fn with_memory(mut self, enabled: bool) -> Self {
        self.memory_enabled = enabled;
        self
    }

    /// Set the context/system prompt for the agent
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    /// Add tools to the agent
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }

    /// Add a dataset for the agent to use
    pub fn with_dataset(mut self, path: &str) -> Self {
        self.dataset_path = Some(path.to_string());
        self
    }

    /// Set the performance tier for the agent
    pub fn with_performance_tier(mut self, tier: &str) -> Self {
        self.performance_tier = tier.to_string();
        self
    }

    /// Set a custom parameter for the agent
    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }

    /// Get the agent's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the agent's model
    pub fn model(&self) -> &str {
        &self.model
    }
}

/// Represents a tool that can be used by an agent
#[derive(Debug, Clone)]
pub struct Tool {
    name: String,
    description: Option<String>,
    parameters: HashMap<String, String>,
}

impl Tool {
    /// Create a new tool with the specified name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            parameters: HashMap::new(),
        }
    }

    /// Add a description to the tool
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a parameter to the tool
    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }
}

/// Manages secure tokens for agent deployment and API access
#[derive(Debug, Clone)]
pub struct TokenManager {
    tokens: HashMap<String, String>,
}

impl TokenManager {
    /// Create a new token manager
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Generate a new token for the specified environment
    pub fn generate_token(mut self, environment: &str) -> Self {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;
        
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();
        
        self.tokens.insert(environment.to_string(), format!("bea_{}", token));
        self
    }

    /// Retrieve a token for the specified environment
    pub fn get_token(&self, environment: &str) -> Option<&String> {
        self.tokens.get(environment)
    }
}

/// Generates web interfaces for agents
#[derive(Debug, Clone)]
pub struct SiteGenerator {
    agent: Option<Agent>,
    theme: String,
    domain: Option<String>,
    auth_method: Option<Auth>,
    custom_css: Option<String>,
}

impl SiteGenerator {
    /// Create a new site generator
    pub fn new() -> Self {
        Self {
            agent: None,
            theme: "default".to_string(),
            domain: None,
            auth_method: None,
            custom_css: None,
        }
    }

    /// Connect the site to an agent
    pub fn with_agent(mut self, agent: &Agent) -> Self {
        self.agent = Some(agent.clone());
        self
    }

    /// Set the site theme
    pub fn with_theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        self
    }

    /// Set a custom domain for the site
    pub fn with_custom_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    /// Set an authentication method for the site
    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth_method = Some(auth);
        self
    }

    /// Add custom CSS to the site
    pub fn with_custom_css(mut self, css: &str) -> Self {
        self.custom_css = Some(css.to_string());
        self
    }
}

/// Authentication methods for site access
#[derive(Debug, Clone)]
pub enum Auth {
    None,
    Basic,
    OAuth2,
    OIDC,
    Custom(String),
}

/// State manager for deployed agents
#[derive(Debug)]
pub struct AgentStateManager {
    agents: Arc<Mutex<HashMap<String, AgentState>>>,
}

impl AgentStateManager {
    /// Create a new agent state manager
    pub fn new() -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a new agent
    pub async fn register_agent(&self, agent: &Agent) -> Result<(), Box<dyn Error>> {
        let mut agents = self.agents.lock().await;
        agents.insert(agent.name().to_string(), AgentState::new(agent));
        Ok(())
    }
}

/// Represents the runtime state of a deployed agent
#[derive(Debug)]
struct AgentState {
    agent: Agent,
    created_at: chrono::DateTime<chrono::Utc>,
    request_count: u64,
    last_active: chrono::DateTime<chrono::Utc>,
}

impl AgentState {
    /// Create a new agent state
    fn new(agent: &Agent) -> Self {
        let now = chrono::Utc::now();
        Self {
            agent: agent.clone(),
            created_at: now,
            request_count: 0,
            last_active: now,
        }
    }
}

/// Deploy an agent with optional token manager and site
pub async fn deploy(
    agent: Agent, 
    token_manager: Option<TokenManager>, 
    site: Option<SiteGenerator>
) -> Result<String, Box<dyn Error>> {
    // This would contain actual deployment logic
    // For now, we'll just return a mock endpoint
    
    let domain = if let Some(site_gen) = &site {
        site_gen.domain.clone().unwrap_or_else(|| format!("{}.bea-bot.app", agent.name()))
    } else {
        format!("{}.bea-bot.app", agent.name())
    };
    
    Ok(format!("https://{}", domain))
}

/// Deploy an agent to a specific environment
pub async fn deploy_to_env(
    agent: Agent,
    environment: &str,
) -> Result<String, Box<dyn Error>> {
    // This would contain environment-specific deployment logic
    
    let domain = match environment {
        "production" => format!("{}.bea-bot.app", agent.name()),
        "staging" => format!("{}.staging.bea-bot.app", agent.name()),
        "development" => format!("{}.dev.bea-bot.app", agent.name()),
        _ => format!("{}.{}.bea-bot.app", agent.name(), environment),
    };
    
    Ok(format!("https://{}", domain))
}
