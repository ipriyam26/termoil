use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Instructions {
    pub instruction_commands: Vec<String>,
    pub external_commands: Vec<String>,
    pub external_install: Vec<String>,
    pub explanation: String,
}

pub fn handle_external_commands(command: &Instructions) {
    let mut found_one = false;
    // check the list of external commands and if any of them is not installed, print the list of commands to install them
    command
        .external_commands
        .iter()
        .enumerate()
        .for_each(|(index, tool)| {
            let output = Command::new("which").arg(tool.trim()).output();
            if let Ok(output) = output {
                if !output.status.success() {
                    if !found_one {
                        println!("Run the following commands to install the required tools:");
                        found_one = true;
                    }
                    println!(
                        "{}",
                        command
                            .external_install
                            .get(index)
                            .expect("Index out of bounds")
                    );
                }
            }
        });
}
