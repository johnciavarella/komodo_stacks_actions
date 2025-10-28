# Komodo Stack Deployer

This repository provides a **GitHub Action and Rust utility** for automatically creating or updating **Docker stacks** in [Komodo](https://docs.rs/komodo_client/latest/komodo_client/api/write/index.html) via its API.

It supports:
- Creating or updating stacks by name
- Reading stack configuration from a `.toml` file
- Optional `.env` file support for Compose variables
- Environment-specific configurations (dev, staging, prod)
- Secure credentials passed via GitHub Action inputs

---

## Repository Structure

```
your-repo/
├── .github/
│   └── workflows/
│       └── deploy-komodo.yml          # GitHub Actions workflow
│
├── komodo/
│   ├── stack-config.toml              # Stack definition for Komodo
│   ├── stack.env                      # Default environment variables
│   └── environments/
│       ├── dev.env                    # Optional: dev-specific env file
│       ├── staging.env                # Optional: staging env file
│       └── prod.env                   # Optional: production env file
│
├── src/
│   └── main.rs                        # Rust deployer code
│
├── Cargo.toml                         # Rust project config
└── README.md
```

---

## How It Works

1. The GitHub Action runs when you push changes or manually trigger it.
2. It builds and runs the Rust-based **Komodo stack deployer**.
3. The deployer:
   - Loads configuration from `stack-config.toml`.
   - Loads environment variables from the file specified in the Action (`stack_env_path`).
   - Calls Komodo’s API to create or update the stack.

---

## GitHub Action Usage

The workflow file lives in:

```
.github/workflows/deploy-komodo.yml
```

### Example:

```yaml
name: Deploy to Komodo

on:
  workflow_dispatch:
  push:
    branches:
      - main

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Deploy stack to Komodo
        uses: ./
        with:
          komodo_url: ${{ secrets.KOMODO_URL }}
          api_token: ${{ secrets.KOMODO_API_TOKEN }}
          stack_name: "my_stack"
          stack_config_path: "./komodo/stack-config.toml"
          stack_env_path: "./komodo/environments/prod.env"
```

### Inputs

| Input Name         | Required | Description |
|--------------------|----------|--------------|
| `komodo_url`       | ✅ | Base URL of your Komodo instance (e.g. `https://komodo.local/api`) |
| `api_token`        | ✅ | API token for Komodo authentication |
| `stack_name`       | ✅ | Name of the stack to create or update |
| `stack_config_path`| ✅ | Path to the `stack-config.toml` file |
| `stack_env_path`   | ❌ | Optional path to `.env` file with environment variables |

---

## Stack Configuration Example

**`komodo/stack-config.toml`**

```toml
[stack]
project_name = "my_stack"
compose_path = "./docker-compose.yml"
repo_url = "https://github.com/your-org/your-repo.git"
branch = "main"
```

---

## Environment File Example

**`komodo/environments/prod.env`**

```
IMAGE_TAG=v1.2.0
DATABASE_URL=mysql://user:password@db:3306/app
REDIS_URL=redis://redis:6379
```

The variables in `.env` are automatically expanded into your Docker Compose file when Komodo deploys the stack.

---

## Local Testing

You can test the deployer locally before running in CI:

```bash
# Build the Rust deployer
cargo build --release

# Run it manually
./target/release/komodo-stack-deployer   --url https://komodo.local/api   --token <your_api_token>   --stack-name my_stack   --config ./komodo/stack-config.toml   --env ./komodo/environments/dev.env
```

---

## Implementation Notes

- The deployer uses the [`komodo_client`](https://docs.rs/komodo_client/latest/komodo_client/) crate for API access.
- It automatically finds an existing stack by name using `find_stack_id_by_name`.
- If the stack exists, it is updated; otherwise, a new one is created.
- `.env` variables are parsed into key-value pairs and merged into the stack definition.

---

## Security

- Your `KOMODO_URL` and `KOMODO_API_TOKEN` should be stored as [GitHub Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets).
- The `.env` files should never contain secrets directly in source control for production — prefer GitHub or Komodo’s own secret management where possible.

---

## License

MIT License © 2025 John Ciavarella

---

## Contributing

Pull requests are welcome!  
For major changes, please open an issue first to discuss what you’d like to modify.

---

## References

- [Komodo Client API Docs](https://docs.rs/komodo_client/latest/komodo_client/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Docker Compose Specification](https://docs.docker.com/compose/compose-file/)
