use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

const INITIAL_BOARD: _Board = [ 
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, 0, 0
];
const BOARD_SIZE: usize = 26;
const BEAR_OFF_IDX: usize = 25;
const BAR_IDX: usize = 0;

type Position = usize;
type Chequer = i8;
type _Board = [Chequer; BOARD_SIZE];

pub struct Board {
    board: _Board,
}

impl Board {
    pub fn init() -> Board {
        Board {
            board: INITIAL_BOARD,
        }
    }

    pub fn apply_move(&self, _m: Move) -> Board {
        Board::init()
    }
}

#[derive(Debug, PartialEq)]
pub struct Submove {
    from: Position,
    to: Position,
}

impl Submove {
    /// Constructs a new submove from the supplied string.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(Submove::new("bar/20"), Submove { from: 25, to: 20 });
    /// assert_eq!(Submove::new("1/2"),    Submove { from: 1, to: 2 });
    /// ```
    pub fn new(s: String) -> Result<Submove, ParseIntError> {
        let s: Vec<usize> =
            s.split('/')
             .map(|x| match x.as_ref() {
                 "bar" => BAR_IDX,
                 _ => x.parse().unwrap(),
             })
             .collect();

        Ok(Submove {
            from: s[0],
            to: s[1],
        })
    }
}

impl fmt::Display for Submove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.from, self.to)
    }
}

pub struct Move {
    submoves: Vec<Submove>,
}

impl Move {
    pub fn new(s: String) -> Result<Move, Box<Error>> {
        Ok(Move {
            submoves: s.split_whitespace()
                       .map(|x| x.to_string())
                       .map(|x| Submove::new(x).unwrap())
                       .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * FIXME: Invalid test cases panic rather than returning a proper `Err`.
     */

    #[test]
    fn parse_valid_submoves() {
        assert_eq!(Submove::new("bar/20".to_string()), Ok(Submove { from: 0, to: 20 }));
        assert_eq!(Submove::new("1/2".to_string()),    Ok(Submove { from: 1, to: 2 }));
    }

    #[test]
    #[should_panic]
    fn parse_invalid_submoves() {
        let xs = vec![
            "",
            "/",
            "/0",
            "ba/20",
        ];

        for s in xs.iter() {
            assert!(Submove::new(s.to_string()).is_err());
        }
    }

    #[test]
    fn parse_valid_moves() {
        let xs = vec![
            "1/2 1/2",
            "1/2 bar/20",
            "10/12",
        ];

        for s in xs.iter() {
            assert!(Move::new(s.to_string()).is_ok());
        }

    }

    #[test]
    #[should_panic]
    fn parse_invalid_moves() {
        let xs = vec![
            "",
            "/ /",
            "1/2 ba/20",
            "10/12 2/",
        ];

        for s in xs.iter() {
            assert!(Move::new(s.to_string()).is_err());
        }
    }
}
