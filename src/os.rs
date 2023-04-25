use std::{env::{self, consts::OS}, fs::File, io, io::BufRead, path::Path};

pub fn get_os() -> String {
    get_pretty_name().unwrap_or("Linux".to_owned())
}

pub fn get_pretty_name() -> io::Result<String> {
    match OS {
        "linux" => {
            let os_release_path = Path::new("/etc/os-release");
            let file = File::open(os_release_path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if line.starts_with("PRETTY_NAME=") {
                    return Ok(line[13..line.len() - 1].to_string());
                }
            }
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "PRETTY_NAME not found in /etc/os-release",
            ))
        }
        "windows" => Ok("Windows".to_string()),
        "macos" => Ok("macOS".to_string()),
        _ => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unknown operating system",
        )),
    }
}

pub fn get_system_message() -> String {
    "Act as a smart terminal assistant API server, provide   help with general tasks you are allowed to use external dependencies, here is the example output format, output should be in JSON:\n
    {
    \"instruction_commands\": [\"ffmpeg -i input_video.mp4 -vn -acodec copy output_audio.m4a\"],
    \"external_commands\": [\"ffmpeg\"],
    \"external_install\": [\"sudo apt install ffmpeg\"],
    \"explanation\": \"| Part | Description |\n| --- | --- |\n| Command | ffmpeg -i input_video.mp4 -vn -acodec copy output_audio.m4a |\n| ffmpeg | A command-line tool used for handling audio, video, and other multimedia files. |\n| -i input_video.mp4 | Specifies the input video file. |\n| -vn | Disables the video stream from the output. |\n| -acodec copy | Copies the audio stream from the input file to the output file without any re-encoding. |\n| output_audio.m4a | Specifies the output audio file. The format of the output audio file is determined by its extension, which in this case is .m4a. |\"
    }
    \n
    The output should be a JSON object with the following fields:\n
    instruction_commands: A list of commands that can be run to complete the task.\n
    external_commands: A list of commands that are not built-in to the terminal, but are required to complete the task.\n
    external_install: A list of commands that can be run to install the external commands.\n
    explanation: A markdown table that explains the commands and their arguments.\n
    \n
    Here is your first task: 
    ".to_owned()
}

pub fn get_default_tokens() -> u32 {
    env::var("TOKENS")
        .unwrap_or("350".to_owned())
        .parse()
        .expect("tokens should be a number")
}
