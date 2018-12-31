
#[derive(Clone,Debug)]
pub struct Route {
	segments: Vec<String>,
	method: String,
}

impl Route {
	pub fn blank() -> Route {
		Route {
			segments: vec!(),
			method: String::new(),
		}
	}

	pub fn new() -> RouteBuilder {
		RouteBuilder::new()
	}
}

#[derive(Clone)]
pub struct RouteBuilder {
	_r: Route,
}

impl RouteBuilder {
	pub fn new() -> RouteBuilder{
		RouteBuilder {
			_r: Route::blank(),
		}
	}
}