#[derive(Clone, Debug)]
pub struct Route {
    segments: Vec<String>,
    method: String,
}

impl Route {
    pub fn blank() -> Route {
        Route {
            segments: vec![],
            method: String::new(),
        }
    }

    pub fn segment(&self, i: usize) -> Result<String, ()> {
        if self.segments.len() - 1 < i {
            Ok(self.segments[i].clone())
        } else {
            Err(())
        }
    }

    pub fn new() -> RouteBuilder {
        RouteBuilder::new()
    }

    pub fn from_str(route: &str) -> Route {
        RouteBuilder::new().segments_from_string(String::from(route)).finish()
    }
}

#[derive(Clone)]
pub struct RouteBuilder {
    _r: Route,
}

impl RouteBuilder {
    pub fn new() -> RouteBuilder {
        RouteBuilder { _r: Route::blank() }
    }

    pub fn segments_from_string(&mut self, route: String) -> RouteBuilder {
        // Lets split the route
        let segments = route.split(".").collect::<Vec<&str>>().iter().map(|s| { String::from(*s) } ).collect();
        self._r.segments = segments;

        self.clone()
    }

    pub fn finish(&self) -> Route {
        self._r.clone()
    }
}
