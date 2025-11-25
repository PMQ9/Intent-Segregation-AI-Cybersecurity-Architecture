use serde::{Deserialize, Serialize};

/// Configuration for the Ollama parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// Ollama API endpoint (default: http://localhost:11434)
    pub endpoint: String,

    /// Model to use (e.g., "llama2", "mistral")
    pub model: String,

    /// Temperature (should be 0 for deterministic output)
    pub temperature: f32,

    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:11434".to_string(),
            model: "llama2".to_string(),
            temperature: 0.0,
            timeout_secs: 30,
        }
    }
}

/// Configuration for the OpenAI parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    /// OpenAI API key
    pub api_key: String,

    /// Model to use (e.g., "gpt-4o-mini")
    pub model: String,

    /// Temperature (should be 0 for deterministic output)
    pub temperature: f32,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Base URL for API (default: https://api.openai.com/v1)
    pub base_url: String,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gpt-4o-mini".to_string(),
            temperature: 0.0,
            timeout_secs: 30,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }
}

impl OpenAIConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

/// Configuration for the ChatGPT parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGPTConfig {
    /// OpenAI API key for ChatGPT
    pub api_key: String,

    /// Model to use (e.g., "gpt-4", "gpt-4-turbo", "gpt-3.5-turbo")
    pub model: String,

    /// Temperature (should be 0 for deterministic output)
    pub temperature: f32,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Base URL for API (default: https://api.openai.com/v1)
    pub base_url: String,
}

impl Default for ChatGPTConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gpt-4-turbo".to_string(),
            temperature: 0.0,
            timeout_secs: 30,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }
}

impl ChatGPTConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

/// Configuration for the DeepSeek parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekConfig {
    /// DeepSeek API key
    pub api_key: String,

    /// Model to use (e.g., "deepseek-chat")
    pub model: String,

    /// Temperature (should be 0 for deterministic output)
    pub temperature: f32,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Base URL for API (default: https://api.deepseek.com/v1)
    pub base_url: String,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "deepseek-chat".to_string(),
            temperature: 0.0,
            timeout_secs: 30,
            base_url: "https://api.deepseek.com/v1".to_string(),
        }
    }
}

impl DeepSeekConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

/// Configuration for the Claude parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    /// Anthropic API key for Claude
    pub api_key: String,

    /// Model to use (e.g., "claude-3-opus-20250219", "claude-3-sonnet-20250219")
    pub model: String,

    /// Temperature (should be 0 for deterministic output)
    pub temperature: f32,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Base URL for API (default: https://api.anthropic.com/v1)
    pub base_url: String,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            temperature: 0.0,
            timeout_secs: 30,
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }
}

impl ClaudeConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

/// Overall parser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Enable deterministic parser
    pub enable_deterministic: bool,

    /// Enable Ollama parser
    pub enable_ollama: bool,

    /// Enable OpenAI parser
    pub enable_openai: bool,

    /// Enable ChatGPT parser
    pub enable_chatgpt: bool,

    /// Enable DeepSeek parser
    pub enable_deepseek: bool,

    /// Enable Claude parser
    pub enable_claude: bool,

    /// Ollama configuration
    pub ollama: OllamaConfig,

    /// OpenAI configuration
    pub openai: OpenAIConfig,

    /// ChatGPT configuration
    pub chatgpt: ChatGPTConfig,

    /// DeepSeek configuration
    pub deepseek: DeepSeekConfig,

    /// Claude configuration
    pub claude: ClaudeConfig,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            enable_deterministic: true,
            enable_ollama: true,
            enable_openai: true,
            enable_chatgpt: true,
            enable_deepseek: true,
            enable_claude: true,
            ollama: OllamaConfig::default(),
            openai: OpenAIConfig::default(),
            chatgpt: ChatGPTConfig::default(),
            deepseek: DeepSeekConfig::default(),
            claude: ClaudeConfig::default(),
        }
    }
}

impl ParserConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
        let chatgpt_api_key = std::env::var("CHATGPT_API_KEY").unwrap_or_default();
        let deepseek_api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_default();
        let claude_api_key = std::env::var("CLAUDE_API_KEY").unwrap_or_default();

        let ollama_endpoint = std::env::var("OLLAMA_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());

        let ollama_model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama2".to_string());

        let openai_model =
            std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
        let chatgpt_model =
            std::env::var("CHATGPT_MODEL").unwrap_or_else(|_| "gpt-4-turbo".to_string());
        let deepseek_model =
            std::env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string());
        let claude_model = std::env::var("CLAUDE_MODEL")
            .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

        Ok(Self {
            enable_deterministic: true,
            enable_ollama: std::env::var("ENABLE_OLLAMA")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            enable_openai: std::env::var("ENABLE_OPENAI")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            enable_chatgpt: std::env::var("ENABLE_CHATGPT")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            enable_deepseek: std::env::var("ENABLE_DEEPSEEK")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            enable_claude: std::env::var("ENABLE_CLAUDE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            ollama: OllamaConfig {
                endpoint: ollama_endpoint,
                model: ollama_model,
                ..Default::default()
            },
            openai: OpenAIConfig {
                api_key: openai_api_key,
                model: openai_model,
                ..Default::default()
            },
            chatgpt: ChatGPTConfig {
                api_key: chatgpt_api_key,
                model: chatgpt_model,
                ..Default::default()
            },
            deepseek: DeepSeekConfig {
                api_key: deepseek_api_key,
                model: deepseek_model,
                ..Default::default()
            },
            claude: ClaudeConfig {
                api_key: claude_api_key,
                model: claude_model,
                ..Default::default()
            },
        })
    }
}
