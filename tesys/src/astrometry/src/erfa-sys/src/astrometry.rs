use crate::bindings;
use crate::bindings::{eraASTROM};

/**
 * Preparation Functions
 */
pub unsafe fn apcs(_date1: f64, _date2: f64, _pv: &mut [[f64; 2usize]; 3usize], _ebpv: &mut [[f64; 2usize]; 3usize], _ehp: &mut [f64; 3usize]) -> eraASTROM {
    let mut _astrom = eraASTROM::new(); 
    let mut _astrom_ptr: *mut eraASTROM = &mut _astrom;
    bindings::eraApcs(_date1, _date2, _pv, _ebpv, _ehp, _astrom_ptr);
    _astrom
}

/**
 * ICRS -> CIRS Transforms
 */

pub unsafe fn atciq(_icrs: Vec<f64>, _pr: f64, _pd: f64, _px: f64, _rv: f64, _astrom: *mut eraASTROM) -> Vec<f64> {
    //! The wrapper function for eraAtciq, which does a quick conversion 
    //! between ICRS and CIRS. 
    
    let mut output: Vec<f64> = Vec::with_capacity(2);
    bindings::eraAtciq(_icrs[0], _icrs[1], _pr, _pd, _px, _rv, _astrom, &mut output[0], &mut output[1]);
    output
}

pub unsafe fn atciqz(_icrs: Vec<f64>, _astrom: *mut eraASTROM) -> Vec<f64> {
    //! The wrapper function for eraAtciq, which does a quick conversion 
    //! between ICRS and CIRS. This version assumes no parallax or proper motion. 
    
    let mut output: Vec<f64> = Vec::with_capacity(2);
    bindings::eraAtciqz(_icrs[0], _icrs[1], _astrom, &mut output[0], &mut output[1]);
    output
}
