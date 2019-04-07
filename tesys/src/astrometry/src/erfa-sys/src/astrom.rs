use crate::bindings::eraASTROM;

impl eraASTROM {
    pub fn new() -> eraASTROM {
        eraASTROM {
            pmt: 0.0,
            eb: [0.0; 3usize],
            eh: [0.0; 3usize],
            em: 0.0,
            v: [0.0; 3usize],
            bm1: 0.0,
            bpn: [[0.0; 3usize]; 3usize],
            along: 0.0,
            phi: 0.0,
            xpl: 0.0,
            ypl: 0.0,
            sphi: 0.0,
            cphi: 0.0,
            diurab: 0.0,
            eral: 0.0,
            refa: 0.0,
            refb: 0.0,
        }
    }
}