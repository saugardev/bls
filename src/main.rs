use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{BufReader, BufWriter};
// Path operations handled by std::fs
use std::time::Instant;

mod encryption;
mod keys;
mod service;
mod types;

use encryption::BLSEncryption;
use keys::KeyManager;

#[derive(Parser)]
#[command(name = "bls-encryption-service")]
#[command(about = "A BLS-based encryption/decryption service")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new BLS keypair
    GenerateKeys {
        /// Output directory for keys
        #[arg(long, default_value = "./keys")]
        output: String,
        /// Name for the keypair files
        #[arg(long, default_value = "keypair")]
        name: String,
    },
    /// Encrypt a file or text
    Encrypt {
        /// Input file to encrypt
        #[arg(long)]
        input: Option<String>,
        /// Text to encrypt (alternative to input file)
        #[arg(long)]
        text: Option<String>,
        /// Public key file
        #[arg(long)]
        public_key: String,
        /// Output file for encrypted data
        #[arg(long)]
        output: String,
    },
    /// Decrypt a file
    Decrypt {
        /// Input encrypted file
        #[arg(long)]
        input: String,
        /// Secret key file
        #[arg(long)]
        secret_key: String,
        /// Output file for decrypted data
        #[arg(long)]
        output: String,
    },
    /// Run performance benchmarks
    Benchmark {
        /// Size in MB to benchmark
        #[arg(long, default_value = "10")]
        size_mb: usize,
    },
    /// Start the HTTP service
    Service {
        /// Port to listen on
        #[arg(long, default_value = "3000")]
        port: u16,
        /// Secret key file for the service
        #[arg(long)]
        secret_key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeys { output, name } => {
            generate_keys_command(&output, &name)?;
        }
        Commands::Encrypt {
            input,
            text,
            public_key,
            output,
        } => {
            encrypt_command(input, text, &public_key, &output)?;
        }
        Commands::Decrypt {
            input,
            secret_key,
            output,
        } => {
            decrypt_command(&input, &secret_key, &output)?;
        }
        Commands::Benchmark { size_mb } => {
            benchmark_command(size_mb)?;
        }
        Commands::Service { port, secret_key } => {
            service::start_service(port, secret_key).await?;
        }
    }

    Ok(())
}

fn generate_keys_command(output_dir: &str, name: &str) -> Result<()> {
    println!("Generating BLS keypair...");
    
    let start = Instant::now();
    let keypair = KeyManager::generate_keypair()?;
    let generation_time = start.elapsed();

    KeyManager::save_keypair_to_files(&keypair, output_dir, name)?;
    
    println!("✅ Keypair generated in {:?}", generation_time);
    println!("📋 Key ID: {}", KeyManager::generate_key_id(&keypair.public_key));
    
    Ok(())
}

fn encrypt_command(
    input_file: Option<String>,
    text: Option<String>,
    public_key_file: &str,
    output_file: &str,
) -> Result<()> {
    // Load public key
    let public_key_bytes = KeyManager::load_public_key_from_file(public_key_file)?;
    let encryption = BLSEncryption::new();

    let start = Instant::now();

    match (input_file, text) {
        (Some(input_path), None) => {
            // Encrypt file
            println!("🔐 Encrypting file: {}", input_path);
            
            let file_size = std::fs::metadata(&input_path)?.len();
            let pb = create_progress_bar(file_size, "Encrypting");

            let input_file = File::open(&input_path)?;
            let output_file = File::create(output_file)?;
            
            let reader = BufReader::new(input_file);
            let writer = BufWriter::new(output_file);

            let encrypted_data = encryption.encrypt_large(reader, writer, &public_key_bytes)?;
            pb.finish_with_message("✅ Encryption complete");

            println!("📊 File size: {} bytes", file_size);
            println!("📋 Data hash: {}", encrypted_data.data_hash);
            println!("🔑 Public key ID: {}", encrypted_data.public_key_id);
        }
        (None, Some(text_content)) => {
            // Encrypt text
            println!("🔐 Encrypting text...");
            
            let encrypted_text = encryption.encrypt_text(&text_content, &public_key_bytes)?;
            std::fs::write(output_file, encrypted_text)?;
            
            println!("✅ Text encrypted");
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Must provide either --input or --text, but not both"
            ));
        }
    }

    let encryption_time = start.elapsed();
    println!("⏱️  Encryption time: {:?}", encryption_time);
    println!("💾 Output saved to: {}", output_file);

    Ok(())
}

