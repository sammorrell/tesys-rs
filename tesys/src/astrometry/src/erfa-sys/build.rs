extern crate cc;
extern crate glob;
extern crate regex;

use std::env;
use std::fs::read_to_string;
use self::glob::glob;
use self::regex::Regex;

fn main() {
    let file_blacklist = vec!(["vendor/erfa/src/t_erfa_c.c"]); // Remove the test file, so we don't get linker errors of repeated main(). 
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap(); // Get the output directory. 

    let erfa_dir = "vendor/erfa";
    let erfa_src_dir = format!("{}/src", erfa_dir);
    let config_file = format!("{}/configure.ac", erfa_dir);
    let mut c_files: Vec<String> = Vec::<String>::new();

    // Get the files from the erfa directory and add them to the vector of C files. 
    let erfa_c_glob_pattern = format!("{}/*.c", erfa_src_dir);
    for e in glob(erfa_c_glob_pattern.as_str()).expect("Failed to get ERFA C files. ") {
        let file_str = format!("{}", e.unwrap().display());
        
        // Check for blacklisted files. If they aren't, add them to the build. 
        if !(file_blacklist.contains( &[file_str.as_str()] )) {
            c_files.push(file_str);
        }
    }

    // Read version numbers from the configure.ac
    let config = read_to_string(config_file).unwrap();
    let version_regex = Regex::new("ERFA_LIB_VERSION_INFO\\(([0-9]{1}),[ ]*([0-9]{1}),[ ]*([0-9]{1})\\)").unwrap();
    let sofa_version_regex = Regex::new("\\[SOFA_VERSION\\], \\[([\"0-9]+)\\]").unwrap();

    let mut version_major = String::new();
    let mut version_minor = String::new();
    let mut version_micro = String::new();
    let mut sofa_version = String::new();

    for cap in sofa_version_regex.captures_iter(config.as_str()) {
        sofa_version = cap[1].to_owned();
    }

    for cap in version_regex.captures_iter(config.as_str()) {
        version_major = cap[1].to_owned();
        version_minor = cap[2].to_owned();
        version_micro = cap[3].to_owned();
    }

    // Now we compile the C files into a library for Rust. 
    let package_version = format!("\"{}.{}.{}\"", version_major, version_minor, version_micro);
    cc::Build::new()
        .files(c_files)
        .define("PACKAGE_VERSION", Some(package_version.as_str()))
        .define("PACKAGE_VERSION_MAJOR", Some(format!("{}", version_major).as_str()))
        .define("PACKAGE_VERSION_MINOR", Some(format!("{}", version_minor).as_str()))
        .define("PACKAGE_VERSION_MICRO", Some(format!("{}", version_micro).as_str()))
        .define("SOFA_VERSION", Some(sofa_version.as_str()))
        .include(erfa_src_dir)
        .compile("erfa");
    
    // Setup the linker options
    println!("cargo:rustc-link-search=native={}", project_dir); 
    println!("cargo:rustc-link-lib=erfa"); 
}