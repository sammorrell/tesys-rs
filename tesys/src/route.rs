
pub struct Route {
	segments: Vec<String>,
	method: String,
}

impl Route {
	pub fn new() -> Route {
		Route {
			segments: vec!(),
			method: String::new(),
		}
	}

	pub fn build() -> RouteBuilder {
		RouteBuilder {}
	}
}

pub  struct RouteBuilder {

}