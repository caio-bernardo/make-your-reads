use std::{fs::File, io::Write};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// overrides file with same output name
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// output file path.
    #[arg(short, long, value_name = "PATH")]
    path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new README file.
    #[command(arg_required_else_help = true)]
    Readme {
        /// project's name
        title: String,
    },
    /// Creates a new License file
    #[command(arg_required_else_help = true)]
    License {
        /// License's name
        #[arg(value_enum)]
        name: License,
    },
    /// Creates a new Code of Conduct file
    Coc,
    /// Creates a new Contributing file
    #[command(arg_required_else_help = true)]
    Contrib {
        /// project's name
        project_name: String,
        /// Contact Info for reporting issues
        #[arg(short, long, value_name = "CONTACT")]
        contact: Option<String>,
        /// URL for the documentation of the project
        #[arg(short, long, value_name = "URL")]
        documentation_url: Option<String>,
    },
    /// Creates a new Changelog file
    Changelog,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
enum License {
    MIT,
    CCBYSA4,
    CCBY4,
    GPL3,
    UNLICENSED,
}

const README_TEMPLATE: &'static str = include_str!("templates/README-template.md");
const COC_TEMPLATE: &'static str = include_str!("templates/COC-template.md");
const CONTRIBUTING_TEMPLATE: &'static str = include_str!("templates/CONTRIBUTING-template.md");
const CHANGELOG_TEMPLATE: &'static str = include_str!("templates/CHANGELOG-template.md");

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Readme { title }) => {
            let file = create_readme(&title);
            save_file(cli.path.as_deref().unwrap_or("README.md"), &file, cli.force);
            println!("New README created. Remember to change the file for your personal needs.")
        }
        Some(Commands::License { name }) => {
            let file_name = match name {
                License::MIT => "src/licenses/MIT",
                License::CCBYSA4 => "src/licenses/CC-BY-SA-4",
                License::CCBY4 => "src/licenses/CC-BY-4",
                License::GPL3 => "src/licenses/GPL-3",
                License::UNLICENSED => "src/licenses/UNLICENSED",
            };
            std::fs::copy(file_name, cli.path.as_deref().unwrap_or("LICENSE.md")).unwrap();
            println!("New LICENSE created. Remember to change the file for your personal needs.")
        }
        Some(Commands::Coc) => {
            let file = create_coc();
            save_file(
                cli.path.as_deref().unwrap_or("CODE_OF_CONDUCT.md"),
                &file,
                cli.force,
            );
            println!(
                "New CODE_OF_CONDUCT created. Remember to change the file for your personal needs."
            )
        }
        Some(Commands::Contrib {
            project_name,
            contact,
            documentation_url,
        }) => {
            let file = create_contrib(
                &project_name,
                contact.as_deref().unwrap_or(""),
                documentation_url.as_deref().unwrap_or(""),
            );
            save_file(
                cli.path.as_deref().unwrap_or("CONTRIBUTING.md"),
                &file,
                cli.force,
            );
            println!(
                "New CONTRIBUTING created. Remember to change the file for your personal needs."
            )
        }
        Some(Commands::Changelog) => {
            let file = create_changelog();
            save_file(
                cli.path.as_deref().unwrap_or("CHANGELOG.md"),
                &file,
                cli.force,
            );
            println!("New CHANGELOG created. Remember to change the file for your personal needs.")
        }
        None => println!("No command provided. See --help for more."),
    };
}

fn save_file(file_name: &str, content: &str, force: bool) {
    // Caso o arquivo exista ou eu não possa confirmar sua existência. Aborte.
    if !force && std::fs::exists(file_name).ok().is_none_or(|v| v) {
        println!(
            "File {} seems to already exists. Use --force to override it.",
            file_name
        )
    } else {
        let mut f = File::create(file_name).expect("Failed to created file");
        f.write_all(content.as_bytes())
            .expect("Failed to write file");
    }
}

fn create_readme(title: &str) -> String {
    README_TEMPLATE.replace("{{Title}}", title)
}

fn create_coc() -> String {
    COC_TEMPLATE.to_string()
}

fn create_contrib(project_name: &str, contact: &str, documentation_url: &str) -> String {
    CONTRIBUTING_TEMPLATE
        .replace("{{ProjectName}}", project_name)
        .replace("{{Contact}}", contact)
        .replace("{{DocumentationURL}}", documentation_url)
}

fn create_changelog() -> String {
    CHANGELOG_TEMPLATE.to_string()
}
