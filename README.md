# Docker Git Mirror

This repository hosts a Docker application designed to automate the mirroring of Git repositories. It's particularly useful for backing up repositories or ensuring that a synchronous copy of a repository is consistently available.

## Features

- **Automated Mirroring**: Set up once and have the application continuously mirror the repositories you specify.
- **Dockerized**: Runs in a Docker container, ensuring platform independence and ease of deployment.
- **Simple Configuration**: Utilizes a straightforward configuration file to set up source and target repositories.

## Getting Started

### Prerequisites

- Docker installed on your system. For installation instructions, visit [Docker's website](https://www.docker.com/get-started).
- A list of Git repositories you wish to mirror.

### Installation

1. **Clone this repository**:
   ```bash
   git clone https://github.com/devmaxde/docker-git-mirror.git
   ```
2. **Navigate to the project directory**:
   ```bash
   cd docker-git-mirror
   ```

### Usage

1. **Configure your repositories**:
   Create a `config.toml` file inside the `docker-config` directory. A Example config can look like this:

```toml
timeout = 120

[[repos]]
ident = "my_project"
source_url = "git@github.com:devmaxde/docker-git-mirror.git"
target_url = "git@github.com:devmaxde/TestREPO.git"
```

2. **Set up SSH keys and known hosts**:
   Place your SSH private key, public key, and known_hosts file in a config directory. Direct access is not recommended. You should also set the ssh-keys to read-only.

3. **Use Docker Compose to run the application**:
   Update the `docker-compose.yml` file as follows and then start the service:

   ```yaml
   version: "3"
   services:
     mirror:
       build: .
       volumes:
         - "./config/config.toml:/root/config.toml"
         - "./config/github:/root/.ssh/id_rsa:ro"
         - "./config/github.pub:/root/.ssh/id_rsa.pub:ro"
         - "./config/known_hosts:/root/.ssh/known_hosts"
   ```

   Run the Docker Compose command:

   ```bash
   docker-compose up -d
   ```

### Contributing

Contributions are welcome! Please fork the repository and submit pull requests with your proposed changes.
