use frame::Frame;

#[derive(Clone)]
pub struct FK5 {
	
}


impl fmt::Display for SkyCoordinate<FK5> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ra_hms = self.coords[0].to_hms();
        let dec_dms = self.coords[1].to_dms();
        let ra_neg_str = if ra_hms.neg { "-" } else { "" };
        let dec_neg_str = if dec_dms.neg { "-" } else { "" };
        write!(
            f,
            "(FK5: {}): RA = {}{:0>2}:{:0>2}:{:2.2}, DEC = {}{:0>2}:{:0>2}:{:2.2}",
            self.epoch(), ra_neg_str, ra_hms.h, ra_hms.m, ra_hms.s, dec_neg_str, dec_dms.d, dec_dms.m, dec_dms.s
        )
    }
}
impl Frame for FK5 {
	type Frame = FK5;

	fn new() -> Self::Frame {
		FK5 {
			
		}
	}
}