fn decrypt_command(input_file: &str, secret_key_file: &str, output_file: &str) -> Result<()> {
    println!("🔓 Decrypting file: {}", input_file);

    // Load secret key
    let secret_key_bytes = KeyManager::load_secret_key_from_file(secret_key_file)?;
    let encryption = BLSEncryption::new();

    let start = Instant::now();

    // Check if it's a text file (base64) or JSON file
    let input_content = std::fs::read_to_string(input_file);
    
    match input_content {
        Ok(text_content) if text_content.trim_start().starts_with('{') => {
            // JSON file (from encrypt_large)
            let file_size = std::fs::metadata(input_file)?.len();
            let pb = create_progress_bar(file_size, "Decrypting");

            let input_file_handle = File::open(input_file)?;
            let output_file_handle = File::create(output_file)?;
            
            let reader = BufReader::new(input_file_handle);
            let writer = BufWriter::new(output_file_handle);

            encryption.decrypt_large(reader, writer, &secret_key_bytes)?;
            pb.finish_with_message("✅ Decryption complete");
        }
        Ok(text_content) if text_content.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' || c.is_whitespace()) => {
            // Likely base64 encoded text
            let decrypted_text = encryption.decrypt_text(&text_content.trim(), &secret_key_bytes)?;
            std::fs::write(output_file, decrypted_text)?;
            println!("✅ Text decrypted");
        }
        _ => {
            // Binary file - shouldn't happen with our current implementation
            return Err(anyhow::anyhow!("Unsupported file format"));
        }
    }

    let decryption_time = start.elapsed();
    println!("⏱️  Decryption time: {:?}", decryption_time);
    println!("💾 Output saved to: {}", output_file);

    Ok(())
}

fn benchmark_command(size_mb: usize) -> Result<()> {
    println!("🚀 Running BLS encryption benchmark...");
    println!("📊 Test size: {} MB", size_mb);

    // Generate test data
    let data_size = size_mb * 1024 * 1024;
    println!("📝 Generating test data...");
    let test_data = vec![0x42u8; data_size];

    // Generate keypair
    println!("🔑 Generating keypair...");
    let start = Instant::now();
    let keypair = KeyManager::generate_keypair()?;
    let keygen_time = start.elapsed();
    println!("   Keypair generation: {:?}", keygen_time);

    let encryption = BLSEncryption::new();

    // Benchmark encryption
    println!("🔐 Benchmarking encryption...");
    let pb = create_progress_bar(data_size as u64, "Encrypting");
    let start = Instant::now();
    let encrypted_data = encryption.encrypt(&test_data, &keypair.public_key)?;
    let encrypt_time = start.elapsed();
    pb.finish_with_message("✅ Encryption complete");

    // Benchmark decryption
    println!("🔓 Benchmarking decryption...");
    let pb = create_progress_bar(data_size as u64, "Decrypting");
    let start = Instant::now();
    let decrypted_data = encryption.decrypt(&encrypted_data, &keypair.secret_key)?;
    let decrypt_time = start.elapsed();
    pb.finish_with_message("✅ Decryption complete");

    // Verify data integrity
    assert_eq!(test_data, decrypted_data);
    println!("✅ Data integrity verified");

    // Calculate throughput
    let encrypt_throughput = (data_size as f64) / encrypt_time.as_secs_f64() / (1024.0 * 1024.0);
    let decrypt_throughput = (data_size as f64) / decrypt_time.as_secs_f64() / (1024.0 * 1024.0);

    println!("\n📈 Benchmark Results:");
    println!("   Data size: {} MB", size_mb);
    println!("   Keypair generation: {:?}", keygen_time);
    println!("   Encryption time: {:?}", encrypt_time);
    println!("   Decryption time: {:?}", decrypt_time);
    println!("   Encryption throughput: {:.2} MB/s", encrypt_throughput);
    println!("   Decryption throughput: {:.2} MB/s", decrypt_throughput);
    println!("   Encrypted size: {} bytes", encrypted_data.encrypted_content.len());
    println!("   Compression ratio: {:.2}%", 
        (encrypted_data.encrypted_content.len() as f64 / data_size as f64) * 100.0);

    Ok(())
}

fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(message.to_string());
    pb
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_keys_command() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().to_str().unwrap();
        
        generate_keys_command(output_dir, "test").unwrap();
        
        // Check that files were created
        let secret_path = format!("{}/test_secret.key", output_dir);
        let public_path = format!("{}/test_public.key", output_dir);
        
        assert!(std::path::Path::new(&secret_path).exists());
        assert!(std::path::Path::new(&public_path).exists());
    }
}
