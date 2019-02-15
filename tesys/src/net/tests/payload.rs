use crate::net::{Message,Payload,Route};
use crate::astrometry::frames::{FK5, ICRS};
use crate::astrometry::{Epoch, Frame, Location, ProperMotion, SkyCoordinate};

// This test is designed to check that we can return something of the correct type from a message payload. 
#[test]
fn message_get_payload() {
    let coord = SkyCoordinate::<ICRS>::new(279.23473479, 38.78368896).with_epoch(Epoch::j2000());

    let m = Message::new().to(Route::from_str("test-peer.test")).with_payload(coord.clone()).finish();
    let dat: SkyCoordinate<ICRS> = match m.get_payload() {
        Ok(p) => p,
        Err(_) => SkyCoordinate::<ICRS>::new(0., 0.),
    };

    assert_eq!(dat.ra().deg(), 279.23473479);
}

// This test is designed to see if our payload can correctly determine the difference between correct and incorrect types when unpacking a payload. 
#[test]
fn message_get_payload_wrong_type() {
    let coord = SkyCoordinate::<ICRS>::new(279.23473479, 38.78368896).with_epoch(Epoch::j2000());

    let m = Message::new().to(Route::from_str("test-peer.test")).with_payload(coord.clone()).finish();
    let dat: SkyCoordinate<FK5> = match m.get_payload() {
        Ok(p) => p,
        Err(_) => SkyCoordinate::<FK5>::new(0., 0.),
    };

    assert_eq!(dat.ra().deg(), 0.0);
}