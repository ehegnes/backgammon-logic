use std::num::ParseIntError;
use std::str::FromStr;

use board::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum Submove {
    Move { from: Position, to: Position },
    Enter { to: Position },
    BearOff { from: Position },
}

/// Constructs a new submove from the supplied string.
///
/// **TODO:** Implement proper error handling with custom `Error` types.
///
/// # Caveats
/// Currently, this implementation only supports moves with a single `/`. That is to say, moves
/// in the format `1/2*/3` will fail to be parsed.
impl FromStr for Submove {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split('/').collect();

        match (s[0], s[1]) {
            ("bar", to) => {
                let to: Position = to.parse().unwrap();
                Ok(Submove::Enter { to: to - 1 })
            },
            (from, "o") => {
                let from: Position =  from.parse().unwrap();
                Ok(Submove::BearOff { from: from })
            },
            (from, to)  => {
                let from: Position =  from.parse().unwrap();
                let to: Position = to.parse().unwrap();
                Ok(Submove::Move { from: from - 1, to: to - 1 })
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub submoves: Vec<Submove>,
}

impl Move {
    pub fn new(s: &str) -> Result<Move, ParseIntError> {
        let s: Result<Vec<Submove>, ParseIntError> =
            s.split_whitespace()
             .map(|x| Submove::from_str(x))
             .collect::<Vec<Result<Submove, ParseIntError>>>()
             .into_iter()
             .collect();

        match s {
            Ok(_) => Ok(Move { submoves: s.unwrap() }),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_submoves() {
        assert_eq!(Submove::from_str("bar/2"), Ok(Submove::Enter { to: 1 }));
        assert_eq!(Submove::from_str("1/2"),   Ok(Submove::Move { from: 0, to: 1 }));
        assert_eq!(Submove::from_str("1/o"),   Ok(Submove::BearOff { from: 1 }));
    }

/*
 *    #[test]
 *    fn parse_invalid_submoves() {
 *        let xs = vec![
 *            "",
 *            "/",
 *            "/0",
 *            "ba/20",
 *        ];
 *
 *        for s in xs.iter() {
 *            assert!(Submove::from_str(s).is_err());
 *        }
 *    }
 */

    #[test]
    fn parse_valid_moves() {
        assert_eq!(
            Move::new("1/2 3/4"),
            Ok(Move { submoves: vec![
                Submove::from_str("1/2").unwrap(),
                Submove::from_str("3/4").unwrap(),
            ]}));
        assert_eq!(
            Move::new("bar/2"),
            Ok(Move { submoves: vec![
                Submove::from_str("bar/2").unwrap(),
            ]}));

    }

/*
 *    #[test]
 *    fn parse_invalid_moves() {
 *        let xs = vec![
 *            //"", // FIXME: Invalidate the empty string case
 *            "asetuh",
 *            "/ /",
 *            "1/2 ba/20",
 *            "10/12 2/",
 *        ];
 *
 *        for s in xs.iter() {
 *            assert!(Move::new(s).is_err());
 *        }
 *    }
 */
}
