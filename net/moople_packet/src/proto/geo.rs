use geo::{Coord, CoordNum};

use super::PacketWrapped;

impl<T> PacketWrapped for Coord<T>
where
    T: CoordNum,
{
    type Inner = (T, T);

    fn packet_into_inner(&self) -> Self::Inner {
        self.x_y()
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self::from(v)
    }
}
