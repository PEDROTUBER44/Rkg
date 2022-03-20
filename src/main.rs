use serde::Deserialize; // Deserialize files
use std::{
    env, // Import standard collector command line arguments from user
    fs, // Import standard files and folders manipulation
    io, // Import standard IO
    io::Write, // // Import Write function from standard IO
    process::Command, // Invoke System Commands
    process::exit, // Import Exit Command
};
mod libs; // Import functions from libs.rs file


fn main() {
    let args: Vec<String> = env::args().collect();
    let option_one = &args[1].trim();
    let option_two = &args[2].trim();

    #[derive(Debug, Deserialize)]
    struct Config {
        data: DataConfig
    }

    #[derive(Debug, Deserialize)]
    struct DataConfig {
        name: String,
        architecture: String,
        version: String,
        docs: String,
        email: String,
        opensource: bool,
        main_maintainer: String,
        min_specs: bool,
        terminal: String,
        type_application: String,
        run_devices: String,
        donate_method: String,
        key_for_donate_method: String,
        site_of_project: String,
        git_of_project: String,
        md5: String,
        sha256: String,
        sha512: String,
        licence: String,
        size: String,
        age_group: u8,
        permission_required_hard_drive: bool,
        permission_required_home_directory: bool,
        permission_required_network: bool,
        permission_required_root_directory: bool,
        compilator: String,
        changelog: String,
        keyword_one: String,
        keyword_two: String,
        keyword_three: String,
        keyword_four: String,
        keyword_five: String,
        keyword_six: String,
        keyword_seven: String,
        keyword_eight: String,
        keyword_nine: String,
        keyword_ten: String,
        keyword_eleven: String,
        keyword_twelve: String,
        keyword_thirteen: String,
        keyword_fourteen: String,
        keyword_fifteen: String,
        keyword_sixteen: String,
        keyword_seventeen: String,
        keyword_eighteen: String,
        keyword_nineteen: String,
        keyword_twenty: String,
        description_en_us: String,
        description_es: String,
        description_pt_pt: String,
        description_pt_br: String,
    
    }

    match &option_one[..] {
        "install" => {
            let file = libs::path_exists(option_two);

            match file {
                true => {
                    
                    // Mount Local File in /tmp/rkg
                    Command::new("mount")
                        .args(Some("-o"))
                        .args(Some("loop"))
                        .args(Some(option_two))
                        .args(Some("/tmp/rkg/"))
                        .status()
                        .expect("Error to mount application");

                    // Variables
                    let rkg_directory: &str = &("/tmp/rkg/");
                    let app_data_directory: &str = &("/tmp/rkg/data/");
                    let config_file: &str = &fs::read_to_string("/tmp/rkg/data/config.toml").expect("Error accessing file");
                    let app_app_directory: &str = &("/tmp/rkg/app/");
                    let app_source_directory: &str = &("/tmp/rkg/source/");
                    let decoded: Config = toml::from_str(&config_file).unwrap();

                    println!("Name: {:#?}", decoded.data.name);
                    println!("");
                    println!("Version: {:#?}", decoded.data.version);
                    println!("");
                    println!("Description: {:#?}", decoded.data.description_en_us);
                    println!("");
                    println!("Permissions Requirements:");
                    println!("This application required permission to access you Hard Drive: {:#?}",  decoded.data.permission_required_hard_drive);
                    println!("This application required permission to access you Home Directory: {:#?}", decoded.data.permission_required_home_directory);
                    println!("This application required permission to access you Network: {:#?}",  decoded.data.permission_required_network);
                    println!("This application required permission to access you Root Directory: {:#?}", decoded.data.permission_required_root_directory);
                    println!("");
                    println!("Size: {:#?}", decoded.data.size);
                    println!("");
                    println!("");
                    print!("Continue installation? [Y/n]: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Error to read input for user");
                    let formated_input = input.trim();

                    match &formated_input[..] {
                        "n" => {
                            println!("Aborting the installation of {}!", option_two);
                            exit(0);
                        },

                        "N" => {
                            println!("Aborting the installation of {}!", option_two);
                            exit(0);
                        },

                        "" => {
                            /* install in /home/$USER/.apps/ */
                            /* add shortcut em /home/$USER/.bin/ */

                            Command::new("umount")
                                .args(Some("/tmp/rkg/"))
                                .status()
                                .expect("Error to umount app");
                        },

                        _ => {
                            println!("Aborting the installation of {}!", option_two);
                            exit(0);
                        }
                    }


                },

                false => {
                    println!("Download package from server");
                }
            }
        },

        "remove" => {
            println!("Remove function");
        },

        "info" => {
            println!("Print info of package");
        },

        "search" => {
            println!("Print search content")
        },

        _ => {
            println!("help");
        }
    }
}