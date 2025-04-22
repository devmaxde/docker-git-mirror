use clap::{Parser, Subcommand};
use execute::Execute;
use serde::{Deserialize, Serialize};
use std::{
    env::{self, set_current_dir},
    fs,
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

pub fn build_command(command: impl ToString, append_command: Option<String>) -> Command {
    let command = command.to_string();
    println!("Building command: {} {:?}", command, append_command);

    let mut data = command.split_whitespace().collect::<Vec<&str>>();
    let mut command = if data.len() == 1 {
        Command::new(data[0])
    } else {
        let mut command = Command::new(data[0]);
        data.remove(0);
        for x in data {
            command.arg(x);
        }
        command
    };
    if let Some(append_command) = append_command {
        for arg in append_command.split_whitespace() {
            command.arg(arg);
        }
    }
    command
}
#[macro_export]
macro_rules! command_suc {
    ($cmd:expr, $append:expr) => {
        assert!(build_command($cmd, $append.clone())
            .execute_output()
            .unwrap()
            .status
            .success());
    };
    ($cmd:expr) => {
        command_suc!($cmd, None);
    };
}
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {},

    CreateConfig {},
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub repos: Vec<RepoConfig>,
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoConfig {
    pub ident: String,

    #[serde(default = "force_default")]
    pub force: bool,

    pub source_url: String,
    pub target_url: String,
}

fn force_default() -> bool {
    false
}

fn read_config() -> Config {
    let Ok(config_file) = fs::read_to_string("./config.toml") else {
        eprintln!("Failed to read config.toml");
        std::process::exit(1);
    };

    let config: Config = toml::from_str(&config_file).unwrap_or_else(|e| {
        eprintln!("Failed to parse the configuration file");
        eprintln!("{}", e);
        exit(1);
    });
    config
}

fn init_repo(repo: &RepoConfig) {
    let source_path = env::current_dir().unwrap();
    println!("{:?}", source_path);

    command_suc!(format!(
        "git clone --mirror {} {}",
        repo.source_url, repo.ident
    ));

    set_current_dir(source_path.clone().join(repo.ident.clone())).unwrap();
    command_suc!(format!(
        "git remote set-url --push origin {}",
        repo.target_url
    ));
    set_current_dir(source_path).unwrap();
}

fn sync_repo(repo: &RepoConfig) {
    let source_path = env::current_dir().unwrap();
    set_current_dir(source_path.clone().join(repo.ident.clone())).unwrap();
    command_suc!("git fetch -p origin");
    let tmp = if repo.force { "--force" } else { "" };
    command_suc!(format!("git push {} --mirror", tmp));
    set_current_dir(source_path).unwrap();
}

fn main() {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Run {}) {
        Commands::CreateConfig {} => {
            let config = Config {
                repos: vec![RepoConfig {
                    force: false,
                    ident: "my_project".to_string(),
                    source_url: "git@github.com:devmaxde/docker-git-mirror.git".to_string(),
                    target_url: "git@github.com:devmaxde/TestREPO.git".to_string(),
                }],
                timeout: 120,
            };

            let Ok(toml) = toml::to_string(&config) else {
                eprintln!("Failed to parse toml to string");
                exit(1);
            };

            let _ = fs::write("example_config.toml", toml);
        }
        Commands::Run {} => {
            let config: Config = read_config();

            for repo in config.repos.iter() {
                init_repo(repo);
            }

            loop {
                for repo in config.repos.iter() {
                    sync_repo(repo);
                }
                println!("Sleeping for {} seconds", config.timeout);
                sleep(Duration::from_secs(config.timeout));
            }
        }
    }
}
