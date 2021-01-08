/// Represents a monetary amount.
///
/// The inner value is the total number of copper coins.
/// In game this is typically instead displayed as a number
/// of gold, silver, and copper coins where every 100 copper coins is
/// one silver coin and every 100 silver coins is one gold coin.
///
/// The Display trait uses this in game format.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coins(i32);

impl Coins {
    /// The number of gold coins.
    pub fn gold(self) -> i32 {
        self.0 / 100_00
    }

    /// Creates an amount of currency from a number of gold coins.
    pub fn from_gold(gold: impl Into<i32>) -> Self {
        Coins(gold.into() * 1_00_00)
    }

    /// The number of silver coins.
    pub fn silver(self) -> i8 {
        ((self.0 / 100) % 100) as i8
    }

    /// Creates an amount of currency from a number of silver coins.
    pub fn from_silver(silver: impl Into<i32>) -> Self {
        Coins(silver.into() * 1_00)
    }

    /// The number of copper coins.
    pub fn copper(self) -> i8 {
        (self.0 % 100) as i8
    }

    /// Creates an amount of currency from a number of copper coins.
    pub fn from_copper(copper: impl Into<i32>) -> Self {
        Coins(copper.into())
    }
}

impl std::fmt::Display for Coins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match (self.gold(), self.silver()) {
            (0, 0) => f.write_fmt(format_args!("{}c", self.copper())),
            (0, _) => f.write_fmt(format_args!("{}s {}c", self.silver(), self.copper().abs())),
            (_, _) => f.write_fmt(format_args!(
                "{}g {}s {}c",
                self.gold(),
                self.silver().abs(),
                self.copper().abs()
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gold() {
        for i in 0..1000 {
            assert_eq!(i, Coins(i * 1_00_00).gold());
            assert_eq!(i, Coins(i * 1_00_00 + 99).gold());
            assert_eq!(i, Coins(i * 1_00_00 + 99_00).gold());

            assert_eq!(-1 * i, Coins(-1 * (i * 1_00_00)).gold());
            assert_eq!(-1 * i, Coins(-1 * (i * 1_00_00 + 99)).gold());
            assert_eq!(-1 * i, Coins(-1 * (i * 1_00_00 + 99_00)).gold());
        }
    }

    #[test]
    fn from_gold() {
        let coins = Coins::from_gold(999);
        assert_eq!(999_00_00, coins.0);
        assert_eq!(999, coins.gold());
    }

    #[test]
    fn silver() {
        for i in 0..100 {
            assert_eq!(i as i8, Coins(1_00 * i).silver());
            assert_eq!(i as i8, Coins(1_00 * i + 99).silver());
            assert_eq!(i as i8, Coins(1_00 * i + 99_00_00).silver());

            assert_eq!(-1 * i as i8, Coins(-1 * (1_00 * i)).silver());
            assert_eq!(-1 * i as i8, Coins(-1 * (1_00 * i + 99)).silver());
            assert_eq!(-1 * i as i8, Coins(-1 * (1_00 * i + 99_00_00)).silver());
        }
    }

    #[test]
    fn from_silver() {
        let coins = Coins::from_silver(99);
        assert_eq!(99_00, coins.0);
        assert_eq!(99, coins.silver());

        let coins = Coins::from_silver(1_00);
        assert_eq!(1_00_00, coins.0);
        assert_eq!(1, coins.gold());
        assert_eq!(0, coins.silver());
    }

    #[test]
    fn copper() {
        for i in 0..100 {
            assert_eq!(i as i8, Coins(i).copper());
            assert_eq!(i as i8, Coins(99_00 + i).copper());
            assert_eq!(i as i8, Coins(99_00_00 + i).copper());

            assert_eq!(-1 * i as i8, Coins(-1 * i).copper());
            assert_eq!(-1 * i as i8, Coins(-1 * (99_00 + i)).copper());
            assert_eq!(-1 * i as i8, Coins(-1 * (99_00_00 + i)).copper());
        }
    }

    #[test]
    fn from_copper() {
        let coins = Coins::from_copper(99);
        assert_eq!(99, coins.0);
        assert_eq!(99, coins.copper());

        let coins = Coins::from_copper(1_00);
        assert_eq!(1_00, coins.0);
        assert_eq!(1, coins.silver());
        assert_eq!(0, coins.copper());

        let coins = Coins::from_copper(1_00_00);
        assert_eq!(1_00_00, coins.0);
        assert_eq!(1, coins.gold());
        assert_eq!(0, coins.silver());
        assert_eq!(0, coins.copper());
    }

    /// Testing Display trait with values that is at least a gold.
    #[test]
    fn display_gold() {
        assert_eq!("1g 0s 0c", Coins(1_00_00).to_string());
        assert_eq!("1g 5s 0c", Coins(1_05_00).to_string());
        assert_eq!("1g 0s 5c", Coins(1_00_05).to_string());
        assert_eq!("1g 2s 3c", Coins(1_02_03).to_string());
        assert_eq!("1g 23s 45c", Coins(1_23_45).to_string());
        assert_eq!("123456g 78s 90c", Coins(123456_78_90).to_string());
        assert_eq!("-1g 2s 3c", Coins(-1_02_03).to_string());
    }

    /// Testing Display trait with values that are less
    /// than a gold and at least a silver.
    #[test]
    fn display_silver() {
        assert_eq!("1s 0c", Coins(1_00).to_string());
        assert_eq!("1s 5c", Coins(1_05).to_string());
        assert_eq!("1s 50c", Coins(1_50).to_string());
        assert_eq!("10s 0c", Coins(10_00).to_string());
        assert_eq!("98s 76c", Coins(98_76).to_string());
        assert_eq!("-1s 2c", Coins(-1_02).to_string());
    }

    /// Testing Display trait with values that are less than a silver.
    #[test]
    fn display_copper() {
        assert_eq!("0c", Coins(0).to_string());
        assert_eq!("1c", Coins(1).to_string());
        assert_eq!("10c", Coins(10).to_string());
        assert_eq!("99c", Coins(99).to_string());
        assert_eq!("-99c", Coins(-99).to_string());
    }
}
