use crate::Plugin;
use crate::Exchange;

pub enum RunMode {
	Thread,
	Process,
}

const DEFAULT_RUN_MODE: RunMode = RunMode::Thread;

pub struct PluginHost {
	pg: Box<Plugin>,
	ex: Exchange,
	run_mode: RunMode
}

impl PluginHost {
	pub fn new(pg: Box<Plugin>) -> PluginHost {
		PluginHost {
			pg: pg,
			ex: Exchange::new(),
			run_mode: DEFAULT_RUN_MODE
		}
	}

	pub fn init() {

	}

	pub fn start() {

	}

	pub fn main() {

	}
}