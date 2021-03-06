// colfade.rs

extern crate lpfx;

use lpfx::launchpad::*;
use lpfx::utils::*;

fn main() {
    let mut lp = get_lp_from_name("Launchpad MIDI 1");

    play(&mut lp);
}


pub enum EightV {
    Vals(u8, u8, u8, u8, u8, u8, u8, u8),
}

fn shiftr(v: EightV) -> EightV {
    match v {
	EightV::Vals(a,b,c,d,e,f,g,h) => EightV::Vals(h,a,b,c,d,e,f,g),
    }
}

fn play(lp: &mut Launchpad) -> LPErr {

    lp.clear()?;

    let mut v = EightV::Vals(0, 1, 2, 3, 2, 1, 0, 0);

    loop {

	match v {
	    EightV::Vals(a,b,c,d,e,f,g,h) => {
		let v: Vec<u8> = vec![a,b,c,d,e,f,g,h];

		for i in 0..8 {
		    let vel = v[i as usize];
		    match vel {
			0 => { lp.column_off(i)?; },
			_ => { lp.column_on(i, vel)?; },
			
		    }
		}
	    },
	}

	sleep_millis(150);
	v = shiftr(v);
    }

    return Ok(());
}

// end colfade.rs
