use crate::bindings::{eraAtciq, eraAtciqz};
use crate::bindings::{eraASTROM, eraLDBODY};

pub unsafe fn atciq(_icrs: Vec<f64>, _pr: f64, _pd: f64, _px: f64, _rv: f64, _astrom: *mut eraASTROM) -> Vec<f64> {
    //! The wrapper function for eraAtciq, which does a quick conversion 
    //! between ICRS and CIRS. 
    
    let mut output: Vec<f64> = Vec::with_capacity(2);
    eraAtciq(_icrs[0], _icrs[1], _pr, _pd, _px, _rv, _astrom, &mut output[0], &mut output[1]);
    output
}

pub unsafe fn atciqz(_icrs: Vec<f64>, _astrom: *mut eraASTROM) -> Vec<f64> {
    //! The wrapper function for eraAtciq, which does a quick conversion 
    //! between ICRS and CIRS. This version assumes no parallax or proper motion. 
    
    let mut output: Vec<f64> = Vec::with_capacity(2);
    eraAtciqz(_icrs[0], _icrs[1], _astrom, &mut output[0], &mut output[1]);
    output
}
