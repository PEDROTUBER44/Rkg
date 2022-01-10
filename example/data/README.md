# Application variables and favicon.svg are here

Here are the package variables along with the program's favicon.svg.

****

## See below what each variable is for.

Note: all variables and their content must be written in lowcase.

****

### name = "application"
This variable serves to store the application name.

****

### architecture = "application architecture"
This variable serves to store the application's architecture, see the supported architectures below:

	architecture = "x86_64"
or

	architecture ="arm64"

or

	architecture = "aarch"

****

### version = "application version"
This variable serves to store the application version.

	version = "0.0.1"

****

### icon = "application icon"
This variable serves to store the application icon.
Note: The icon must be a .svg

	icon = "favicon.svg"

****

### docs = "application documentation website"
This variable serves to store the application documentation website

	docs = "https://www.rocketos/docs/rkg"

Note: If the project does not have a document site, the value must be "none"

	docs = "none"

****

### mantainers = "Main mantainers"
This variable serves to store the name of the main maintainers of the application the maximum number of names is 5 and minimum is 1.

	maintainers = ["Pedro Rosendo", "Kauan Costa"]	

****

### min_specs = "Minimum application requirements"
This variable stores the minimum specifications to run this application if it is false it means that this application does not have minimum requirements but if it is false it means that it does not run on any pc then the minimum and recommended requirements must be specified as shown below:

if false:

	min_specs = "false"

if true:

	min_specs = "true"

	[minimum-specs]
	CPU_INTEL_FULL_NAME = "Intel Celeron G530 2.4GHz"
	CPU_AMD_FULL_NAME = "Amd Ryzen 3 1200 3.1GHz"
	MEGABYTE_RAM = "512"
	RAM_FREQUENCY = "1066 mhz"
	GPU_AMD_FULL_NAME = "Amd Radeon R7"
	GPU_INTEL_FULL_NAME = "Intel HD Graphics 2000"
	GPU_NVIDIA_FULL_NAME = "Nvidia GT 710 1GB"

	[recommended-specs]
	CPU_INTEL_FULL_NAME = "Intel Celeron G530 2.4GHz"
	CPU_AMD_FULL_NAME = "Amd Ryzen 5 1600 3.2GHz"
	MEGABYTE_RAM = "2048"
	RAM_FREQUENCY = "1066 mhz"
	GPU_AMD_FULL_NAME = "Amd Radeon R7"
	GPU_INTEL_FULL_NAME = "Intel HD Graphics 2000"
	GPU_NVIDIA_FULL_NAME = "Nvidia GT 710 1GB"

****

### terminal = "Method of use application"
This variable is used to store whether the application works through the terminal or through the interface and can have 3 values, which are **true** to say that the application runs through the terminal or **false** to say that the application runs in graphical mode and **all** which means that the application runs through the terminal and the interface.

	terminal = "true"

or

	terminal = "false"

or

	terminal = "all"

****

### type_application = "Type of application"
This variable serves to store the type of application the values ​​can be:

Creative for creation apps like video editors, image editors... :

	type_application = "creative"

Utility for utility apps like file managers, compressed file managers... :

	type_application = "utility"

Driver for driver packages:

	type_application = "driver"

Game for games:

	type_application = "game"

Office for office applications like spreadsheet, presentation, and text editors:

	type_application = "office"

Codecs for video, audio codecs... :

	type_application = "codec"

****

### donate_method = "Donation method"
This variable serves to store the first donation transaction method. :

	donate_method = "pix"

If you don't have any, put it like this:

	donate_method = "none"

****

### other_donate_method = "Second donation method"
This variable serves to store the second donation transaction method. :

	other_donate_method = "paypal"

If you don't have any, put it like this:

	other_donate_method = "none"

****

### site_of_project = "Project Website"
This variable serves to store the project's website link:

	site_of_project = "https://www.rocketos/rkg"

Caso o projeto não tenha coloque assim:

	site_of_project = "none"

****

### git_of_project = "Project link in git"
This variable serves to store the project's git repository link:

	git_of_project = "https://www.github.com/PEDROTUBER44/rkg"

If the project is open source, this option must* contain the link to the project's git repository, but if it is closed source, the following value must be entered:

	git_of_project = "proprietary"

****

### checksum_rkg = "CheckSum of package"
This variable is used to store the package checksum.

	checksum_rkg [
		"55f921e91f1cc4aa9b4bbae8048d6911", # First md5
		"accb47520537bf91993696b4b3568c681925dac69e7764948e3c7070dd4db481", # Second sha256
		"397e2bedb3db7c0c5ddd06c12110a86829a1606838a86a62a9db2aa387ea62c870e751316992ae2475be1b4c7ecbdf9efe31d822917a6d086afbcec45b693b9f", # Third sha512
	]

Type the following command to verify the package checksum:

	md5sum rkg.rkg && sha256sum rkg.rkg && sha512sum rkg.rkg

****

### licence = "Package license"
This variable serves to store the package license.

	licence = "gplv3"

Supported licenses are:

	licence = "gplv2"

	licence = "gplv3"

	licence = "agpl"

	licence = "mit"

	licence = "apache-licencev2"

	licence = "creative-commons"

	licence = "mozila-public-licencev2"

****

### size = "size of package"
This variable serves to store the complete packet size in MB.

	size = "7MB"

To check the size of your package type the following command and then change it to MB and put it in the variable above:

	 du -shc rkg.rkg

****

### keywords = "Application Keywords"
This variable serves to store the application's keywords to facilitate the search for packages.
Note: the maximum keywords are 20 words and minimum is 6.

	keywords = [
		"Rkg", "rkg",
		"Rocket", "rocket",
		"Package", "package",
	]

****

### age_group = "Minimum age to use your app"
This variable stores the minimum age required to run your app.

	age_group = "14"

If you do not have a minimum age or the app can be used by all groups, please write this:

	age_group = "0"

****

### permissions_requirements = "Required permissions for running your app"
This variable serves to store all necessary permissions for the execution of your app.

	permissions_requirements = [
		"harddisk", "network",
		"home", "root",
	]

****

### [changelog]
This group of variables serves to store the package's change description history.

	version_0_0_1 = "Optimize performance and more"
	version_0_0_2 = "Bug fixes, Added ...."

****

### [build]
This group of variables serves to store the compiler needed to compile or run the application, it is mandatory if your app is open source.

	compilator = "cargo"

Supported compilers or runners:

* javac
* cargo
* gcc
* g++
* go
* python3

****

Example : https://github.com/PEDROTUBER44/rkg/blob/main/example/data/config.toml