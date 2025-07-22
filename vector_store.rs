use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::Agent;

/// Theme for site generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Default,
    Light,
    Dark,
    ModernLight,
    ModernDark,
    Custom(String),
}

impl Theme {
    /// Get the CSS class for this theme
    pub fn css_class(&self) -> &str {
        match self {
            Theme::Default => "bea-theme-default",
            Theme::Light => "bea-theme-light",
            Theme::Dark => "bea-theme-dark",
            Theme::ModernLight => "bea-theme-modern-light",
            Theme::ModernDark => "bea-theme-modern-dark",
            Theme::Custom(_) => "bea-theme-custom",
        }
    }
    
    /// Parse a theme from a string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "default" => Theme::Default,
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            "modern-light" => Theme::ModernLight,
            "modern-dark" => Theme::ModernDark,
            _ => Theme::Custom(s.to_string()),
        }
    }
}

/// Authentication configuration for site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication method
    pub method: AuthMethod,
    /// Redirect URL for OAuth
    pub redirect_url: Option<String>,
    /// Client ID for OAuth
    pub client_id: Option<String>,
    /// Client secret for OAuth
    pub client_secret: Option<String>,
    /// Allowed domains for email authentication
    pub allowed_domains: Option<Vec<String>>,
    /// Custom authentication headers
    pub headers: Option<HashMap<String, String>>,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    None,
    Basic,
    OAuth2,
    OIDC,
    Email,
    Custom,
}

impl From<crate::Auth> for AuthMethod {
    fn from(auth: crate::Auth) -> Self {
        match auth {
            crate::Auth::None => AuthMethod::None,
            crate::Auth::Basic => AuthMethod::Basic,
            crate::Auth::OAuth2 => AuthMethod::OAuth2,
            crate::Auth::OIDC => AuthMethod::OIDC,
            crate::Auth::Custom(_) => AuthMethod::Custom,
        }
    }
}

/// Site configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    /// Site ID
    pub id: String,
    /// Site name
    pub name: String,
    /// Site theme
    pub theme: Theme,
    /// Custom domain
    pub domain: Option<String>,
    /// Associated agent ID
    pub agent_id: Option<String>,
    /// Authentication configuration
    pub auth: Option<AuthConfig>,
    /// Custom CSS
    pub custom_css: Option<String>,
    /// Custom JS
    pub custom_js: Option<String>,
    /// Custom HTML head
    pub custom_head: Option<String>,
    /// Site settings
    pub settings: HashMap<String, String>,
}

/// Site generator
pub struct SiteGenerator {
    pub(crate) config: SiteConfig,
}

impl SiteGenerator {
    /// Create a new site generator
    pub fn new() -> Self {
        Self {
            config: SiteConfig {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Bea Bot Agent".to_string(),
                theme: Theme::Default,
                domain: None,
                agent_id: None,
                auth: None,
                custom_css: None,
                custom_js: None,
                custom_head: None,
                settings: HashMap::new(),
            },
        }
    }
    
    /// Set the site name
    pub fn with_name(mut self, name: &str) -> Self {
        self.config.name = name.to_string();
        self
    }
    
    /// Connect the site to an agent
    pub fn with_agent(mut self, agent: &Agent) -> Self {
        self.config.agent_id = Some(agent.name().to_string());
        self.config.name = format!("{} Agent", agent.name());
        self
    }
    
    /// Set the site theme
    pub fn with_theme(mut self, theme: &str) -> Self {
        self.config.theme = Theme::from_str(theme);
        self
    }
    
    /// Set a custom domain for the site
    pub fn with_custom_domain(mut self, domain: &str) -> Self {
        self.config.domain = Some(domain.to_string());
        self
    }
    
    /// Set an authentication method for the site
    pub fn with_auth(mut self, auth: crate::Auth) -> Self {
        let method = AuthMethod::from(auth);
        
        self.config.auth = Some(AuthConfig {
            method,
            redirect_url: None,
            client_id: None,
            client_secret: None,
            allowed_domains: None,
            headers: None,
        });
        
        self
    }
    
    /// Add custom CSS to the site
    pub fn with_custom_css(mut self, css: &str) -> Self {
        self.config.custom_css = Some(css.to_string());
        self
    }
    
    /// Add custom JS to the site
    pub fn with_custom_js(mut self, js: &str) -> Self {
        self.config.custom_js = Some(js.to_string());
        self
    }
    
    /// Add custom HTML to the head
    pub fn with_custom_head(mut self, head: &str) -> Self {
        self.config.custom_head = Some(head.to_string());
        self
    }
    
