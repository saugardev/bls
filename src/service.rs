use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::encryption::BLSEncryption;
use crate::keys::KeyManager;
use crate::types::*;

#[derive(Clone)]
pub struct AppState {
    encryption: BLSEncryption,
    #[allow(dead_code)]
    key_manager: Option<KeyManager>,
    public_key_bytes: Option<Vec<u8>>,
}

impl AppState {
    pub fn new(secret_key_path: Option<String>) -> anyhow::Result<Self> {
        let (key_manager, public_key_bytes) = if let Some(path) = secret_key_path {
            let secret_key_bytes = KeyManager::load_secret_key_from_file(&path)?;
            let key_manager = KeyManager::from_secret_key(&secret_key_bytes)?;
            let public_key_bytes = key_manager.get_public_key_bytes()?;
            (Some(key_manager), Some(public_key_bytes))
        } else {
            (None, None)
        };

        Ok(Self {
            encryption: BLSEncryption::new(),
            key_manager,
            public_key_bytes,
        })
    }
}

pub async fn start_service(port: u16, secret_key_path: Option<String>) -> anyhow::Result<()> {
    let state = Arc::new(AppState::new(secret_key_path)?);

    let app = Router::new()
        .route("/encrypt", post(encrypt_handler))
        .route("/decrypt", post(decrypt_handler))
        .route("/pubkey", get(get_public_key_handler))
        .route("/health", get(health_handler))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    println!("🚀 BLS Encryption Service starting on port {}", port);
    println!("📡 Endpoints:");
    println!("   POST /encrypt  - Encrypt data");
    println!("   POST /decrypt  - Decrypt data");
    println!("   GET  /pubkey   - Get public key");
    println!("   GET  /health   - Health check");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn encrypt_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, StatusCode> {
    // Decode input data
    let data = match BASE64.decode(&request.data) {
        Ok(data) => data,
        Err(_) => {
            return Ok(Json(EncryptResponse {
                encrypted_data: EncryptedData {
                    encrypted_aes_key: vec![],
                    encrypted_content: vec![],
                    nonce: vec![],
                    data_hash: String::new(),
                    public_key_id: String::new(),
                },
                success: false,
                message: "Invalid base64 data".to_string(),
            }));
        }
    };

    // Decode public key
    let public_key_bytes = match BASE64.decode(&request.public_key) {
        Ok(key) => key,
        Err(_) => {
            return Ok(Json(EncryptResponse {
                encrypted_data: EncryptedData {
                    encrypted_aes_key: vec![],
                    encrypted_content: vec![],
                    nonce: vec![],
                    data_hash: String::new(),
                    public_key_id: String::new(),
                },
                success: false,
                message: "Invalid base64 public key".to_string(),
            }));
        }
    };

    // Encrypt data
    match state.encryption.encrypt(&data, &public_key_bytes) {
        Ok(encrypted_data) => Ok(Json(EncryptResponse {
            encrypted_data,
            success: true,
            message: "Data encrypted successfully".to_string(),
        })),
        Err(e) => Ok(Json(EncryptResponse {
            encrypted_data: EncryptedData {
                encrypted_aes_key: vec![],
                encrypted_content: vec![],
                nonce: vec![],
                data_hash: String::new(),
                public_key_id: String::new(),
            },
            success: false,
            message: format!("Encryption failed: {}", e),
        })),
    }
}

async fn decrypt_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DecryptRequest>,
) -> Result<Json<DecryptResponse>, StatusCode> {
    // Decode secret key
    let secret_key_bytes = match BASE64.decode(&request.secret_key) {
        Ok(key) => key,
        Err(_) => {
            return Ok(Json(DecryptResponse {
                data: String::new(),
                success: false,
                message: "Invalid base64 secret key".to_string(),
            }));
        }
    };

    // Decrypt data
    match state.encryption.decrypt(&request.encrypted_data, &secret_key_bytes) {
        Ok(decrypted_data) => {
            let encoded_data = BASE64.encode(&decrypted_data);
            Ok(Json(DecryptResponse {
                data: encoded_data,
                success: true,
                message: "Data decrypted successfully".to_string(),
            }))
        }
        Err(e) => Ok(Json(DecryptResponse {
            data: String::new(),
            success: false,
            message: format!("Decryption failed: {}", e),
        })),
    }
}

async fn get_public_key_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<PublicKeyResponse>, StatusCode> {
    match &state.public_key_bytes {
        Some(public_key_bytes) => {
            let encoded_key = BASE64.encode(public_key_bytes);
            let key_id = KeyManager::generate_key_id(public_key_bytes);
            
            Ok(Json(PublicKeyResponse {
                public_key: encoded_key,
                key_id,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "bls-encryption-service",
        "version": "0.1.0"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;
    use crate::keys::KeyManager;

    #[tokio::test]
    async fn test_health_endpoint() {
        let state = Arc::new(AppState::new(None).unwrap());
        let app = Router::new()
            .route("/health", get(health_handler))
            .with_state(state);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_flow() {
        // Generate test keypair
        let keypair = KeyManager::generate_keypair().unwrap();
        
        let state = Arc::new(AppState::new(None).unwrap());
        let app = Router::new()
            .route("/encrypt", post(encrypt_handler))
            .route("/decrypt", post(decrypt_handler))
            .with_state(state);

        // Test data
        let test_data = "Hello, BLS service!";
        let encoded_data = BASE64.encode(test_data.as_bytes());
        let encoded_public_key = BASE64.encode(&keypair.public_key);
        let encoded_secret_key = BASE64.encode(&keypair.secret_key);

        // Encrypt request
        let encrypt_request = EncryptRequest {
            data: encoded_data,
            public_key: encoded_public_key,
        };

        let encrypt_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/encrypt")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&encrypt_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(encrypt_response.status(), StatusCode::OK);

        let encrypt_body = axum::body::to_bytes(encrypt_response.into_body(), usize::MAX).await.unwrap();
        let encrypt_result: EncryptResponse = serde_json::from_slice(&encrypt_body).unwrap();
        assert!(encrypt_result.success);

        // Decrypt request
        let decrypt_request = DecryptRequest {
            encrypted_data: encrypt_result.encrypted_data,
            secret_key: encoded_secret_key,
        };

        let decrypt_response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/decrypt")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&decrypt_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(decrypt_response.status(), StatusCode::OK);

        let decrypt_body = axum::body::to_bytes(decrypt_response.into_body(), usize::MAX).await.unwrap();
        let decrypt_result: DecryptResponse = serde_json::from_slice(&decrypt_body).unwrap();
        assert!(decrypt_result.success);

        // Verify decrypted data
        let decrypted_bytes = BASE64.decode(&decrypt_result.data).unwrap();
        let decrypted_text = String::from_utf8(decrypted_bytes).unwrap();
        assert_eq!(decrypted_text, test_data);
    }
}
