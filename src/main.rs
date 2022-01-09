use std::process::Command;
use std::process::exit;
use std::io::Write;
use std::env;
use std::io;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option_one = &args[1].trim();
    let option_two = &args[2].trim();

    let app_data_directory: &str = &(("/mnt/".to_owned() + option_two + "/data/"));
    let app_app_directory: &str = &(("/mnt/".to_owned() + option_two + "/app/"));
    let app_source_directory: &str = &(("/mnt/".to_owned() + option_two + "/source/"));

    match &option_one[..] {

        "install" => {

            let file = lib::path_exists(option_two);
            
            match file {

                true => {

                    // Read config.toml from data and copy name and version for printed for user
                    /* Code here */

                    print!("Do you want to proceed with the installation of {}? [Y/n]: ", option_two);
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

                        _ => {

                            // Mount Local File in /mnt/rkg
                            Command::new("mount")
                                .args(Some("-o"))
                                .args(Some("loop"))
                                .args(Some(option_two))
                                .args(Some("/mnt/rkg/"))
                                .status()
                                .expect("Error to mount app");

                        }
                    }

                    

                },

                false => {
                    // Download External File

                    println!("Printed");

                },

                _ => {
                    println!("{} not found", option_two);
                    exit(0);
                }
            }

        },

        "remove" => {

            println!("Printed");

        },

        "update" => {

            println!("Printed");

        },

        "config" => {

            println!("Printed");

        },

        "help" => {

            println!("Printed");

        },

        "info" => {

            println!("Printed");

        },

        _ => {
            println!("Error");
        }

    }
}
