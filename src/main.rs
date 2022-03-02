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
        name: String, // Name of application or package
        architecture: String, // Architecture of compilated package or application
        version: String, // Version of application
        docs: String, // Project documentation website
        email: String, // Project contact email
        opensource: bool, // Here it will be stored whether or not the package is open source. Can have 2 values ​​being "true" and "false"
        main_maintainer: String, // Name of main maintainer or maintainers being divided if more than 1 by ","
        min_specs: bool, // If it is "true" it means that the application has minimum requirements to run, then it will be necessary to put the minimum and recommended requirements and if it is "false" it means that the application has no minimum requirements to run
        terminal: String, // If the value is "true" it means that the application is operated by the command line, if it is "false" it means that it is executed through the graphical interface and if it is operated by both ways the value must be "all"
        type_application: String, // This is where the package type is stored. This value can be "utility", "game", "office", "codec", "driver", "creative"
        run_devices: String, // Here it is stored on which devices your app is made to run. It can be "adaptative" if it runs on all devices, "mobile" if it runs only on mobile devices, "television" if it runs only on TV's and "desktop" if it only runs on desktops.
        donate_method: String, // Here the accepted donation type is stored. It can be "bitcoin", "pix", "paypal"
        key_for_donate_method: String, // Here is the key or name of the wallet for the donation deposit
        site_of_project: String, // Put the link to the project website here
        git_of_project: String, // Project git repository if open source.
        md5: String, // Here the md5 hash of package.rkg will be stored
        sha256: String, // Here the sha256 hash of package.rkg will be stored
        sha512: String, // Here the sha512 hash of package.rkg will be stored
        licence: String, // Put here licence of project: gplv3, mit, mpl ...
        size: String, // Put here total size of application or package
        age_group: u8, // Add here the minimum age range for installing and using your application, game or other. Put 0 if you don't have an age range and put 10, 12, 14, 16, 18 or 21 to restrict for some audiences
        permission_required_hard_drive: bool, // Say if your application or package needs to access the HD
        permission_required_home_directory: bool, // Say if your application or package needs to access the home directory
        permission_required_network: bool, // Say if your application or package needs to access the network
        permission_required_root_directory: bool, // Say if your application or package needs to access the root files and directories
        compilator: String, // Name of compilator of project: g++, gcc, cargo, go and other
        changelog: String, // Put the package changes from the previous version here
        keyword_one: String, // Here will be the 1* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_two: String, // Here will be the 2* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_three: String, // Here will be the 3* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_four: String, // Here will be the 4* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_five: String, // Here will be the 5* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_six: String, // Here will be the 6* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_seven: String, // Here will be the 7* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_eight: String, // Here will be the 8* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_nine: String, // Here will be the 9* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_ten: String, // Here will be the 10* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_eleven: String, // Here will be the 11* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_twelve: String, // Here will be the 12* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_thirteen: String, // Here will be the 13* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_fourteen: String, // Here will be the 14* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_fifteen: String, // Here will be the 15* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_sixteen: String, // Here will be the 16* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_seventeen: String, // Here will be the 17* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_eighteen: String, // Here will be the 18* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_nineteen: String, // Here will be the 19* keyword that will serve to find your application in the store or in the search bar of rocket os
        keyword_twenty: String, // Here will be the 20* keyword that will serve to find your application in the store or in the search bar of rocket os
        description_be: String, // Description of application or package in Belarusian
        description_bg: String, // Description of application or package in Bulgarian
        description_br: String, // Description of application or package in Sundanês
        description_bs: String, // Description of application or package in Croatian
        description_ca: String, // Description of application or package in Catalan
        description_cs: String, // Description of application or package in cs language
        description_da: String, // Description of application or package in da language
        description_de: String, // Description of application or package in de language
        description_dz: String, // Description of application or package in dz language
        description_el: String, // Description of application or package in el language
        description_eo: String, // Description of application or package in eo language
        description_es: String, // Description of application or package in es language
        description_et: String, // Description of application or package in et language
        description_eu: String, // Description of application or package in eu language
        description_fi: String, // Description of application or package in fi language
        description_fr: String, // Description of application or package in fr language
        description_gd: String, // Description of application or package in gd language
        description_gl: String, // Description of application or package in gl language
        description_gu: String, // Description of application or package in gu language
        description_hr: String, // Description of application or package in hr language
        description_hu: String, // Description of application or package in hu language
        description_id: String, // Description of application or package in id language
        description_is: String, // Description of application or package in is language
        description_it: String, // Description of application or package in it language
        description_ja: String, // Description of application or package in ja language
        description_kk: String, // Description of application or package in kk language
        description_km: String, // Description of application or package in km language
        description_kn: String, // Description of application or package in kn language
        description_ko: String, // Description of application or package in ko language
        description_lt: String, // Description of application or package in lt language
        description_lv: String, // Description of application or package in lv language
        description_mk: String, // Description of application or package in mk language
        description_mr: String, // Description of application or package in mr language
        description_my: String, // Description of application or package in my language
        description_nb: String, // Description of application or package in nb language
        description_ne: String, // Description of application or package in ne language
        description_nl: String, // Description of application or package in nl language
        description_nn: String, // Description of application or package in nn language
        description_oc: String, // Description of application or package in oc language
        description_pa: String, // Description of application or package in pa language
        description_pl: String, // Description of application or package in pl language
        description_pt: String, // Description of application or package Portuguese
        description_ro: String, // Description of application or package in ro language
        description_ru: String, // Description of application or package in ru language
        description_sk: String, // Description of application or package in sk language
        description_sl: String, // Description of application or package in sl language
        description_sr: String, // Description of application or package in sr language
        description_sv: String, // Description of application or package in sv language
        description_ta: String, // Description of application or package in ta language
        description_te: String, // Description of application or package in te language
        description_tr: String, // Description of application or package in tr language
        description_uk: String, // Description of application or package in uk language
        description_vi: String, // Description of application or package in vi language
        description_ar: String, // Description of application or package in ar language
        description_ast: String, // Description of application or package in ast language
        description_en_ca: String, // Description of application or package in en_ca language
        description_en_gb: String, // Description of application or package in en_gb language
        description_en_us: String, // Description of application or package in en_us language
        description_zh_cn: String, // Description of application or package in zh_cn language
        description_zh_hk: String, // Description of application or package in zh_hk language
        description_zh_tw: String, // Description of application or package in zh_tw language
        description_pt_br: String, // Description of application or package in Portuguese - Brazil
        description_sr_latin: String, // Description of application or package in sr_latin language
        description_ca_valencia: String, // Description of application or package in ca_valencia language
    
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
                    let config_file: &str = &fs::read_to_string("/tmp/rkg/data/config.toml").expect("Error to acess file");
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