    /// Add a setting to the site
    pub fn with_setting(mut self, key: &str, value: &str) -> Self {
        self.config.settings.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Generate the site HTML
    pub fn generate_html(&self) -> Result<String, Box<dyn Error>> {
        // In a real implementation, this would generate the HTML for the site
        
        // For demonstration purposes, we'll return a simple template
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css">
    <script src="https://cdn.jsdelivr.net/npm/alpinejs@3.12.3/dist/cdn.min.js" defer></script>
    {}
    <style>
        :root {{
            --primary-color: #4f46e5;
            --secondary-color: #7c3aed;
            --text-color: #111827;
            --bg-color: #ffffff;
            --accent-color: #8b5cf6;
        }}
        
        .dark {{
            --primary-color: #818cf8;
            --secondary-color: #a78bfa;
            --text-color: #f9fafb;
            --bg-color: #111827;
            --accent-color: #c4b5fd;
        }}
        
        body {{
            background-color: var(--bg-color);
            color: var(--text-color);
            transition: background-color 0.3s, color 0.3s;
        }}
        
        .chat-container {{
            height: calc(100vh - 12rem);
        }}
        
        .message {{
            border-radius: 1rem;
            padding: 1rem;
            margin-bottom: 1rem;
            max-width: 80%;
        }}
        
        .user-message {{
            background-color: var(--primary-color);
            color: white;
            align-self: flex-end;
        }}
        
        .assistant-message {{
            background-color: #f3f4f6;
            color: var(--text-color);
            align-self: flex-start;
        }}
        
        .dark .assistant-message {{
            background-color: #1f2937;
            color: #f9fafb;
        }}
        
        .typing-indicator span {{
            animation: blink 1.4s infinite both;
        }}
        
        .typing-indicator span:nth-child(2) {{
            animation-delay: 0.2s;
        }}
        
        .typing-indicator span:nth-child(3) {{
            animation-delay: 0.4s;
        }}
        
        @keyframes blink {{
            0% {{ opacity: 0.1; }}
            20% {{ opacity: 1; }}
            100% {{ opacity: 0.1; }}
        }}
        
        {}
    </style>
</head>
<body class="{}">
    <div x-data="chatApp()" class="flex flex-col h-screen">
        <nav class="bg-gray-800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                <div class="flex items-center">
                    <svg class="w-8 h-8 mr-2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="currentColor"/>
                        <path d="M12 17C14.7614 17 17 14.7614 17 12C17 9.23858 14.7614 7 12 7C9.23858 7 7 9.23858 7 12C7 14.7614 9.23858 17 12 17Z" fill="currentColor"/>
                    </svg>
                    <span class="text-xl font-bold">{}</span>
                </div>
                <div class="flex items-center">
                    <button @click="toggleTheme()" class="p-2 rounded-full hover:bg-gray-700">
                        <svg x-show="!darkMode" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>
                        </svg>
                        <svg x-show="darkMode" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                        </svg>
                    </button>
                    <button class="ml-4 px-4 py-2 bg-indigo-600 rounded-md hover:bg-indigo-700">Settings</button>
                </div>
            </div>
        </nav>
        
        <div class="flex-1 overflow-hidden flex flex-col">
            <div class="flex-1 overflow-y-auto p-4">
                <div class="container mx-auto max-w-4xl">
                    <div class="flex flex-col">
                        <template x-for="(message, index) in messages" :key="index">
                            <div :class="{{'message': true, 'user-message': message.role === 'user', 'assistant-message': message.role === 'assistant'}}">
                                <div x-text="message.content"></div>
                            </div>
                        </template>
                        <div x-show="isTyping" class="message assistant-message typing-indicator">
                            <span>.</span><span>.</span><span>.</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="border-t p-4">
                <div class="container mx-auto max-w-4xl">
                    <form @submit.prevent="sendMessage()" class="flex">
                        <input 
                            type="text" 
                            x-model="userInput" 
                            class="flex-1 rounded-l-lg border-2 border-gray-300 p-2 focus:outline-none focus:border-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                            placeholder="Type your message..."
                        >
                        <button 
                            type="submit" 
                            class="bg-indigo-600 text-white px-4 py-2 rounded-r-lg hover:bg-indigo-700 focus:outline-none"
                            :disabled="userInput.trim() === ''"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
                            </svg>
                        </button>
                    </form>
                </div>
            </div>
        </div>
    </div>

    <script>
        function chatApp() {{
            return {{
                darkMode: window.matchMedia('(prefers-color-scheme: dark)').matches,
                userInput: '',
                messages: [],
                isTyping: false,
                
                init() {{
                    if (this.darkMode) {{
                        document.body.classList.add('dark');
                    }}
                    
                    // Welcome message
                    setTimeout(() => {{
                        this.addMessage('assistant', 'Hello! I\'m your AI assistant. How can I help you today?');
                    }}, a500);
                }},
                
                toggleTheme() {{
                    this.darkMode = !this.darkMode;
                    document.body.classList.toggle('dark');
                }},
                
                sendMessage() {{
                    if (this.userInput.trim() === '') return;
                    
                    const userMessage = this.userInput;
                    this.addMessage('user', userMessage);
                    this.userInput = '';
                    
                    // Simulate typing
                    this.isTyping = true;
                    
                    // Simulate API call to agent
                    setTimeout(() => {{
                        this.isTyping = false;
                        this.addMessage('assistant', 'I\'m processing your request: "' + userMessage + '". This is a demo interface for the Bea Bot agent platform.');
                    }}, 1500);
                }},
                
                addMessage(role, content) {{
                    this.messages.push({{ role, content }});
                    // Scroll to bottom
                    setTimeout(() => {{
                        const container = document.querySelector('.overflow-y-auto');
                        container.scrollTop = container.scrollHeight;
                    }}, 50);
                }}
            }};
        }}
    </script>
    {}
</body>
</html>"#,
            self.config.name,
            self.config.custom_head.as_deref().unwrap_or(""),
            self.config.custom_css.as_deref().unwrap_or(""),
            self.config.theme.css_class(),
            self.config.name,
            self.config.custom_js.as_deref().unwrap_or(""),
        );
        
        Ok(html)
    }
    
    /// Deploy the site
    pub async fn deploy(&self) -> Result<String, Box<dyn Error>> {
        // In a real implementation, this would deploy the site to the hosting platform
        
        // For demonstration purposes, we'll just return the URL
        let domain = self.config.domain.clone().unwrap_or_else(|| {
            if let Some(agent_id) = &self.config.agent_id {
                format!("{}.bea-bot.app", agent_id)
            } else {
                format!("{}.bea-bot.app", self.config.id)
            }
        });
        
        Ok(format!("https://{}", domain))
    }
}
