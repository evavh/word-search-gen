use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug)]
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

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Difficulty {
    Reading,
    Diagonal,
    Reverse,
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
