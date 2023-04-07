use std::ops::RangeInclusive;

use geo::{coord, Coord, Rect, Contains};
use itertools::Itertools;
use rstar::{RTree, RTreeObject, SelectionFunction, AABB};

type FhScalar = f32;

fn clamp_range(r: RangeInclusive<FhScalar>, v: FhScalar) -> f32 {
    v.clamp(*r.start(), *r.end())
}

#[derive(Debug, Clone)]
pub struct FhSlope {
    line: geo::Line<FhScalar>,
    slope: FhScalar,
}

impl FhSlope {
    pub fn new(line: geo::Line<FhScalar>) -> Self {
        Self {
            line,
            slope: line.slope(),
        }
    }

    pub fn calc_y(&self, x: FhScalar) -> FhScalar {
        let x = x - self.line.start.x;
        // Basic linear interpolation
        x * self.slope + self.line.start.y
    }
}

#[derive(Debug, Clone)]
pub enum Foothold {
    Wall(geo::Line<FhScalar>),
    Platform(geo::Line<FhScalar>),
    Slope(FhSlope),
}

impl Foothold {
    pub fn from_coords(low: Coord<FhScalar>, high: Coord<FhScalar>) -> Self {
        let line = geo::Line::new(low, high);
        if high.x == low.x {
            return Foothold::Wall(line);
        }
        if high.y == low.y {
            return Foothold::Platform(line);
        }
        Foothold::Slope(FhSlope::new(line))
    }

    pub fn get_line(&self) -> &geo::Line<FhScalar> {
        match self {
            Foothold::Wall(l) => l,
            Foothold::Platform(l) => l,
            Foothold::Slope(l) => &l.line,
        }
    }

    pub fn get_x_range(&self) -> RangeInclusive<FhScalar> {
        let line = self.get_line();
        line.start.x..=line.end.x
    }

    pub fn calc_y(&self, x: FhScalar) -> FhScalar {
        match self {
            Foothold::Wall(l) => l.start.y.max(l.end.y),
            Foothold::Platform(l) => l.start.y,
            Foothold::Slope(slope) => slope.calc_y(x),
        }
    }

    pub fn get_coord_below(&self, c: geo::Coord<FhScalar>) -> Option<geo::Coord<FhScalar>> {
        let x_range = self.get_x_range();
        if !x_range.contains(&c.x) {
            return None;
        }

        let y = match self {
            Foothold::Wall(l) => l.start.y.max(l.end.y),
            Foothold::Platform(l) => l.start.y,
            Foothold::Slope(slope) => slope.calc_y(c.x),
        };

        if y < c.y {
            return None;
        }

        Some(coord! {x: c.x, y: y})
    }

    pub fn clamp_x(&self, x: f32) -> f32 {
        let r_x = self.get_x_range();
        x.clamp(*r_x.start(), *r_x.end())
    }

    pub fn get_item_spread(
        &self,
        x: FhScalar,
        items: usize,
    ) -> impl Iterator<Item = geo::Coord<FhScalar>> + '_ {
        //TODO make this config-able
        const STEP: FhScalar = 20.;
        let start_x = if items > 0 {
            x - (items - 1) as FhScalar * STEP
        } else {
            x
        };

        (0..items)
            .map(move |i| self.clamp_x(start_x + (i as f32) * STEP))
            .map(|x| (x, self.calc_y(x)).into())
    }
}

impl RTreeObject for Foothold {
    type Envelope = AABB<geo::Coord<FhScalar>>;

    fn envelope(&self) -> Self::Envelope {
        let line = self.get_line();
        AABB::from_corners(line.start, line.end)
    }
}

#[derive(Debug)]
pub struct FhTree {
    tree: RTree<Foothold, rstar::DefaultParams>,
    bounds: geo::Rect<FhScalar>,
}

pub struct BelowPointSelector {
    coord: geo::Coord<FhScalar>,
}

impl SelectionFunction<Foothold> for BelowPointSelector {
    fn should_unpack_parent(&self, envelope: &<Foothold as RTreeObject>::Envelope) -> bool {
        let u = envelope.upper();
        let l = envelope.lower();

        // Check that x is in range
        if !(l.x..=u.x).contains(&self.coord.x) {
            return false;
        }

        let min_y = u.y.max(l.y);
        // Check that y is below
        min_y > self.coord.y
    }

