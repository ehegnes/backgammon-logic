use std::fmt;
use std::num::ParseIntError;

use board::Position;
use constants::*;

#[derive(Debug, PartialEq)]
pub struct Submove {
    pub from: Position,
    pub to: Position,
}

impl Submove {
    pub fn new(from: Position, to: Position) -> Submove {
        Submove {
            from,
            to,
        }
    }
    /// Constructs a new submove from the supplied string.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(Submove::new("bar/20"), Submove { from: 25, to: 20 });
    /// assert_eq!(Submove::new("1/2"),    Submove { from: 1, to: 2 });
    /// ```
    pub fn parse(s: &str) -> Result<Submove, ParseIntError> {
        let s: Result<Vec<usize>, ParseIntError> =
            s.split('/')
            .map(|x| match x.as_ref() {
                "bar" => Ok(BAR_IDX),
                _ => x.parse(),
            })
            .collect::<Vec<Result<_, _>>>()
            .into_iter()
            .collect();

        match s {
            Ok(_) => Ok(Submove { from: s.clone()?[0], to: s?[1] }),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for Submove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.from, self.to)
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub submoves: Vec<Submove>,
}

impl Move {
    pub fn new(s: &str) -> Result<Move, ParseIntError> {
        let s: Result<Vec<Submove>, ParseIntError> =
            s.split_whitespace()
             .map(|x| Submove::parse(x))
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
        assert_eq!(Submove::parse("bar/2"), Ok(Submove::new(0, 2)));
        assert_eq!(Submove::parse("1/2"),   Ok(Submove::new(1, 2)));
    }

    #[test]
    fn parse_invalid_submoves() {
        let xs = vec![
            "",
            "/",
            "/0",
            "ba/20",
        ];

        for s in xs.iter() {
            assert!(Submove::parse(s).is_err());
        }
    }

    #[test]
    fn parse_valid_moves() {
        assert_eq!(
            Move::new("1/2 3/4"),
            Ok(Move { submoves: vec![
                Submove::new(1, 2),
                Submove::new(3, 4),
            ]}));
        assert_eq!(
            Move::new("bar/2"),
            Ok(Move { submoves: vec![
                Submove::new(0, 2),
            ]}));

    }

    #[test]
    fn parse_invalid_moves() {
        let xs = vec![
            //"", // FIXME: Invalidate the empty string case
            "asetuh",
            "/ /",
            "1/2 ba/20",
            "10/12 2/",
        ];

        for s in xs.iter() {
            assert!(Move::new(s).is_err());
        }
    }
}
