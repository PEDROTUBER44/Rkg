use std::process::Command;
use std::process::exit;
use std::env;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option_one = &args[1].trim();
    let option_two = &args[2].trim();

    let app_data_directory: &str = ("/mnt/" + option_two + "/data/");
    let app_app_directory: &str = ("/mnt/" + option_two + "/app/");
    let app_source_directory: &str = ("/mnt/" + option_two + "/source/");

    match &option_one[..] {

        "install" => {

            let file = lib::path_exists(option_two);
            
            match file {

                true => {
                    // Download Local File
                    Command::new("mount")
                        .args(Some("-o"))
                        .args(Some("loop"))
                        .args(Some(option_two))
                        .args(Some("/mnt"))
                        .status()
                        .expect("Error to mount app");

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
