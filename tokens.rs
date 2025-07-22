use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;

/// Token type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Bearer,
    API,
    Deployment,
    Session,
}

/// Token metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    /// Token ID
    pub id: String,
    /// Token type
    pub token_type: TokenType,
    /// Environment (production, staging, development)
    pub environment: String,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Expiration time (None for non-expiring tokens)
    pub expires_at: Option<DateTime<Utc>>,
    /// Associated agent ID
    pub agent_id: Option<String>,
    /// Associated user ID
    pub user_id: Option<String>,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Claims for JWT tokens
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject (token ID)
    sub: String,
    /// Issuer
    iss: String,
    /// Issued at
    iat: i64,
    /// Expiration time
    exp: Option<i64>,
    /// Token type
    #[serde(rename = "type")]
    token_type: String,
    /// Environment
    env: String,
    /// Agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    aid: Option<String>,
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    /// Custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<HashMap<String, String>>,
}

/// Token store for managing tokens
pub struct TokenStore {
    tokens: Arc<Mutex<HashMap<String, TokenMetadata>>>,
    jwt_secret: String,
}

impl TokenStore {
    /// Create a new token store
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
            jwt_secret: jwt_secret.to_string(),
        }
    }
    
    /// Generate a new token
    pub async fn generate_token(
        &self,
        token_type: TokenType,
        environment: &str,
        duration: Option<Duration>,
        agent_id: Option<&str>,
        user_id: Option<&str>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn Error>> {
        let token_id = format!("tok_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        let now = Utc::now();
        let expires_at = duration.map(|d| now + d);
        
        let token_metadata = TokenMetadata {
            id: token_id.clone(),
            token_type: token_type.clone(),
            environment: environment.to_string(),
            created_at: now,
            expires_at,
            agent_id: agent_id.map(|s| s.to_string()),
            user_id: user_id.map(|s| s.to_string()),
            metadata: metadata.unwrap_or_default(),
        };
        
        // Create JWT claims
        let claims = Claims {
            sub: token_id.clone(),
            iss: "bea-bot".to_string(),
            iat: now.timestamp(),
            exp: expires_at.map(|exp| exp.timestamp()),
            token_type: match token_type {
                TokenType::Bearer => "bearer",
                TokenType::API => "api",
                TokenType::Deployment => "deployment",
                TokenType::Session => "session",
            }.to_string(),
            env: environment.to_string(),
            aid: agent_id.map(|s| s.to_string()),
            uid: user_id.map(|s| s.to_string()),
            meta: if token_metadata.metadata.is_empty() {
                None
            } else {
                Some(token_metadata.metadata.clone())
            },
        };
        
        // Generate JWT token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        
        // Store token metadata
        let mut tokens = self.tokens.lock().await;
        tokens.insert(token_id, token_metadata);
        
        // Generate Bea Bot token format
        let token_prefix = match token_type {
            TokenType::Bearer => "bea_b",
            TokenType::API => "bea_a",
            TokenType::Deployment => "bea_d",
            TokenType::Session => "bea_s",
        };
        
        Ok(format!("{}_{}", token_prefix, token))
    }
    
    /// Validate a token
    pub async fn validate_token(&self, token: &str) -> Result<TokenMetadata, Box<dyn Error>> {
        // Extract token type and JWT
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() < 3 || parts[0] != "bea" {
            return Err("Invalid token format".into());
        }
        
        let token_type = match parts[1] {
            "b" => TokenType::Bearer,
            "a" => TokenType::API,
            "d" => TokenType::Deployment,
            "s" => TokenType::Session,
            _ => return Err("Invalid token type".into()),
        };
        
        let jwt = parts[2..].join("_");
        
        // Validate JWT
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;
        
        let claims = token_data.claims;
        
        // Check if token exists in store
        let tokens = self.tokens.lock().await;
        let token_metadata = tokens.get(&claims.sub).ok_or("Token not found")?;
        
        // Check if token is expired
        if let Some(expires_at) = token_metadata.expires_at {
            if expires_at < Utc::now() {
                return Err("Token expired".into());
            }
        }
        
        Ok(token_metadata.clone())
    }
    
    /// Revoke a token
    pub async fn revoke_token(&self, token_id: &str) -> Result<(), Box<dyn Error>> {
        let mut tokens = self.tokens.lock().await;
        tokens.remove(token_id).ok_or_else(|| "Token not found".into())?;
        Ok(())
    }
    
    /// List tokens for an agent
    pub async fn list_tokens_for_agent(&self, agent_id: &str) -> Vec<TokenMetadata> {
        let tokens = self.tokens.lock().await;
        tokens
            .values()
            .filter(|t| t.agent_id.as_deref() == Some(agent_id))
            .cloned()
            .collect()
    }
    
    /// List tokens for a user
    pub async fn list_tokens_for_user(&self, user_id: &str) -> Vec<TokenMetadata> {
        let tokens = self.tokens.lock().await;
        tokens
            .values()
            .filter(|t| t.user_id.as_deref() == Some(user_id))
            .cloned()
            .collect()
    }
}
