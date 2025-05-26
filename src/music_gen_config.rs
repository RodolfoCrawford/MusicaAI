use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

/// Configuration for the complete MusicGen pipeline
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct MusicGenConfig {
    #[serde(default = "default_audio_encoder")]
    #[validate]
    pub audio_encoder: AudioEncoderConfig,
    
    #[serde(default = "default_decoder")]
    #[validate]
    pub decoder: DecoderConfig,
    
    #[serde(default = "default_text_encoder")]
    #[validate]
    pub text_encoder: TextEncoderConfig,
    
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    
    #[serde(default = "default_device")]
    pub device: String,
}

/// Audio encoder configuration
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct AudioEncoderConfig {
    #[serde(default = "default_sampling_rate")]
    #[validate(range(min = 8000, max = 192000))]
    pub sampling_rate: usize,
    
    #[serde(default = "default_hop_length")]
    pub hop_length: usize,
    
    #[serde(default = "default_n_fft")]
    pub n_fft: usize,
}

/// Transformer decoder configuration
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DecoderConfig {
    #[serde(default = "default_num_attention_heads")]
    #[validate(range(min = 1, max = 32))]
    pub num_attention_heads: usize,
    
    #[serde(default = "default_num_hidden_layers")]
    #[validate(range(min = 1, max = 24))]
    pub num_hidden_layers: usize,
    
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    
    #[serde(default = "default_pad_token_id")]
    pub pad_token_id: i64,
    
    #[serde(default = "default_hidden_size")]
    pub hidden_size: usize,
}

/// Text encoder configuration
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct TextEncoderConfig {
    #[serde(default = "default_d_kv")]
    pub d_kv: usize,
    
    #[serde(default = "default_d_model")]
    pub d_model: usize,
    
    #[serde(default = "default_max_position_embeddings")]
    pub max_position_embeddings: usize,
}

// Default value implementations
fn default_audio_encoder() -> AudioEncoderConfig {
    AudioEncoderConfig {
        sampling_rate: default_sampling_rate(),
        hop_length: default_hop_length(),
        n_fft: default_n_fft(),
    }
}

fn default_decoder() -> DecoderConfig {
    DecoderConfig {
        num_attention_heads: default_num_attention_heads(),
        num_hidden_layers: default_num_hidden_layers(),
        top_k: default_top_k(),
        pad_token_id: default_pad_token_id(),
        hidden_size: default_hidden_size(),
    }
}

fn default_text_encoder() -> TextEncoderConfig {
    TextEncoderConfig {
        d_kv: default_d_kv(),
        d_model: default_d_model(),
        max_position_embeddings: default_max_position_embeddings(),
    }
}

// Individual default values
fn default_sampling_rate() -> usize { 44100 }
fn default_hop_length() -> usize { 512 }
fn default_n_fft() -> usize { 2048 }
fn default_num_attention_heads() -> usize { 8 }
fn default_num_hidden_layers() -> usize { 6 }
fn default_top_k() -> usize { 50 }
fn default_pad_token_id() -> i64 { 0 }
fn default_hidden_size() -> usize { 768 }
fn default_d_kv() -> usize { 64 }
fn default_d_model() -> usize { 768 }
fn default_max_position_embeddings() -> usize { 512 }
fn default_batch_size() -> usize { 1 }
fn default_device() -> String { "cpu".to_string() }

/// Configuration error types
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid configuration value: {0}")]
    ValidationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl MusicGenConfig {
    /// Load configuration from JSON file
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    /// Save configuration to JSON file
    pub fn save_to_file(&self, path: &str) -> Result<(), ConfigError> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.audio_encoder.validate()
            .map_err(|e| ConfigError::ValidationError(e.to_string()))?;
        self.decoder.validate()
            .map_err(|e| ConfigError::ValidationError(e.to_string()))?;
        self.text_encoder.validate()
            .map_err(|e| ConfigError::ValidationError(e.to_string()))?;
        
        if self.batch_size == 0 {
            return Err(ConfigError::ValidationError("Batch size cannot be zero".to_string()));
        }
        
        Ok(())
    }
    
    /// Create configuration with default values
    pub fn default() -> Self {
        Self {
            audio_encoder: default_audio_encoder(),
            decoder: default_decoder(),
            text_encoder: default_text_encoder(),
            batch_size: default_batch_size(),
            device: default_device(),
        }
    }
}
