pub use geom::Geom;
pub use line::Line;
pub use point::Point;
pub use rect::Rect;
pub use ring::Ring;
pub use segment::Segment;
pub use visitors::{SearchVisitor,NearestSegmentVisitor};
mod geom;
mod line;
mod point;
mod poly;
mod rect;
mod ring;
mod segment;
mod visitors;

use tg_sys::tg_index;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub enum IndexType {
    #[default]
    Default,
    None,
    Natural,
    YStripes,
}

impl From<IndexType> for tg_index {
    fn from(value: IndexType) -> Self {
        match value {
            IndexType::Default => tg_index::TG_DEFAULT,
            IndexType::None => tg_index::TG_NONE,
            IndexType::Natural => tg_index::TG_NATURAL,
            IndexType::YStripes => tg_index::TG_YSTRIPES,
        }
    }
}

impl From<tg_index> for IndexType {
    fn from(value: tg_index) -> Self {
        match value {
            tg_index::TG_DEFAULT => Self::Default,
            tg_index::TG_NONE => Self::None,
            tg_index::TG_NATURAL => Self::Natural,
            tg_index::TG_YSTRIPES => Self::YStripes,
        }
    }
}
