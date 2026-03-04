use clap::{Parser, Subcommand};
use colored::Colorize;
use env_manager::{EnvFile, EnvManager, EnvType, EnvLinter, ReferenceChecker};
use std::collections::HashMap;
use std::path::PathBuf;
use env_manager::{EnvValidator, SecurityScanner};

#[derive(Parser)]
#[command(name = "env_manager")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "A powerful .env file management tool", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum BatchOps {
    /// Delete multiple variables
    Delete { keys: Vec<String>, #[arg(short, long, default_value = "local")] env: String },
    /// Update multiple variables  
    Update { updates: Vec<String>, #[arg(short, long, default_value = "local")] env: String },
    /// Rename variables
    Rename { old: String, new: String, #[arg(short, long, default_value = "local")] env: String },
}

#[derive(Subcommand)]
enum Commands {
    /// List all environment variables
    List {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Get a specific variable
    Get {
        key: String,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Set a variable
    Set {
        key: String,
        value: String,
        #[arg(short, long)]
        comment: Option<String>,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Remove a variable
    Remove {
        key: String,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Validate environment file
    Validate {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Check for broken references and circular dependencies
    Check {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Lint .env file for best practices
    Lint {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Backup environment file
    Backup {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Restore from backup
    Restore {
        #[arg(short, long)]
        list: bool,
        #[arg(short, long)]
        file: Option<String>,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Merge two environment files
    Merge {
        env1: String,
        env2: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        prefer_first: bool,
    },

    /// Expand variables
    Expand {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Show security report
    Security,

    /// Export environment to different formats
    Export {
        #[arg(short, long, default_value = "json")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Search for variables by keyword or pattern
    Search {
        query: String,
        #[arg(short, long)]
        key: bool,
        #[arg(short, long)]
        value: bool,
        #[arg(short, long)]
        regex: bool,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Format and organize .env file
    Format {
        #[arg(short, long)]
        sort: bool,
        #[arg(short, long)]
        group: bool,
        #[arg(short, long)]
        check: bool,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Create a new .env file template
    Init {
        #[arg(short, long, default_value = "local")]
        env_type: String,
    },

    /// Compare two environment files
    Diff {
        env1: String,
        env2: String,
    },

    /// Load and display all environment files
    All,

    /// Show statistics about environment variables
    Stats {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Show variable dependency tree
    Tree {
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Encrypt sensitive values
    Encrypt {
        key: String,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Decrypt values
    Decrypt {
        key: String,
        value: String,
        #[arg(short, long)]
        env: Option<String>,
    },

    /// Batch operations on multiple variables
    Batch {
        #[command(subcommand)]
        operation: BatchOps,
    },

    /// Test environment against schema
    Test {
        #[arg(short, long)]
        schema: Option<String>,
        #[arg(short, long, default_value = "local")]
        env: String,
    },

    /// Interactive mode
    Interactive,

    /// Set color theme
    Theme {
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Initialize with extended templates
    Template {
        #[arg(short, long)]
        kind: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red(), e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::List { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                println!("\n{}", format!("Environment Variables ({})", env).blue().bold());
                println!("{}\n", "=".repeat(50));

                for entry in env_file.entries.values() {
                    if let Some(ref comment) = entry.comment {
                        println!("  # {}", comment.dimmed());
                    }
                    println!("  {}={}", entry.key.green(), entry.value.yellow());
                }
                println!();
            } else {
                println!("{} No environment file found for '{}", "Warning:".yellow(), env);
            }
        }

        Commands::Get { key, env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                if let Some(value) = env_file.get(&key) {
                    println!("{}={}", key.green(), value.yellow());
                } else {
                    println!("{} Variable '{}' not found", "Error:".red(), key);
                }
            }
        }

        Commands::Set { key, value, comment, env } => {
            let env_type = parse_env_type(&env)?;
            let _manager = EnvManager::new(cli.path.clone());

            // Try to load existing file or create new one
            let mut env_file = match EnvFile::load(cli.path.join(env_type.to_filename())) {
                Ok(f) => f,
                Err(_) => {
                    let mut f = EnvFile::new(cli.path.join(env_type.to_filename()));
                    f.path = cli.path.join(env_type.to_filename());
                    f
                }
            };

            env_file.set(key.clone(), value.clone(), comment);
            env_file.save()?;

            println!("{} Set {}={}", "✓".green(), key.green(), value.yellow());
        }

        Commands::Remove { key, env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.environments.get_mut(&env_type) {
                if env_file.remove(&key).is_some() {
                    env_file.save()?;
                    println!("{} Removed {}", "✓".green(), key.red());
                } else {
                    println!("{} Variable '{}' not found", "Error:".red(), key);
                }
            }
        }

        Commands::Validate { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                let validator = EnvValidator::with_common_rules();

                match validator.validate(env_file) {
                    Ok(()) => {
                        println!("{} Validation passed!", "✓".green());
                        println!("\nValidated {} variables", env_file.entries.len());
                    }
                    Err(e) => {
                        println!("{} {}", "✗".red(), "Validation failed:".red());
                        println!("  {}", e);
                    }
                }
            }
        }

        Commands::Check { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                let report = ReferenceChecker::generate_reference_report(env_file);
                println!("{}", report);
            }
        }

        Commands::Lint { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                let report = EnvLinter::generate_lint_report(env_file);
                println!("{}", report);
            }
        }

        Commands::Backup { env } => {
            let env_type = parse_env_type(&env)?;
            let path = cli.path.join(env_type.to_filename());
            
            if !path.exists() {
                println!("{} No environment file found to backup", "Error:".red());
                return Ok(());
            }
            
            // Create backup directory if it doesn't exist
            let backup_dir = cli.path.join(".env_backups");
            if !backup_dir.exists() {
                std::fs::create_dir_all(&backup_dir)?;
            }
            
            // Create timestamped backup filename
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let backup_filename = format!("{}_{}.backup", env_type.to_filename().replace('.', "_"), timestamp);
            let backup_path = backup_dir.join(&backup_filename);
            
            // Copy the file
            std::fs::copy(&path, &backup_path)?;
            
            println!("{} Created backup: {}", "✓".green(), backup_path.display());
        }

        Commands::Restore { list, file, env } => {
            let env_type = parse_env_type(&env)?;
            let backup_dir = cli.path.join(".env_backups");
            
            if list {
                // List available backups
                if !backup_dir.exists() {
                    println!("{} No backups found", "Info:".blue());
                    return Ok(());
                }
                
                println!("\n{}", "📦 Available Backups".bold());
                println!("{}\n", "=".repeat(50));
                
                // Get all backup files  
                let mut backups: Vec<std::path::PathBuf> = Vec::new();
                
                match std::fs::read_dir(&backup_dir) {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.extension().is_some_and(|ext| ext == "backup") {
                                backups.push(path);
                            }
                        }
                    }
                    Err(e) => {
                        println!("{} Could not read backup directory: {}", "Error:".red(), e);
                        return Ok(());
                    }
                }
                
                backups.sort_by_key(|e| std::fs::metadata(e).ok().and_then(|m| m.modified().ok()).unwrap_or(std::time::SystemTime::UNIX_EPOCH));
                
                if backups.is_empty() {
                    println!("{} No backups found for '{}'", "Info:".blue(), env);
                } else {
                    let now = std::time::SystemTime::now();
                    for (i, backup) in backups.iter().enumerate() {
                        let metadata = std::fs::metadata(backup)?;
                        let modified = metadata.modified()?;
                        let elapsed = now.duration_since(modified).unwrap_or_default();
                        let total_secs = elapsed.as_secs();
                        let hours = total_secs / 3600;
                        let mins = (total_secs % 3600) / 60;
                        let secs = total_secs % 60;
                        
                        let name = backup.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "unknown".to_string());
                        
                        println!("  {}. {} ({:02}:{:02}:{:02} ago)", 
                            i + 1, 
                            name.yellow(),
                            hours, mins, secs
                        );
                    }
                }
            } else if let Some(backup_file) = file {
                // Restore from specific backup
                let backup_path = if std::path::Path::new(&backup_file).is_absolute() {
                    std::path::PathBuf::from(&backup_file)
                } else {
                    backup_dir.join(&backup_file)
                };
                
                if !backup_path.exists() {
                    println!("{} Backup file not found: {}", "Error:".red(), backup_path.display());
                    return Ok(());
                }
                
                let target_path = cli.path.join(env_type.to_filename());
                
                // Create a backup of current file before restoring
                if target_path.exists() {
                    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                    let pre_restore_backup = format!("{}_{}_pre_restore.backup", env_type.to_filename().replace('.', "_"), timestamp);
                    let pre_restore_path = backup_dir.join(&pre_restore_backup);
                    std::fs::copy(&target_path, &pre_restore_path)?;
                    println!("ℹ Created pre-restore backup: {}", pre_restore_path.display());
                }
                
                std::fs::copy(&backup_path, &target_path)?;
                println!("{} Restored from: {}", "✓".green(), backup_path.display());
            } else {
                println!("{} Use --list to see available backups or --file <filename> to restore", "Info:".blue());
            }
        }

        Commands::Merge { env1, env2, output, prefer_first } => {
            let env1_type = parse_env_type(&env1)?;
            let env2_type = parse_env_type(&env2)?;
            
            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_env(env1_type.clone())?;
            manager.load_env(env2_type.clone())?;
            
            let env1_file = manager.get_env(&env1_type).unwrap();
            let env2_file = manager.get_env(&env2_type).unwrap();
            
            // Create merged env file
            let mut merged = EnvFile::new(cli.path.join(output.as_deref().unwrap_or(".env.merged")));
            
            // Add all from env1
            for entry in env1_file.entries.values() {
                merged.set(entry.key.clone(), entry.value.clone(), entry.comment.clone());
            }
            
            // Add from env2 (respecting prefer_first flag)
            for entry in env2_file.entries.values() {
                if !merged.contains_key(&entry.key) || !prefer_first {
                    merged.set(entry.key.clone(), entry.value.clone(), entry.comment.clone());
                }
            }
            
            merged.save()?;
            println!("{} Merged {} and {} into {}", "✓".green(), env1, env2, merged.path.display());
        }

        Commands::Tree { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                println!("\n{}", "🌳 Variable Dependency Tree".bold());
                println!("{}\n", "=".repeat(50));
                
                let ref_regex = regex::Regex::new(r"\$\{([^}]+)\}|\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
                
                // Build dependency graph
                for (key, entry) in &env_file.entries {
                    let refs = ref_regex.find_iter(&entry.value)
                        .filter_map(|m| {
                            let text = m.as_str();
                            if text.starts_with("${") {
                                Some(text[2..text.len()-1].to_string())
                            } else if text.starts_with('$') {
                                Some(text[1..].to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    
                    if !refs.is_empty() {
                        println!("{} {}", key.green(), "→".yellow());
                        for ref_var in refs {
                            let exists = env_file.contains_key(&ref_var);
                            if exists {
                                println!("  └─ {} {}", "↳".blue(), ref_var.green());
                            } else {
                                println!("  └─ {} {} {}", "↳".blue(), ref_var.red(), "(missing)".yellow());
                            }
                        }
                        println!();
                    }
                }
            }
        }

        Commands::Encrypt { key, env } => {
            use sha2::{Sha256, Digest};
            use base64::{Engine as _, engine::general_purpose};
            
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.environments.get_mut(&env_type) {
                // Hash the key to get 32 bytes
                let mut hasher = Sha256::new();
                hasher.update(key.as_bytes());
                let hash = hasher.finalize();
                
                // Encrypt sensitive values using simple XOR (for demo - use libsodium in production)
                for (key_name, entry) in env_file.entries.iter_mut() {
                    let sensitive_patterns = ["password", "secret", "key", "token", "auth"];
                    if sensitive_patterns.iter().any(|p| key_name.to_lowercase().contains(p)) {
                        let data = entry.value.as_bytes();
                        let encrypted: Vec<u8> = data.iter()
                            .zip(hash.iter().cycle())
                            .map(|(&a, &b)| a ^ b)
                            .collect();
                        entry.value = format!("ENC:{}", general_purpose::STANDARD.encode(&encrypted));
                    }
                }
                
                env_file.save()?;
                println!("{} Encrypted sensitive values", "✓".green());
            }
        }

        Commands::Decrypt { key, value, env: _ } => {
            use sha2::{Sha256, Digest};
            use base64::{Engine as _, engine::general_purpose};
            
            if !value.starts_with("ENC:") {
                println!("{}", value);
                return Ok(());
            }
            
            let encoded = &value[4..];
            let encrypted = general_purpose::STANDARD.decode(encoded)
                .map_err(|_| "Invalid base64")?;
            
            // Hash the key
            let mut hasher = Sha256::new();
            hasher.update(key.as_bytes());
            let hash = hasher.finalize();
            
            // Decrypt
            let decrypted: Vec<u8> = encrypted.iter()
                .zip(hash.iter().cycle())
                .map(|(&a, &b)| a ^ b)
                .collect();
            
            let result = String::from_utf8_lossy(&decrypted);
            println!("{}", result.yellow());
        }

        Commands::Batch { operation } => {
            match operation {
                BatchOps::Delete { keys, env } => {
                    let env_type = parse_env_type(&env)?;
                    let mut manager = EnvManager::new(cli.path.clone());
                    manager.load_env(env_type.clone())?;

                    if let Some(env_file) = manager.environments.get_mut(&env_type) {
                        let mut deleted = 0;
                        for key in &keys {
                            if env_file.remove(key).is_some() {
                                deleted += 1;
                            }
                        }
                        env_file.save()?;
                        println!("{} Deleted {} variables", "✓".green(), deleted);
                    }
                }
                BatchOps::Update { updates, env } => {
                    let env_type = parse_env_type(&env)?;
                    let path = cli.path.join(env_type.to_filename());
                    
                    let mut env_file = match EnvFile::load(&path) {
                        Ok(f) => f,
                        Err(_) => {
                            let mut f = EnvFile::new(path.clone());
                            f.path = path;
                            f
                        }
                    };
                    
                    let mut updated = 0;
                    for update in &updates {
                        if let Some((key, value)) = update.split_once('=') {
                            env_file.set(key.to_string(), value.to_string(), None);
                            updated += 1;
                        }
                    }
                    
                    env_file.save()?;
                    println!("{} Updated {} variables", "✓".green(), updated);
                }
                BatchOps::Rename { old, new, env } => {
                    let env_type = parse_env_type(&env)?;
                    let mut manager = EnvManager::new(cli.path.clone());
                    manager.load_env(env_type.clone())?;

                    if let Some(env_file) = manager.environments.get_mut(&env_type) {
                        if let Some(entry) = env_file.entries.remove(&old) {
                            let mut new_entry = entry;
                            new_entry.key = new.clone();
                            env_file.entries.insert(new.clone(), new_entry);
                            env_file.save()?;
                            println!("{} Renamed {} → {}", "✓".green(), old, new);
                        } else {
                            println!("{} Variable '{}' not found", "Error:".red(), old);
                        }
                    }
                }
            }
        }

        Commands::Test { schema: _, env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                println!("\n{}", "🧪 Environment Test Report".bold());
                println!("{}\n", "=".repeat(50));
                
                let validator = EnvValidator::with_common_rules();
                let mut passed = 0;
                let mut failed = 0;
                
                // Run validation
                match validator.validate(env_file) {
                    Ok(_) => {
                        println!("{} Common validation rules", "✓".green());
                        passed += 1;
                    }
                    Err(e) => {
                        println!("{} Common validation: {}", "✗".red(), e);
                        failed += 1;
                    }
                }
                
                // Check for empty values
                let empty_count = env_file.entries.values().filter(|e| e.value.is_empty()).count();
                if empty_count == 0 {
                    println!("{} No empty values", "✓".green());
                    passed += 1;
                } else {
                    println!("{} {} empty value(s) found", "⚠".yellow(), empty_count);
                    failed += 1;
                }
                
                // Check for placeholders
                let placeholder_patterns = ["your_", "changeme", "xxx", "placeholder"];
                let has_placeholders = env_file.entries.values().any(|e| {
                    placeholder_patterns.iter().any(|p| e.value.to_lowercase().contains(p))
                });
                
                if !has_placeholders {
                    println!("{} No placeholder values", "✓".green());
                    passed += 1;
                } else {
                    println!("{} Placeholder values detected", "⚠".yellow());
                    failed += 1;
                }
                
                println!("\n{} Tests: {} passed, {} failed", "Σ".blue(), passed, failed);
            }
        }

        Commands::Interactive => {
            println!("\n{}", "🎯 Interactive Mode".bold());
            println!("{}\n", "=".repeat(50));
            println!("Select an action:");
            println!("1. List variables");
            println!("2. Get variable");
            println!("3. Set variable");
            println!("4. Remove variable");
            println!("5. Validate");
            println!("6. Stats");
            println!("7. Lint");
            println!("8. Exit");
            println!("\nEnter choice (1-8): ");
            
            // Simple interactive loop (in production, use a proper TUI library like ratatui)
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => println!("Run: env_manager list"),
                "2" => println!("Run: env_manager get <KEY>"),
                "3" => println!("Run: env_manager set <KEY> <VALUE>"),
                "4" => println!("Run: env_manager remove <KEY>"),
                "5" => println!("Run: env_manager validate"),
                "6" => println!("Run: env_manager stats"),
                "7" => println!("Run: env_manager lint"),
                "8" => println!("Exiting..."),
                _ => println!("Invalid choice"),
            }
        }

        Commands::Theme { name } => {
            if let Some(theme_name) = name {
                println!("{} Theme set to: {}", "✓".green(), theme_name);
                println!("Note: Full theme support requires additional configuration");
            } else {
                println!("\n{}", "🌈 Available Themes".bold());
                println!("{}\n", "=".repeat(50));
                println!("  - default (current)");
                println!("  - dark");
                println!("  - light");
                println!("  - high-contrast");
                println!("\nUse --name <theme> to set theme");
            }
        }

        Commands::Template { kind } => {
            let template_kind = kind.as_deref().unwrap_or("nodejs");
            let path = cli.path.join(".env");
            
            if path.exists() {
                println!("{} File already exists: {}", "Warning:".yellow(), path.display());
                return Ok(());
            }
            
            let mut env_file = EnvFile::new(path.clone());
            env_file.path = path;
            
            // Add templates based on kind
            match template_kind {
                "nodejs" | "javascript" => {
                    env_file.set("NODE_ENV".to_string(), "development".to_string(), None);
                    env_file.set("PORT".to_string(), "3000".to_string(), Some("Server port".to_string()));
                    env_file.set("DB_HOST".to_string(), "localhost".to_string(), Some("Database host".to_string()));
                    env_file.set("JWT_SECRET".to_string(), "your-secret-key".to_string(), Some("JWT signing secret".to_string()));
                }
                "python" | "django" => {
                    env_file.set("DJANGO_SETTINGS_MODULE".to_string(), "config.settings".to_string(), None);
                    env_file.set("DEBUG".to_string(), "True".to_string(), Some("Debug mode".to_string()));
                    env_file.set("SECRET_KEY".to_string(), "your-django-secret".to_string(), Some("Django secret key".to_string()));
                    env_file.set("DATABASE_URL".to_string(), "postgres://user:pass@localhost/db".to_string(), Some("Database URL".to_string()));
                }
                "docker" => {
                    env_file.set("DOCKER_HOST".to_string(), "tcp://localhost:2375".to_string(), None);
                    env_file.set("COMPOSE_PROJECT_NAME".to_string(), "myapp".to_string(), Some("Project name".to_string()));
                }
                "database" | "db" => {
                    env_file.set("DB_CONNECTION".to_string(), "mysql".to_string(), Some("Database type".to_string()));
                    env_file.set("DB_HOST".to_string(), "127.0.0.1".to_string(), None);
                    env_file.set("DB_PORT".to_string(), "3306".to_string(), None);
                    env_file.set("DB_DATABASE".to_string(), "laravel".to_string(), None);
                    env_file.set("DB_USERNAME".to_string(), "root".to_string(), None);
                    env_file.set("DB_PASSWORD".to_string(), "secret".to_string(), None);
                }
                _ => {
                    println!("{} Unknown template: {}. Using default.", "Warning:".yellow(), template_kind);
                    env_file.set("APP_ENV".to_string(), "local".to_string(), None);
                    env_file.set("APP_DEBUG".to_string(), "true".to_string(), None);
                    env_file.set("APP_KEY".to_string(), "base64:key-here".to_string(), None);
                }
            }
            
            env_file.save()?;
            println!("{} Created .env with {} template", "✓".green(), template_kind);
        }

        Commands::Expand { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.environments.get_mut(&env_type) {
                env_file.expand_variables()?;
                env_file.save()?;
                println!("{} Variables expanded successfully", "✓".green());
            }
        }

        Commands::Export { format, output, env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                let exported = export_env(env_file, &format)?;
                
                if let Some(output_path) = output {
                    std::fs::write(&output_path, &exported)?;
                    println!("{} Exported to {} in {} format", "✓".green(), output_path, format);
                } else {
                    println!("{}", exported);
                }
            }
        }

        Commands::Search { query, key, value, regex, env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                println!("\n{} Searching for '{}'", "🔍".blue(), query);
                println!("{}\n", "=".repeat(50));

                let mut found_count = 0;
                let search_pattern: Option<regex::Regex> = if regex {
                    Some(regex::Regex::new(&query).map_err(|e| format!("Invalid regex: {}", e))?)
                } else {
                    None
                };

                for entry in env_file.entries.values() {
                    let mut matches = false;

                    // Determine what to search in
                    let search_both = !key && !value; // Search both by default
                    let search_in_key = search_both || key;
                    let search_in_value = search_both || value;

                    if search_in_key {
                        if let Some(ref pattern) = search_pattern {
                            if pattern.is_match(&entry.key) {
                                matches = true;
                            }
                        } else if entry.key.to_lowercase().contains(&query.to_lowercase()) {
                            matches = true;
                        }
                    }

                    if search_in_value && !matches {
                        if let Some(ref pattern) = search_pattern {
                            if pattern.is_match(&entry.value) {
                                matches = true;
                            }
                        } else if entry.value.to_lowercase().contains(&query.to_lowercase()) {
                            matches = true;
                        }
                    }

                    if matches {
                        found_count += 1;
                        if let Some(ref comment) = entry.comment {
                            println!("  # {}", comment.dimmed());
                        }
                        println!("  {}={}", entry.key.green(), entry.value.yellow());
                        println!();
                    }
                }

                println!("{} Found {} match(es)", "Σ".blue(), found_count);
            }
        }

        Commands::Format { sort, group, check, env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.environments.get_mut(&env_type) {
                // Collect entries into a vector for sorting/grouping
                let mut entries: Vec<_> = env_file.entries.values().cloned().collect();

                if sort {
                    entries.sort_by(|a, b| a.key.cmp(&b.key));
                }

                if group {
                    // Group by category (DB_, API_, MAIL_, etc.)
                    entries.sort_by(|a, b| {
                        let prefix_a = a.key.split('_').next().unwrap_or("");
                        let prefix_b = b.key.split('_').next().unwrap_or("");
                        
                        if prefix_a == prefix_b {
                            a.key.cmp(&b.key)
                        } else {
                            prefix_a.cmp(prefix_b)
                        }
                    });
                }

                if check {
                    // Just check if formatting is needed, don't modify
                    let current_keys: Vec<_> = env_file.entries.keys().collect();
                    let formatted_keys: Vec<_> = entries.iter().map(|e| &e.key).collect();
                    
                    if current_keys == formatted_keys {
                        println!("{} File is already formatted", "✓".green());
                    } else {
                        println!("{} File needs formatting", "✗".yellow());
                        println!("  Run without --check to format");
                    }
                } else {
                    // Rebuild the env file with sorted entries
                    let mut new_entries = std::collections::HashMap::new();
                    for (i, mut entry) in entries.into_iter().enumerate() {
                        entry.line_number = i + 1;
                        new_entries.insert(entry.key.clone(), entry);
                    }
                    env_file.entries = new_entries;
                    env_file.save()?;
                    
                    println!("{} Formatted .env file", "✓".green());
                    if sort {
                        println!("  - Sorted alphabetically");
                    }
                    if group {
                        println!("  - Grouped by category");
                    }
                }
            }
        }

        Commands::Security => {
            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_all()?;

            println!("\n{}", "🔒 Security Report".bold());
            println!("{}\n", "=".repeat(50));

            for (env_type, env_file) in &manager.environments {
                let env_name = match env_type {
                    EnvType::Local => "local",
                    EnvType::Production => "production",
                    EnvType::Staging => "staging",
                    EnvType::Test => "test",
                    EnvType::Custom(name) => name.as_str(),
                };

                println!("\n{}", format!("Environment: {}", env_name).blue().bold());
                println!("{}", "-".repeat(30));
                let report = SecurityScanner::generate_security_report(env_file);
                println!("{}", report);
            }
        }

        Commands::Init { env_type } => {
            let env_t = parse_env_type(&env_type)?;
            let path = cli.path.join(env_t.to_filename());

            if path.exists() {
                println!("{} File already exists: {}", "Warning:".yellow(), path.display());
                return Ok(());
            }

            let mut env_file = EnvFile::new(path.clone());
            env_file.path = path;

            // Add template content based on type
            env_file.set("NODE_ENV".to_string(), get_default_env_value(&env_t), None);
            env_file.set(
                "PORT".to_string(),
                "3000".to_string(),
                Some("Application port".to_string()),
            );
            env_file.set(
                "DB_HOST".to_string(),
                "localhost".to_string(),
                Some("Database host".to_string()),
            );
            env_file.set(
                "DB_PORT".to_string(),
                "5432".to_string(),
                Some("Database port".to_string()),
            );
            env_file.set(
                "DB_USER".to_string(),
                "admin".to_string(),
                Some("Database user".to_string()),
            );
            env_file.set(
                "DB_PASSWORD".to_string(),
                "changeme".to_string(),
                Some("Database password - CHANGE THIS!".to_string()),
            );

            env_file.save()?;
            println!(
                "{} Created {} with template",
                "✓".green(),
                env_t.to_filename()
            );
        }

        Commands::Diff { env1, env2 } => {
            let env1_type = parse_env_type(&env1)?;
            let env2_type = parse_env_type(&env2)?;

            let mut manager = EnvManager::new(cli.path.clone());
            manager.load_env(env1_type.clone())?;
            manager.load_env(env2_type.clone())?;

            let env1_file = manager.get_env(&env1_type).unwrap();
            let env2_file = manager.get_env(&env2_type).unwrap();

            println!("\n{} vs {}\n", env1.blue(), env2.blue());
            println!("{}\n", "=".repeat(50));

            // Find differences
            let all_keys: std::collections::HashSet<_> = env1_file
                .keys()
                .into_iter()
                .chain(env2_file.keys())
                .collect();

            let mut only_in_env1 = Vec::new();
            let mut only_in_env2 = Vec::new();
            let mut different = Vec::new();
            let mut same = Vec::new();

            for key in all_keys {
                let val1 = env1_file.get(key);
                let val2 = env2_file.get(key);

                match (val1, val2) {
                    (Some(v1), Some(v2)) => {
                        if v1 == v2 {
                            same.push(key);
                        } else {
                            different.push(key);
                        }
                    }
                    (Some(_), None) => only_in_env1.push(key),
                    (None, Some(_)) => only_in_env2.push(key),
                    _ => {}
                }
            }

            if !only_in_env1.is_empty() {
                println!("\n{} Only in {}:", "→".yellow(), env1);
                for key in only_in_env1 {
                    println!("  + {}", key.green());
                }
            }

            if !only_in_env2.is_empty() {
                println!("\n{} Only in {}:", "→".yellow(), env2);
                for key in only_in_env2 {
                    println!("  + {}", key.green());
                }
            }

            if !different.is_empty() {
                println!("\n{} Different values:", "≠".yellow());
                for key in &different {
                    let v1 = env1_file.get(key).unwrap();
                    let v2 = env2_file.get(key).unwrap();
                    println!(
                        "  {}  {}:{} {}:{}",
                        "≠".yellow(),
                        key.green(),
                        v1.yellow(),
                        key.green(),
                        v2.yellow()
                    );
                }
            }

            println!(
                "\n{} {} identical, {} different",
                "Σ".blue(),
                same.len(),
                different.len()
            );
        }

        Commands::All => {
            let mut manager = EnvManager::new(cli.path);
            manager.load_all()?;

            println!("\n{}", "📦 All Environment Files".bold());
            println!("{}\n", "=".repeat(50));

            for (env_type, env_file) in &manager.environments {
                let env_name = match env_type {
                    EnvType::Local => "local",
                    EnvType::Production => "production",
                    EnvType::Staging => "staging",
                    EnvType::Test => "test",
                    EnvType::Custom(name) => name.as_str(),
                };

                println!(
                    "\n{}",
                    format!("📄 {} ({} vars)", env_name, env_file.entries.len())
                        .blue()
                        .bold()
                );
                println!("{}", "-".repeat(40));

                for entry in env_file.entries.values() {
                    println!("  {}={}", entry.key.green(), entry.value.yellow());
                }
            }

            if manager.environments.is_empty() {
                println!("{} No environment files found", "Info:".blue());
            }
        }

        Commands::Stats { env } => {
            let env_type = parse_env_type(&env)?;
            let mut manager = EnvManager::new(cli.path);
            manager.load_env(env_type.clone())?;

            if let Some(env_file) = manager.get_env(&env_type) {
                show_stats(env_file);
            } else {
                println!("{} No environment file found for '{}", "Warning:".yellow(), env);
            }
        }
    }

    Ok(())
}

/// Export environment to different formats
fn export_env(
    env_file: &EnvFile,
    format: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match format.to_lowercase().as_str() {
        "json" => {
            // Create a JSON object with key-value pairs
            let mut json_obj = serde_json::Map::new();
            for entry in env_file.entries.values() {
                json_obj.insert(entry.key.clone(), serde_json::Value::String(entry.value.clone()));
            }
            let json = serde_json::to_string_pretty(&json_obj)?;
            Ok(json)
        }

        "yaml" => {
            // Create YAML output using yaml-rust
            use yaml_rust::YamlEmitter;
            use yaml_rust::Yaml;
            
            let mut yaml_map = Vec::new();
            for entry in env_file.entries.values() {
                yaml_map.push((
                    Yaml::String(entry.key.clone()),
                    Yaml::String(entry.value.clone()),
                ));
            }
            
            let mut out_str = String::new();
            {
                let mut emitter = YamlEmitter::new(&mut out_str);
                emitter.dump(&Yaml::Hash(yaml_map.into_iter().collect()))?;
            }
            Ok(out_str)
        }

        "shell" | "bash" => {
            // Create shell export format
            let mut output = String::new();
            output.push_str("#!/bin/bash\n# Environment variables exported as shell script\n\n");
            
            for entry in env_file.entries.values() {
                // Escape special characters in values
                let escaped_value = entry.value.replace('\\', "\\\\").replace('"', "\\\"");
                output.push_str(&format!("export {}=\"{}\"\n", entry.key, escaped_value));
            }
            Ok(output)
        }

        "dotenv" | "env" => {
            // Standard .env format (already available via to_string)
            Ok(env_file.to_string())
        }

        _ => Err(format!("Unknown format: {}. Supported formats: json, yaml, shell, dotenv", format).into()),
    }
}

/// Show statistics about environment variables
fn show_stats(env_file: &EnvFile) {
    println!("\n{}", "📊 Environment Statistics".bold());
    println!("{}\n", "=".repeat(50));

    // Basic counts
    let total_vars = env_file.entries.len();
    let empty_values = env_file.entries.values().filter(|e| e.value.is_empty()).count();
    let with_comments = env_file.entries.values().filter(|e| e.comment.is_some()).count();

    // Value length stats
    let mut value_lengths: Vec<usize> = env_file.entries.values().map(|e| e.value.len()).collect();
    value_lengths.sort();
    
    let avg_length = if total_vars > 0 {
        value_lengths.iter().sum::<usize>() / total_vars
    } else {
        0
    };

    let median_length = if total_vars > 0 {
        let mid = total_vars / 2;
        if total_vars.is_multiple_of(2) {
            (value_lengths[mid - 1] + value_lengths[mid]) / 2
        } else {
            value_lengths[mid]
        }
    } else {
        0
    };

    // Category analysis (group by prefix like DB_, API_, etc.)
    let mut categories: HashMap<String, usize> = HashMap::new();
    for key in env_file.keys() {
        let prefix = key.split('_').next().unwrap_or("").to_string();
        *categories.entry(prefix).or_insert(0) += 1;
    }

    // Reference detection (variables that reference others)
    let ref_regex = regex::Regex::new(r"\$\{([^}]+)\}|\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let referencing = env_file.entries.values().filter(|e| ref_regex.is_match(&e.value)).count();

    // Print stats
    println!("{} Overview", "📈".blue());
    println!("  Total variables:      {}", total_vars);
    println!("  With comments:        {} ({:.1}%)", with_comments, if total_vars > 0 { (with_comments as f64 / total_vars as f64) * 100.0 } else { 0.0 });
    println!("  Empty values:         {} ({:.1}%)", empty_values, if total_vars > 0 { (empty_values as f64 / total_vars as f64) * 100.0 } else { 0.0 });
    println!("  With references:      {} ({:.1}%)", referencing, if total_vars > 0 { (referencing as f64 / total_vars as f64) * 100.0 } else { 0.0 });
    println!();

    println!("{} Value Length Analysis", "📏".blue());
    println!("  Average length:       {} chars", avg_length);
    println!("  Median length:        {} chars", median_length);
    println!("  Shortest value:       {} chars", value_lengths.first().unwrap_or(&0));
    println!("  Longest value:        {} chars", value_lengths.last().unwrap_or(&0));
    println!();

    if !categories.is_empty() {
        println!("{} Categories (by prefix)", "🗂️".blue());
        let mut cat_vec: Vec<_> = categories.iter().collect();
        cat_vec.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
        
        for (prefix, count) in cat_vec {
            let bar = "█".repeat(*count);
            println!("  {:15} {:3} {}", format!("{}_", prefix), count, bar.dimmed());
        }
        println!();
    }

    // Security insights
    let sensitive_patterns = [
        "password", "passwd", "pwd", "secret", "private", "token",
        "api_key", "apikey", "auth", "credential",
    ];
    let sensitive_count = env_file.entries.keys().filter(|k| {
        sensitive_patterns.iter().any(|p| k.to_lowercase().contains(p))
    }).count();

    if sensitive_count > 0 {
        println!("{} Security Insights", "🔒".blue());
        println!("  Sensitive variables:  {} (ensure these are protected!)", sensitive_count);
        println!();
    }
}

fn parse_env_type(env_str: &str) -> Result<EnvType, Box<dyn std::error::Error>> {
    match env_str.to_lowercase().as_str() {
        "local" | "dev" | "development" => Ok(EnvType::Local),
        "production" | "prod" => Ok(EnvType::Production),
        "staging" | "stage" => Ok(EnvType::Staging),
        "test" => Ok(EnvType::Test),
        other => Ok(EnvType::Custom(other.to_string())),
    }
}

fn get_default_env_value(env_type: &EnvType) -> String {
    match env_type {
        EnvType::Local => "development".to_string(),
        EnvType::Production => "production".to_string(),
        EnvType::Staging => "staging".to_string(),
        EnvType::Test => "test".to_string(),
        EnvType::Custom(_) => "development".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_type_local() {
        let result = parse_env_type("local");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Local);
    }

    #[test]
    fn test_parse_env_type_dev() {
        let result = parse_env_type("dev");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Local);
    }

    #[test]
    fn test_parse_env_type_development() {
        let result = parse_env_type("development");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Local);
    }

    #[test]
    fn test_parse_env_type_production() {
        let result = parse_env_type("production");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Production);
    }

    #[test]
    fn test_parse_env_type_prod() {
        let result = parse_env_type("prod");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Production);
    }

    #[test]
    fn test_parse_env_type_staging() {
        let result = parse_env_type("staging");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Staging);
    }

    #[test]
    fn test_parse_env_type_test() {
        let result = parse_env_type("test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EnvType::Test);
    }

    #[test]
    fn test_parse_env_type_custom() {
        let result = parse_env_type("my_custom_env");
        assert!(result.is_ok());
        match result.unwrap() {
            EnvType::Custom(name) => assert_eq!(name, "my_custom_env"),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn test_get_default_env_value_local() {
        let default = get_default_env_value(&EnvType::Local);
        assert_eq!(default, "development");
    }

    #[test]
    fn test_get_default_env_value_production() {
        let default = get_default_env_value(&EnvType::Production);
        assert_eq!(default, "production");
    }

    #[test]
    fn test_get_default_env_value_staging() {
        let default = get_default_env_value(&EnvType::Staging);
        assert_eq!(default, "staging");
    }

    #[test]
    fn test_get_default_env_value_test() {
        let default = get_default_env_value(&EnvType::Test);
        assert_eq!(default, "test");
    }

    #[test]
    fn test_get_default_env_value_custom() {
        let custom = EnvType::Custom("custom".to_string());
        let default = get_default_env_value(&custom);
        assert_eq!(default, "development");
    }

    #[test]
    fn test_cli_parser_basic() {
        // Test that CLI parser can be constructed
        let args = vec!["env_manager", "--path", ".", "list"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_parser_with_env() {
        let args = vec!["env_manager", "--path", "/tmp", "list", "--env", "production"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        assert_eq!(cli.path.to_str().unwrap(), "/tmp");
        
        if let Commands::List { env } = cli.command {
            assert_eq!(env, "production");
        } else {
            panic!("Expected List command");
        }
    }

    #[test]
    fn test_cli_set_command() {
        let args = vec![
            "env_manager",
            "set",
            "MY_KEY",
            "my_value",
            "--comment",
            "A test variable",
            "--env",
            "local"
        ];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Set { key, value, comment, env } = cli.command {
            assert_eq!(key, "MY_KEY");
            assert_eq!(value, "my_value");
            assert_eq!(comment, Some("A test variable".to_string()));
            assert_eq!(env, "local");
        } else {
            panic!("Expected Set command");
        }
    }

    #[test]
    fn test_cli_remove_command() {
        let args = vec!["env_manager", "remove", "MY_KEY", "--env", "staging"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Remove { key, env } = cli.command {
            assert_eq!(key, "MY_KEY");
            assert_eq!(env, "staging");
        } else {
            panic!("Expected Remove command");
        }
    }

    #[test]
    fn test_cli_validate_command() {
        let args = vec!["env_manager", "validate", "--env", "production"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Validate { env } = cli.command {
            assert_eq!(env, "production");
        } else {
            panic!("Expected Validate command");
        }
    }

    #[test]
    fn test_cli_backup_command() {
        let args = vec!["env_manager", "backup", "--env", "local"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Backup { env } = cli.command {
            assert_eq!(env, "local");
        } else {
            panic!("Expected Backup command");
        }
    }

    #[test]
    fn test_cli_search_command() {
        let args = vec!["env_manager", "search", "DATABASE", "--key", "--regex"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Search { query, key, value, regex, env } = cli.command {
            assert_eq!(query, "DATABASE");
            assert!(key);
            assert!(!value);
            assert!(regex);
            assert_eq!(env, "local");
        } else {
            panic!("Expected Search command");
        }
    }

    #[test]
    fn test_cli_format_command() {
        let args = vec!["env_manager", "format", "--sort", "--group"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Format { sort, group, check, env } = cli.command {
            assert!(sort);
            assert!(group);
            assert!(!check);
            assert_eq!(env, "local");
        } else {
            panic!("Expected Format command");
        }
    }

    #[test]
    fn test_cli_export_command() {
        let args = vec!["env_manager", "export", "--format", "json", "--env", "production"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Export { format, output, env } = cli.command {
            assert_eq!(format, "json");
            assert!(output.is_none());
            assert_eq!(env, "production");
        } else {
            panic!("Expected Export command");
        }
    }

    #[test]
    fn test_cli_diff_command() {
        let args = vec!["env_manager", "diff", ".env.local", ".env.production"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Diff { env1, env2 } = cli.command {
            assert_eq!(env1, ".env.local");
            assert_eq!(env2, ".env.production");
        } else {
            panic!("Expected Diff command");
        }
    }

    #[test]
    fn test_cli_batch_delete_command() {
        let args = vec!["env_manager", "batch", "delete", "KEY1", "KEY2", "--env", "local"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Batch { operation } = cli.command {
            if let BatchOps::Delete { keys, env } = operation {
                assert_eq!(keys, vec!["KEY1", "KEY2"]);
                assert_eq!(env, "local");
            } else {
                panic!("Expected Delete batch operation");
            }
        } else {
            panic!("Expected Batch command");
        }
    }

    #[test]
    fn test_cli_tree_command() {
        let args = vec!["env_manager", "tree", "--env", "staging"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Tree { env } = cli.command {
            assert_eq!(env, "staging");
        } else {
            panic!("Expected Tree command");
        }
    }

    #[test]
    fn test_cli_stats_command() {
        let args = vec!["env_manager", "stats", "--env", "production"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        let cli = result.unwrap();
        if let Commands::Stats { env } = cli.command {
            assert_eq!(env, "production");
        } else {
            panic!("Expected Stats command");
        }
    }
}
