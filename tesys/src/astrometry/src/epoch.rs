extern crate chrono;
use std::ops::Add;
use std::ops::Sub;
use chrono::prelude::*;
use std::fmt;
use crate::datetime::*;

#[derive(Clone,Debug)]
pub struct Epoch {
	_ref: DateTime<Utc>
}

impl Epoch {
	pub fn new(_ref: DateTime<Utc>) -> Epoch{
		Epoch {
			_ref: _ref,
		}
	}

	pub fn now() -> Epoch {
		Epoch::new(Utc::now())
	}

	pub fn j2000() -> Epoch {
		Epoch::new(DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0), Utc))
	}

	pub fn epoch_date(&self) ->f64 {
		2000.0 + ( datetime_to_julian_date(self._ref) - 2451545.0 ) / 365.232
	}
}

impl Sub<Epoch> for Epoch {
	type Output = chrono::Duration;

	fn sub(self, rhs: Epoch) -> chrono::Duration {
		self._ref - rhs._ref
	}
}

impl Sub<chrono::Duration> for Epoch {
	type Output = Epoch;

	fn sub(self, rhs: chrono::Duration) -> Epoch {
		Epoch {
			_ref: self._ref - rhs
		}
	}
}

impl Add<Epoch> for Epoch {
	type Output = chrono::Duration;

	fn add(self, rhs: Epoch) -> chrono::Duration {
		self._ref - rhs._ref
	}
}

impl Add<chrono::Duration> for Epoch {
	type Output = Epoch;

	fn add(self, rhs: chrono::Duration) -> Epoch {
		Epoch {
			_ref: self._ref + rhs
		}
	}
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "J{:2.5}", self.epoch_date()
        )
    }
}
