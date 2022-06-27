use serde::Deserialize; /* Importing library to deserialize files */
use colored::Colorize; /* Importing the library to customize the terminal fonts */
use std::{
    process::{
        exit, /* Function to exit the program */
        Command /* Function to invoke system commands */
    },
    io::{
        Write /* Importing standard input and output write function */
    },
    env, /* Importing the library to collect arguments passed by the user via the command line */
    fs, /* Importing the library to handle files */
    io, /* Importing the library for input and output */
};

fn main() {
    let args: Vec<String> = env::args().collect(); /* Collects all user-supplied arguments, and places them in an array of strings */
    let option = &args[1].trim().to_lowercase(); /* Collects the user's first argument */
    let package = &args[2].trim(); /* Collects the user's second argument */
    let noconfirm = &args[3].trim().to_lowercase(); /* Collects the user's third argument */

    #[derive(Debug, Deserialize)]
    struct Config {data: DataConfig}

    #[derive(Debug, Deserialize)]
    struct DataConfig {
        name: String,
        bin: String,
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

    pub fn path_exists(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }
    
    pub fn systemcommand_asuser(package: &str, command: &str, err: &str) {
        Command::new(package).args(command.split_ascii_whitespace()).status().expect(err);
    }
    
    pub fn systemcommand_asroot(command: &str, err: &str) {
        Command::new("sudo").args(command.split_ascii_whitespace()).status().expect(err);
    }

    match &option[..] {
        "install" => {
            let package_exists = path_exists(package);

            match package_exists {
                true => {
                    Command::new("mount").args(Some("-o")).args(Some("loop")).args(Some(package)).args(Some("/tmp/rkg/")).status().expect("Error to mount Rocket Package");
                    let config_file: &str = &fs::read_to_string("/tmp/rkg/data/config.toml").expect("Error to accessing the file"); /* Read the config.toml file to rocket package */
                    let decoded: Config = toml::from_str(&config_file).unwrap();

                    match decoded.data.type_application {
                        "utility" | "game" | "office" | "creative" => {

                            match &noconfirm[..] {
                                ""  => {
                                    loop {
                                        println!("{} {:#?}", "Name:".blue().bold(), decoded.data.name);
                                        println!("");
                                        println!("{} {:#?}", "Version:".blue().bold(), decoded.data.version);
                                        println!("");
                                        println!("{} {:#?}", "Description:".blue().bold(), decoded.data.description_en_us);
                                        println!("");
                                        println!("{}","Permissions Requirements:".yellow().bold());
                                        println!("This application required permission to access you {}: {:#?}", "Hard Drive".red().bold(), decoded.data.permission_required_hard_drive);
                                        println!("This application required permission to access you {}: {:#?}", "Home Directory".red().bold(), decoded.data.permission_required_home_directory);
                                        println!("This application required permission to access you {}: {:#?}", "Network".red().bold(), decoded.data.permission_required_network);
                                        println!("This application required permission to access you {}: {:#?}", "Root Directory".red().bold(), decoded.data.permission_required_root_directory);
                                        println!("");
                                        println!("{} {:#?}", "Size:".blue().bold(), decoded.data.size);
                                        println!("");
                                        println!("");
                                        print!("Do you want to continue? [{}/{}]: ","Y".green().bold(), "n".red().bold());
                                
                                        io::stdout().flush().unwrap();
                                        let mut input = String::new();
                                        io::stdin().read_line(&mut input).expect("Error to read user input");
                                        let input = input.trim().to_lowercase();
                                
                                        match &input[..] {
                                            "y" | "yes" | ""  => break,
                                            "n" | "no" => {println!("Aborted installation of {}", package.red().bold());exit(0);},
                                            _ => continue
                                        }
                                    }

                                    let user_app_directory: &str = &("/home/$USER/.apps/{}", decoded.data.name);
                                    let bin_location: &str = &("/home/$USER/.apps/{}/{}", decoded.data.name, decoded.data.bin);
                                    let bin_name: &str = &("/home/$USER/.bin/{}", decoded.data.bin);

                                    Command::new("mkdir").args(Some("-p")).args(Some("/tmp/rkg/")).args(Some(user_app_directory)).status().expect("Error to creating a folder for the application to be installed");
                                    Command::new("mkdir").args(Some("-p")).args(Some("/home/$USER/.apps/")).status().expect("Error to creating apps folder into a user home");
                                    Command::new("mv").args(Some("-r")).args(Some("/tmp/rkg/")).args(Some(user_app_directory)).status().expect("Error to move package files to user apps directory");
                                    Command::new("ln").args(Some("-s")).args(Some(bin_location)).args(Some(bin_name)).status().expect("Error to create symbolic link in .bin folder");
                                    systemcommand_asroot("umount /tmp/rkg/","Error to umount app");

                                },

                                "-y" | "--y" | "-yes" | "--yes" => {

                                },

                                _ => {
                                    println!("Invalid argument, try rkg help to see the list of possible commands");
                                }

                            }
                            
                        },

                        "codec" | "driver" | "font" => {

                        },

                        "mod" => {

                        },

                        _ => {

                        }
                    }
                },

                false => {
                    println!("Download package from server");
                }
            }
        },
        //"remove" => remove_package();
        //"update" => update_package();
        //"build" => build_package();
        //"help" => help();
        //"info" => info();
        //"search" => search();
        _ => {
            println!("Invalid argument, try rkg help to see the list of possible commands");
        }
    }
}