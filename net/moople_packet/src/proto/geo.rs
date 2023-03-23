use geo::{Coord, CoordNum};

use super::MapleWrapped;

impl<T> MapleWrapped for Coord<T>
where
    T: CoordNum,
{
    type Inner = (T, T);

    fn maple_into_inner(&self) -> Self::Inner {
        self.x_y()
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self::from(v)
    }
}
