use std::fmt;

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
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub(crate) fn directions(&self) -> Vec<Direction> {
        use Difficulty::*;
        use Direction::*;

        match self {
            Easy => vec![Right, Down],
            Normal => vec![Right, Down, RightDown, RightUp],
            Hard => vec![Right, Down, RightDown, RightUp, LeftUp, LeftDown],
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Difficulty::*;
        write!(
            f,
            "{}",
            match self {
                Easy => "Easy - reading direction only",
                Normal => "Normal - reading direction and diagonal words",
                Hard => "Hard - all directions",
            }
        )
    }
}
