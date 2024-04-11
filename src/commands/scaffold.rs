/// Meta scaffolding file for creating new commands
/// @note This command will not show in release builds and is only meant to help
///       create new commands in development.
///
///       Unless you want to change the way your commands are created, you can
///       ignore this file.
use std::io::{BufRead, Read, Seek, Write};

const TEMPLATES_FOLDER: &str = ".meta/templates";
const COMMANDS_FOLDER: &str = "src/commands";

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
}

const COMMAND_ABOUT: &str = "
Meta scaffolding command for creating new commands

The command will create a new command file in the commands directory, update 
the commands.rs file to include the new command, and make modifications to 
main.rs for you to start working with the command.";

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Meta scaffolding command for creating new commands")]
    #[clap(long_about = COMMAND_ABOUT)]
    Command { name: String },
}

#[derive(thiserror::Error, Debug)]
enum ScaffoldErrors {
    #[error("The command file already exists")]
    CommandFileExists,
    #[error("The template file is missing, check .meta/templates/command.rs")]
    MissingCommandTemplate,
}

pub(crate) fn run(args: &Arguments) -> crate::Result<()> {
    if let Some(command) = &args.command {
        match command {
            Commands::Command { name } => scaffold_command(name),
        }?;
    };

    Ok(())
}

fn scaffold_command(name: &str) -> crate::Result<()> {
    let name = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_end_matches(".rs")
        .to_owned();

    print!("🤞 Creating {name} command...");
    std::io::stdout().flush()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    let dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let dir = std::path::Path::new(&dir);
    let template = dir.join(format!("{TEMPLATES_FOLDER}/command.rs"));

    let mut file = match std::fs::File::open(template) {
        Ok(file) => file,
        Err(_) => return Err(ScaffoldErrors::MissingCommandTemplate.into()),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let target_path = dir.join(COMMANDS_FOLDER).join(format!("{name}.rs"));

    if std::path::Path::exists(&target_path) {
        print!("\r❌ Failed to create {name} command\n");
        std::io::stdout().flush()?;

        return Err(ScaffoldErrors::CommandFileExists.into());
    }

    std::fs::write(&target_path, contents)?;
    println!("\r✅ Created {name}.rs in {COMMANDS_FOLDER}!");
    std::io::stdout().flush()?;

    print!("🤞 Writing to commands.rs...");
    std::io::stdout().flush()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    // Append the new command to commands.rs
    // If this fails, that means that commands.rs doesn't exist and we have a bigger problem
    let commands_file_path = dir.join(COMMANDS_FOLDER).join("../commands.rs");
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open(commands_file_path)?;

    let mut last_byte: [u8; 1] = [0];
    if file.seek(std::io::SeekFrom::End(-1)).is_ok() {
        file.read_exact(&mut last_byte)?;
        if last_byte != [b'\n'] {
            file.write_all(b"\n")?;
        }
    }

    // We don't need to check if the mod statement this already exists in commands.rs
    // If the statement already exists but the file doesn't, the compiler will throw an error
    file.write_all(format!("pub(crate) mod {name};\n").as_bytes())?;
    println!("\r✅ Added {name} to commands.rs!");
    std::io::stdout().flush()?;

    print!("🤞 Writing to main.rs...");
    std::io::stdout().flush()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    let main_file_path = dir.join("src").join("main.rs");
    let main_file = std::fs::File::open(&main_file_path)?;
    let reader = std::io::BufReader::new(main_file);

    let output_file_path = dir.join("src").join("main.rs.tmp");
    let output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&output_file_path)?;
    let mut writer = std::io::BufWriter::new(output_file);

    let title_case_name = name
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect::<String>();

    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "{line}")?;

        if line.contains("Scaffold(scaffold::Arguments)") {
            writeln!(writer, "    {title_case_name}({name}::Arguments),")?;
        }

        if line.contains("Commands::Scaffold(args) => scaffold::run(args)") {
            writeln!(
                writer,
                "            Commands::{title_case_name}(args) => {name}::run(args),"
            )?;
        }
    }

    std::fs::rename(&output_file_path, &main_file_path)?;
    println!("\r✅ Added {name} to main.rs!");
    std::io::stdout().flush()?;

    println!("🎉 Command {name} created successfully!");
    Ok(())
}
