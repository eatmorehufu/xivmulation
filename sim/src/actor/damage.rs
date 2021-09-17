#[derive(Default)]
pub struct Damage(i64);

impl Damage {
    pub fn add(&mut self, amount: i64) {
        println!("Damage dealt: {}", amount);
        self.0 += amount;
    }
}

#[cfg(tests)]
mod tests {
    macro_rules! test_add {
        ($test_name:ident, $sim_time:expr, $expiration:expr, $expected:expr) => {
            #[test]
            fn $test_name() -> std::result::Result<(), String> {
                let mut damage = Actor::default();
                assert_eq!($starting, damage.0);
                damage.add($damage);
                assert_eq!($expected, damage.0);
                Ok(())
            }
        };
    }

    test_add!(add, 0, 10, 10);
    test_add!(add, 5, 5, 10);
}