    fn should_unpack_leaf(&self, leaf: &Foothold) -> bool {
        leaf.get_coord_below(self.coord).is_some()
    }
}

impl FhTree {
    pub fn from_meta(meta: &game_data::map::Map) -> Self {
        let fhs = meta
            .foothold
            .values()
            .flat_map(|v| v.values())
            .flat_map(|v| v.iter())
            .map(|(_id, fh)| {
                Foothold::from_coords(
                    coord! { x: fh.x_1 as f32, y: fh.y_1 as f32 },
                    coord! { x: fh.x_2 as f32, y: fh.y_2 as f32 },
                )
            })
            .collect_vec();

        let info = &meta.info;

        let bounds = Rect::new(
            coord! { x: info.vr_left.unwrap_or(0) as f32, y: info.vr_bottom.unwrap_or(0) as f32 },
            coord! { x: info.vr_right.unwrap_or(0) as f32 , y: info.vr_top.unwrap_or(0) as f32 },
        );

        Self {
            tree: rstar::RTree::bulk_load(fhs),
            bounds,
        }
    }

    pub fn get_foothold_below(&self, coord: geo::Coord<FhScalar>) -> Option<&Foothold> {
        let (_x, y) = coord.x_y();
        let fh_below = self
            .tree
            .locate_with_selection_function(BelowPointSelector { coord });

        fh_below.min_by_key(|fh| (y - fh.get_coord_below(coord).unwrap().y).abs() as i32)
    }

    pub fn x_range(&self) -> RangeInclusive<FhScalar> {
        self.bounds.min().x..=self.bounds.max().x
    }

    pub fn y_range(&self) -> RangeInclusive<FhScalar> {
        self.bounds.min().y..=self.bounds.max().y
    }

    pub fn clamp(&self, coord: geo::Coord<FhScalar>) -> geo::Coord<FhScalar> {
        (
            clamp_range(self.x_range(), coord.x),
            clamp_range(self.y_range(), coord.y),
        )
            .into()
    }

    pub fn contains(&self, coord: geo::Coord<FhScalar>) -> bool {
        self.bounds.contains(&coord)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use geo_svg::ToSvg;
    use proto95::id::MapId;

    use crate::services::meta::meta_service::MetaService;

    use super::*;

    #[test]
    fn map_fh() -> anyhow::Result<()> {
        let meta = MetaService::load_from_dir("../../game_data/rbin")?;
        let field_1 = meta.get_field_data(MapId::SOUTHPERRY).unwrap();

        let fh_tree = FhTree::from_meta(&field_1);
        dbg!(&fh_tree.bounds);
        let line0 = geo::Point::new(0., 0.);
        let mut svg = line0.to_svg();

        for fh in fh_tree.tree.iter() {
            let svg_line = fh
                .get_line()
                .to_svg()
                .with_radius(2.0)
                .with_stroke_width(5.)
                .with_fill_opacity(0.7);
            let color = match fh {
                Foothold::Wall(_) => "black",
                Foothold::Platform(_) => "green",
                Foothold::Slope(_) => "red",
            };

            svg = svg.and(
                svg_line
                    .with_fill_color(geo_svg::Color::Named(color))
                    .with_stroke_color(geo_svg::Color::Named(color)),
            );
        }

        let mut lines = Vec::new();

        let bounds = &fh_tree.bounds;
        for test_pt_x in (bounds.min().x as i32..bounds.max().x as i32).step_by(50) {
            let test_pt_x = test_pt_x as f32;
            let pt = (test_pt_x, 0.).into();

            if let Some(fh) = fh_tree.get_foothold_below(pt) {
                let intersec = fh.get_coord_below(pt).unwrap();
                let line = geo::Line::new(pt, intersec);
                lines.push(line);
            } else {
                dbg!(pt);
            }
        }

        for line in lines.iter() {
            svg = svg.and(
                line.to_svg()
                    .with_radius(2.0)
                    .with_fill_color(geo_svg::Color::Named("blue"))
                    .with_stroke_color(geo_svg::Color::Named("blue"))
                    .with_stroke_width(5.)
                    .with_fill_opacity(0.7),
            );
        }

        let mut f = File::create("sp2.svg")?;
        writeln!(&mut f, "{svg}")?;

        Ok(())
    }
}
