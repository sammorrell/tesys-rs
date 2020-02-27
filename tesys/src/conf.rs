use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub type PluginMap = BTreeMap<String, PluginConfigurationContainer>;

fn read_file(_conf_file: &str) -> String {
	let file = File::open(_conf_file)
		.expect("Could not open file.");
	let mut buffered_reader = BufReader::new(file);
	let mut contents = String::new();
	let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
		Ok(_number_of_bytes) => _number_of_bytes,
		Err(_err) => 0
	};

	contents
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TesysConfiguration {
    pub plugins: Option<PluginMap>,
}

impl TesysConfiguration {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TesysConfiguration { ..Default::default() }
    }

    pub fn from_file(conf_file: &str) -> Result<Self, serde_json::Error> {
        println!("Loading config: {}", conf_file);
        let contents = read_file(&conf_file);
        let conf: TesysConfiguration = serde_json::from_str(&contents)?;
        Ok(conf)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PluginConfigurationContainer {
    pub instanceof: String,

    #[serde(flatten)]
    pub config: BTreeMap<String, Value>,
}