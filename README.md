# ZS

A multiplexer and git session manager to improve your workflow.

## Usage

The command to use `zs` is.. `zs`.

`zs` uses a subcommand architecture, so every type of operation uses a subcommand.

To reduce the amount of typing required, every single subcommand (and its subcommands) also have a single character alias.

Example command invocations:

```bash
# Help
zs --help

# Open a session (default command)
zs
zs session
zs s # Same as above
zs session open
zs s o # Same as above

# Create a new workspace from a configured repo
zs workspace
zs w

# Create a new session from any directory
zs session new /path/to/project
zs s n /path/to/project

# Remove a session
zs session remove
zs session r
```


## Config
`zs` uses a TOML formatted config that's located at `~/.config/zs/zs.toml`.

### Example Config

```toml
# Multiplexer to use (currently only "zellij" is supported)
multiplexer = "zellij"

# Define your repositories
[repo.myproject]
type = "git"
path = "~/projects/myproject"
url = "git@github.com:user/myproject.git"
branch = "main"                    # Optional: specific branch to clone (default: repo default)
shallow = false                    # Optional: shallow clone (default: false)
submodules = true                  # Optional: initialize submodules (default: true)

# Optional: Hook script that runs after creating a workspace
workspace_hook = """
git submodule update --init --recursive
echo "Setting up environment..."
cat > .env << 'EOF'
DATABASE_URL=postgres://localhost/mydb
DEBUG=true
EOF
npm install
"""

[repo.another-project]
type = "git"
path = "~/work/another-project"
url = "https://github.com/org/another-project.git"
# Minimal config - uses defaults
```

### Config Options

#### Global Options

- **`multiplexer`** (required): The terminal multiplexer to use
  - Currently supported: `"zellij"`

#### Repository Configuration

Each repository is defined under `[repo.<name>]` where `<name>` is a unique identifier.

**Required fields:**

- **`type`**: The repository type
  - Currently supported: `"git"`

- **`path`**: Local path where the repository will be stored
  - Supports `~` expansion and environment variables
  - The primary clone will be at `<path>/primary`
  - Workspaces will be created at `<path>/workspaces/<workspace-name>`

- **`url`**: The remote repository URL
  - Supports SSH (`git@github.com:user/repo.git`)
  - Supports HTTPS (`https://github.com/user/repo.git`)

**Optional fields:**

- **`branch`**: Specific branch to clone (default: repository default branch)

- **`shallow`**: Enable shallow cloning with depth=1 (default: `false`)
  - Useful for large repositories to save space and time

- **`submodules`**: Initialize and update git submodules (default: `true`)

- **`workspace_hook`**: Shell script to execute after creating a workspace
  - Runs in the workspace directory
  - Use multiline strings for complex scripts
  - Useful for:
    - Cloning submodules
    - Creating configuration files
    - Installing dependencies
    - Running setup scripts
  - The hook is executed with `sh -c`
  - If the hook fails, workspace creation is aborted
