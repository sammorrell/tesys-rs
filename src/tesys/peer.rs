use tesys::Loggable;

#[derive(Loggable)]
pub struct Peer {

}

impl Peer {
	pub fn new() -> Peer {
		Peer::log("Starting Peer...");
		Peer{
		}
	}
}
