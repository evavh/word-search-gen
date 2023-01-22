use std::{collections::{HashMap, HashSet}, fmt};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(crate) enum Direction {
    Right,
    Left,
    Up,
    Down,
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(crate) enum Difficulty {
    Reading,
    Diagonal,
    Reverse,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Difficulty::*;
        write!(
            f,
            "{}",
            match self {
                Reading => "reading direction only",
                Diagonal => "reading direction and diagonal words",
                Reverse => "all directions",
            }
        )
    }
}

pub(crate) fn define_difficulty() -> HashMap<Difficulty, HashSet<Direction>> {
    use Difficulty::*;
    use Direction::*;

    HashMap::from([
        (Reading, HashSet::from([Right, Down])),
        (Diagonal, HashSet::from([Right, Down, RightDown, RightUp])),
        (
            Reverse,
            HashSet::from([
                Right, Left, Up, Down, RightUp, RightDown, LeftUp, LeftDown,
            ]),
        ),
    ])
}
