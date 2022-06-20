use colored::Colorize; // Library to customize the terminal font
use serde::Deserialize; // Deserialize files
use std::{
    env, // Import standard collector command line arguments from user
    fs, // Import standard files and folders manipulation
    io, // Import standard IO
    io::Write, // // Import Write function from standard IO
    process::Command, // Invoke System Commands
    process::exit // Import Exit Command
};
mod utils;


fn main() {

    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();
    let package = &args[2].trim();
    let noconfirm = &args[3].trim();
    let rkg_directory: &str = &("/tmp/rkg/");
    let package_data_directory: &str = &("/tmp/rkg/data/");
    let package_directory: &str = &("/tmp/rkg/package/");
    let package_source_directory: &str = &("/tmp/rkg/source/");


    match &option[..] {
        "install" => {

            #[derive(Debug, Deserialize)]
            struct Config {
                data: DataConfig
            }
        
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
        
            let package_exists = utils::path_exists(package);
        
            match package_exists {
                true => {
                    // Mount Rocket Package in /tmp/rkg/
                    Command::new("mount").args(Some("-o")).args(Some("loop")).args(Some(package)).args(Some("/tmp/rkg/"))
                        .status()
                        .expect("Error to mount Rocket Package");
        
                    // Read config file and decodection
                    let config_file: &str = &fs::read_to_string("/tmp/rkg/data/config.toml").expect("Error accessing file");
                    let decoded: Config = toml::from_str(&config_file).unwrap();
        
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

                            Command::new("mkdir").args(Some("-p")).args(Some("/tmp/rkg/")).args(Some(user_app_directory))
                                .status()
                                .expect("Error to creating a folder for the application to be installed");

                            Command::new("mkdir").args(Some("-p")).args(Some("/home/$USER/.apps/"))
                                .status()
                                .expect("Error to creating apps folder into a user home");

                            Command::new("mv").args(Some("-r")).args(Some("/tmp/rkg/")).args(Some(user_app_directory))
                                .status()
                                .expect("Error to move package files to user apps directory");

                            Command::new("ln").args(Some("-s")).args(Some(bin_location)).args(Some(bin_name))
                                .status()
                                .expect("Error to create symbolic link in .bin folder");
                            
                            utils::systemcommand_asroot("umount /tmp/rkg/","Error to umount app");
                        },
        
                        "-y" => {
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
        
                            utils::systemcommand_asroot("umount /tmp/rkg/","Error to umount app");
                        },

                        _ => {
                            println!("Invalid Argument");
                        }
                    } 
        
                    
        
                },
        
                false => {
                    /*Implement download package from server function*/
                    println!("Download package from server");
                }
                
            }

        },
        //"remove" => utils::remove_package();
        //"update" => utils::update_package();
        //"build" => utils::build_package();
        //"help" => utils::help();
        //"info" => utils::info();
        //"search" => utils::search();
        _ => {
            println!("Ola");
        }
    }

}