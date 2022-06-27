# Instructions

Follow the instructions below to manually create a config.toml from your package.rkg:

First create the following tag at the beginning of the config.toml file:

    [data]

Now create a variable called "name" and give it the name of your app:

    name = "Rocket Package Manager"

Now create a variable called "architecture" containing the name of the architecture your app was built on. Values can be ("amd64", "arm64", "aarch", "all"):

    architecture = "amd64"

Now create a variable called "version" containing the version of your app containing only numbers and dots:

    version = "0.0.1"

Now create a variable called "docs" containing the link to your project's documentation. If the project does not have a documentation site, create a doc.md (Markdown) or doc.txt (Text) file in “package/docs/” and assign the value “null” to this variable:

    docs = "http://www.rocketos.com/docs/"

Now create a variable called "email" containing your project's contact email:

    email = "rocketos@protonmail.com"

Now create a variable called "opensource" to store whether your app is open source or proprietary. The values can be true or false, if true, the source folder must be created containing all the source code files needed to compile the package:

    opensource = true

Now create a variable called "main_maintainer" to store the full name of the overall project leader:

    main_maintainer = "Pedro Rosendo"

Now create a variable called "min_specs" to store if your app can be run on any pc or has minimum requirements if it has you must describe the minimum requirements. Values can be true or false:

    min_specs = false

Now create a variable called "terminal" to store whether your app is operated via command line, via GUI, or both. Values can be "terminal", "gui" or "all":

    terminal = "terminal"

Now create a variable called "type_application" to store the package type. Values can be "utility", "game", "office", "codec", "driver", "font", "creative", "mod":

    type_application = "utility"

Now create a variable called "run_devices" to store the type of device your app is designed for. The value can be "adaptative" if it runs on all devices, "mobile" for mobile devices, "television" for TV's and "desktop" for PC's:

    run_devices = "desktop"

Now create a variable called "donate_method" to store the accepted donation method type. The value can be "bitcoin", "pix" and "paypal":

    donate_method = "bitcoin"

Now create a variable called "key_for_donate_method" to store the name or code of the digital wallet:

    key_for_donate_method = "test123test123test123"

Now create a variable called "site_of_project" to store the project's website link. If not, set the value “null” to this variable:

    site_of_project = "https://www.rocketos.com"

Now create a variable called "git_of_project" to store the project's git repository link. If the project is closed source, assign this variable the value “null”:

    git_of_project = "https://github.com/PEDROTUBER44/Rkg"

Now create a variable called "id" to store the code of that specific version remember that this value can never be the same as some previous version:

    id = "dsd230mc932cds93nC0SAMRQ0vmewmf-sc9adewqtrby6urtsbf"

Now create a variable called "licence" to store the package license. The value can be gplv3, mit, mpl, gplv2, agpl, propritary...

    licence = "gplv3"

Now create a variable called "size" to store the total size of your package:

    size = "7MB"

Now create a variable called "age_group" to store the minimum age to install and use your app. The value can be 0 if you have no minimum age for installation and use and put 10, 12, 14, 16, 18, 21 to restrict for some audiences:

    age_group = 0

Now create a variable called "permission_required_hard_drive" to store if your app needs to modify hard drive partitions to use. The value can be true or false:

    permission_required_hard_drive = false

Now create a variable called "permission_required_home_directory" to store if your app needs to access files from the user's home. The value can be true or false:

    permission_required_home_directory = true

Now create a variable called "permission_required_network" to store if your app needs internet access. The value can be true or false:

    permission_required_network = true

Now create a variable called "permission_required_root_directory" to store whether your app needs to access files or folders from the system root. The value can be true or false:

    permission_required_root_directory = true

Now create a variable called "compiler" to store the name of the compiler needed by the user to compile your app. If it is an open source app or package and the language is interpreted, put the name of the interpreter and if it is a closed source package or app and but it has a compiler, assign this variable the value “null” but if it is a package or closed-source app but the language in which it was developed is interpreted, give this variable the name of the interpreter:

    compilador = "cargo"

Now create a variable called "changelog" to store the changes made against the last version. If it is the first version, put the value “null”:

    changelog = "null"

Now create a variable called "keyword_one" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_one = "rkg"

Now create a variable called "keyword_two" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_two = "rocket"

Now create a variable called "keyword_three" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_three = "rocket package"

Now create a variable called "keyword_four" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_four = "rkpk"

Now create a variable called "keyword_five" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_five = "manager"

Now create a variable called "keyword_six" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_six = "package"

Now create a variable called "keyword_seven" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_seven = "package manager"

