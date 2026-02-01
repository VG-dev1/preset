use clap::{Parser, Subcommand};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use dirs_next::home_dir;
use std::path::PathBuf;
use std::process::{Command, exit};

type Presets = HashMap<String, Vec<String>>;

#[derive(Parser)]
#[command(name = "preset")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { name: String },

    Append {
        name: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    Insert {
        name: String,
        index: usize,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    Remove { name: String, command: String },

    Pop { name: String, index: usize },

    Delete { name: String },

    Run {
        name: String,
        #[arg(long)]
        skip_errors: bool,
        #[arg(long)]
        no_message: bool,
        #[arg(long)]
        dry_run: bool,
    },

    List,
}

fn get_storage_path() -> PathBuf {
    let home = home_dir().expect("Could not find home directory");
    home.join("presets.json")
}

fn setup_storage() {
    let file = get_storage_path();
    if !file.exists() {
        fs::write(&file, "{}").unwrap();
    }
}

fn load_data() -> Presets {
    let file = get_storage_path();
    let content = fs::read_to_string(file).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn save_data(data: &Presets) {
    let file = get_storage_path();
    let content = serde_json::to_string_pretty(data).unwrap();
    fs::write(file, content).unwrap();
}

fn main() {
    setup_storage();
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => {
            let mut data = load_data();
            if data.contains_key(&name) {
                println!("error: Preset '{}' already exists.", name);
            } else {
                data.insert(name.clone(), Vec::new());
                save_data(&data);
                println!("Preset '{}' created.", name);
            }
        }

        Commands::Append { name, command } => {
            let mut data = load_data();
            if let Some(list) = data.get_mut(&name) {
                let cmd = command.join(" ");
                list.push(cmd);
                save_data(&data);
                println!("Added to '{}'.", name);
            } else {
                println!("error: Invalid preset name.");
            }
        }

        Commands::Insert { name, index, command } => {
            let mut data = load_data();
            if let Some(list) = data.get_mut(&name) {
                if index <= list.len() {
                    let cmd = command.join(" ");
                    list.insert(index, cmd);
                    save_data(&data);
                    println!("Inserted to '{}'.", name);
                } else {
                    println!("error: Invalid index.");
                }
            } else {
                println!("error: Invalid preset name.");
            }
        }

        Commands::Remove { name, command } => {
            let mut data = load_data();
            if let Some(list) = data.get_mut(&name) {
                if let Some(pos) = list.iter().position(|x| x == &command) {
                    list.remove(pos);
                    save_data(&data);
                    println!("Removed command from '{}'.", name);
                } else {
                    println!("error: Command not found in preset.");
                }
            } else {
                println!("error: Invalid preset name.");
            }
        }

        Commands::Pop { name, index } => {
            let mut data = load_data();
            if let Some(list) = data.get_mut(&name) {
                if index < list.len() {
                    let removed = list.remove(index);
                    save_data(&data);
                    println!("Popped: {}", removed);
                } else {
                    println!("error: Invalid index.");
                }
            } else {
                println!("error: Invalid preset name.");
            }
        }

        Commands::Delete { name } => {
            let mut data = load_data();
            if data.remove(&name).is_some() {
                save_data(&data);
                println!("Deleted preset '{}'.", name);
            } else {
                println!("error: Invalid preset name.");
            }
        }

        Commands::Run { name, skip_errors, no_message, dry_run } => {
            let data = load_data();
            let commands = match data.get(&name) {
                Some(c) => c,
                None => {
                    println!("error: Invalid preset name.");
                    return;
                }
            };

            let mut success = 0;
            let mut fail = 0;
            let total = commands.len();

            for cmd_orig in commands {
                let mut cmd = cmd_orig.clone();

                while cmd.contains("{}") {
                    print!(
                        "{}",
                        format!("Enter value for the placeholder in '{}': ", cmd_orig).cyan()
                    );
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    cmd = cmd.replacen("{}", input.trim(), 1);
                }

                if !no_message {
                    if !dry_run {
                        println!("{}", format!("Executing: {}", cmd).bright_green());
                    } else {
                        println!("{}", format!("{} would be executed", cmd).cyan());
                    }
                }

                if !dry_run {
                    let status = if cfg!(target_os = "windows") {
                        Command::new("cmd").arg("/C").arg(&cmd).status()
                    } else {
                        Command::new("sh").arg("-c").arg(&cmd).status()
                    };

                    if status.is_err() || !status.as_ref().unwrap().success() {
                        fail += 1;
                        if !no_message {
                            if !skip_errors {
                                println!("{}", "error: Command failed. Stopping.".red());
                                println!(
                                    "{}",
                                    format!(
                                        "Execution partially completed: {} commands executed succesfully, {} commands failed.",
                                        success, fail
                                    )
                                    .yellow()
                                );
                                exit(1);
                            } else {
                                println!("{}", "warning: Command failed. Skipping...".yellow());
                            }
                        }
                    } else {
                        success += 1;
                    }

                    if success + fail == total && !no_message {
                        println!(
                            "{}",
                            format!(
                                "Execution completed: {} commands executed succesfully, {} commands failed.",
                                success, fail
                            )
                            .bright_green()
                        );
                    }
                }
            }
        }

        Commands::List => {
            let data = load_data();
            println!("{}", serde_json::to_string_pretty(&data).unwrap());
        }
    }
}
