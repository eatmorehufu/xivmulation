#[derive(Default)]
struct Gcd {
    expiration: SimTime,
    duration: SimTime,
}

impl Gcd {
    pub fn ready(&mut self, sim_time: SimTime) -> bool {
        self.expiration <= sim_time
    }

    pub fn start(&mut self, sim_time: SimTime) {
        self.expiration = sim_time + self.duration;
    }
}

#[cfg(tests)]
mod tests {

    macro_rules! test_gcd_ready {
        ($test_name:ident, $sim_time:expr, $expiration:expr, $expected:expr) => {
            #[test]
            fn $test_name() -> std::result::Result<(), String> {
                let mut gcd = Gcd {
                    expiration: $expiration,
                    duration: 2500,
                    ..Gcd::default()
                };
                assert_eq!($expected, gcd.ready($sim_time));
                Ok(())
            }
        };
    }
    test_gcd_ready!(ready, 1, 1, true);
    test_gcd_ready!(ready2, 2, 1, true);
    test_gcd_ready!(not_ready, 0, 1, false);
}