Now create a variable called "keyword_eight" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_eight = "pkg"

Now create a variable called "keyword_nine" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_nine = "application"

Now create a variable called "keyword_ten" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_ten = "manager application"

Now create a variable called "keyword_eleven" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_eleven = "install"

Now create a variable called "keyword_twelve" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_twelve = "installation"

Now create a variable called "keyword_thirteen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_thirteen = "remove"

Now create a variable called "keyword_fourteen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_fourteen = "info" 

Now create a variable called "keyword_fifteen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_fifteen = "search application"

Now create a variable called "keyword_sixteen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_sixteen = "null"

Now create a variable called "keyword_seventeen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_seventeen = "null"

Now create a variable called "keyword_eighteen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_eighteen = "null"

Now create a variable called "keyword_nineteen" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_nineteen = "null"

Now create a variable called "keyword_twenty" to store the keywords used to search for your app in the search bar or in the rocket os store. If you have no idea what to put, give this variable the value "null":

    keyword_twenty = "null"

Now create a variable called "minimal_intel_cpu_required_name" if you have set the value true in "min_specs" set the name of the minimum intel cpu required for your app to work if the value has been false set the value "null":

    minimal_intel_cpu_required_name = "null"

Now create a variable called "minimal_amd_cpu_required_name" if you have set the value true in "min_specs" set the name of the minimum amd cpu required for your app to work if the value has been false set the value "null":

    minimal_amd_cpu_required_name = "null"

Now create a variable called "minimal_nvidia_gpu_required_name" if you have set the value true in "min_specs" set the name of the minimum nvidia gpu required for your app to work if the value has been false set the value "null":

    minimal_nvidia_gpu_required_name = "null"

Now create a variable called "minimal_amd_gpu_required_name" if you have assigned the value true in "min_specs" assign the name of the minimum amd gpu required for your app to work if the value has been false assign the value "null":

    minimal_amd_gpu_required_name = "null"

Now create a variable called "minimal_memory_ram_required" if you have set the value true in "min_specs" set the minimum amount of ram memory required for your app to work if the value has been false set the value "null":

    minimal_memory_ram_required = "null"

Now create a variable called "minimal_free_space_disk_required" if you have set the value true in "min_specs" set the amount of minimum disk space required for your app to work if the value has been false set the value "null":

    minimal_free_space_disk_required = "null"

Now create a variable called "recommended_intel_cpu_required_name" if you have given the value true in "min_specs" give the name of the recommended intel cpu needed for your app to work if the value has been false give the value "null":

    recommended_intel_cpu_required_name = "null"

Now create a variable called "recommended_amd_cpu_required_name" if you have given the value true in "min_specs" give the name of the recommended amd cpu needed for your app to work if the value has been false give the value "null":

    recommended_amd_cpu_required_name = "null"

Now create a variable called "recommended_nvidia_gpu_required_name" if you have given the value true in "min_specs" give the name of the recommended nvidia gpu needed for your app to work if the value has been false give the value "null":

    recommended_nvidia_gpu_required_name = "null"

Now create a variable called "recommended_amd_gpu_required_name" if you have given the value true in "min_specs" give the name of the recommended amd gpu needed for your app to work if the value has been false give the value "null":

    recommended_amd_gpu_required_name = "null"

Now create a variable called "recommended_memory_ram_required" if you have set the value true in "min_specs" set the recommended amount of ram memory needed for your app to work if the value has been false set the value "null":

    recommended_memory_ram_required = "null"

Now create a variable called "recommended_free_space_disk_required" if you have set the value true in "min_specs" set the amount of recommended disk space needed for your app to work if the value has been false set the value "null":

    recommended_free_space_disk_required = "null"

Now create a variable called "description_en_us" to store the description of your app or package in English (United States):

    description_en_us = "RKG or rocket package is a package manager written in rust with a focus on performance, security and scalability."

Now create a variable called "description_es" to store the description of your app or package in Spanish:

    description_es = "RKG o Rocket Package es un administrador de paquetes escrito en óxido con un enfoque en el rendimiento, la seguridad y la escalabilidad."

Now create a variable called "description_pt_pt" to store the description of your app or package in Portuguese (Portugal):

    description_pt_pt = "RKG ou rocket package é um gerenciador de pacotes escrito em rust com foco em performance, seguranca e escalabilidade."

Now create a variable called "description_pt_pt" to store the description of your app or package in Portuguese (Brazil):

    description_pt_br = "RKG ou rocket package é um gerenciador de pacotes escrito em rust com foco em performance, seguranca e escalabilidade."
