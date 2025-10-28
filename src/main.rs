use anyhow::{Context, Result};
use komodo_client::{
    KomodoClient,
    api::{
        read::ListStacks,
        write::{CreateStack, UpdateStack},
    },
    entities::stack::StackConfig,
};
use std::{collections::HashMap, env, fs};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    let address = env::var("KOMODO_ADDRESS")?;
    let key = env::var("KOMODO_API_KEY")?;
    let secret = env::var("KOMODO_API_SECRET")?;
    let stack_name = env::var("STACK_NAME")?;
    let config_file_path = env::var("STACK_CONFIG_PATH").unwrap_or_else(|_| "./komodo/stack-config.toml".to_string());
    let env_file_path = env::var("STACK_ENV_PATH").unwrap_or_else(|_| "./komodo/stack.env".to_string());

    // Initialize Komodo client
    let client = KomodoClient::new(address, key, secret);

    // Load stack configuration
    let config_contents = fs::read_to_string(&config_file_path)
        .with_context(|| format!("Failed to read stack config file: {}", config_file_path))?;
    let mut stack_config: StackConfig = toml::from_str(&config_contents)
        .with_context(|| "Failed to parse TOML stack configuration")?;

    // Load stack.env if specified
    if let Ok(env_vars) = read_env_file(&env_file_path) {
        println!("Loaded {} environment variables from {}", env_vars.len(), env_file_path);
        if let Some(existing) = &mut stack_config.environment {
            existing.extend(env_vars);
        } else {
            stack_config.environment = Some(env_vars);
        }
    } else {
        println!("No stack.env file found at {}", env_file_path);
    }

    // Check if the stack already exists
    if let Some(stack_id) = find_stack_id_by_name(&client, &stack_name).await? {
        println!("Found existing stack: {} (id={})", stack_name, stack_id);

        let update = UpdateStack {
            id: stack_id.clone(),
            config: stack_config.clone(),
        };
        let resp = client.write(update).await?;
        println!("Updated stack response: {:?}", resp);
    } else {
        println!("Stack not found. Creating new stack: {}", stack_name);

        let create = CreateStack {
            name: stack_name.clone(),
            config: stack_config.clone(),
        };
        let resp = client.write(create).await?;
        println!("Created stack response: {:?}", resp);
    }

    Ok(())
}

/// Find a stack by its name using the Komodo read API
async fn find_stack_id_by_name(client: &KomodoClient, name: &str) -> Result<Option<String>> {
    let list_req = ListStacks {};
    let stacks = client.read(list_req).await?;

    for stack in stacks {
        if stack.name.eq_ignore_ascii_case(name) {
            return Ok(Some(stack.id.clone()));
        }
    }

    Ok(None)
}

/// Read a .env-style file into a HashMap<String, String>
fn read_env_file(path: &str) -> Result<HashMap<String, String>> {
    let contents = fs::read_to_string(path)?;
    let mut vars = HashMap::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            vars.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(vars)
}
