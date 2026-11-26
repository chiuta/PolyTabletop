//! Simple grid dungeon generator: rectangular rooms joined by L-shaped corridors.

use crate::tables::RandomTable;
use polytabletop_core::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Door,
    StairsUp,
    StairsDown,
}

impl Tile {
    pub fn walkable(self) -> bool {
        self != Tile::Wall
    }
    pub fn glyph(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Door => '+',
            Tile::StairsUp => '<',
            Tile::StairsDown => '>',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Room {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Room {
    pub fn center(&self) -> (usize, usize) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
    fn overlaps(&self, o: &Room, gap: usize) -> bool {
        self.x < o.x + o.w + gap
            && o.x < self.x + self.w + gap
            && self.y < o.y + o.h + gap
            && o.y < self.y + self.h + gap
    }
}

#[derive(Clone, Debug)]
pub struct Dungeon {
    pub w: usize,
    pub h: usize,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub const MIN_W: usize = 16;
    pub const MIN_H: usize = 10;

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.w + x]
    }
    fn set(&mut self, x: usize, y: usize, t: Tile) {
        self.tiles[y * self.w + x] = t;
    }

    /// Generate a dungeon; `w`/`h` are clamped to a sane minimum.
    pub fn generate(w: usize, h: usize, max_rooms: usize, rng: &mut Rng) -> Dungeon {
        let w = w.clamp(Self::MIN_W, 200);
        let h = h.clamp(Self::MIN_H, 100);
        let mut d = Dungeon {
            w,
            h,
            tiles: vec![Tile::Wall; w * h],
            rooms: vec![],
        };
        let max_rooms = max_rooms.max(1);
        for _ in 0..max_rooms * 12 {
            if d.rooms.len() >= max_rooms {
                break;
            }
            let rw = 3 + rng.below(6.min(w - 6));
            let rh = 3 + rng.below(5.min(h - 6));
            if rw + 2 > w || rh + 2 > h {
                continue;
            }
            let x = 1 + rng.below(w - rw - 1);
            let y = 1 + rng.below(h - rh - 1);
            let room = Room {
                id: d.rooms.len(),
                x,
                y,
                w: rw,
                h: rh,
            };
            if d.rooms.iter().any(|r| r.overlaps(&room, 2)) {
                continue;
            }
            d.rooms.push(room);
        }
        if d.rooms.is_empty() {
            d.rooms.push(Room {
                id: 0,
                x: 2,
                y: 2,
                w: 4,
                h: 4,
            });
        }
        let rooms = d.rooms.clone();
        for r in &rooms {
            for yy in r.y..r.y + r.h {
                for xx in r.x..r.x + r.w {
                    d.set(xx, yy, Tile::Floor);
                }
            }
        }
        for i in 1..rooms.len() {
            let (ax, ay) = rooms[i - 1].center();
            let (bx, by) = rooms[i].center();
            if rng.below(2) == 0 {
                d.hline(ax, bx, ay);
                d.vline(ay, by, bx);
            } else {
                d.vline(ay, by, ax);
                d.hline(ax, bx, by);
            }
        }
        d.place_doors();
        if let Some(r) = rooms.first() {
            let (x, y) = r.center();
            d.set(x, y, Tile::StairsUp);
        }
        if rooms.len() > 1 {
            let (x, y) = rooms[rooms.len() - 1].center();
            d.set(x, y, Tile::StairsDown);
        }
        d
    }

    fn hline(&mut self, x0: usize, x1: usize, y: usize) {
        for x in x0.min(x1)..=x0.max(x1) {
            self.set(x, y, Tile::Floor);
        }
    }
    fn vline(&mut self, y0: usize, y1: usize, x: usize) {
        for y in y0.min(y1)..=y0.max(y1) {
            self.set(x, y, Tile::Floor);
        }
    }

    fn in_room(&self, x: usize, y: usize) -> bool {
        self.rooms
            .iter()
            .any(|r| x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h)
    }

    /// A one-wide corridor tile touching a room edge becomes a door.
    fn place_doors(&mut self) {
        let rooms = self.rooms.clone();
        let mut doors = Vec::new();
        for r in &rooms {
            for x in r.x..r.x + r.w {
                for y in [r.y.wrapping_sub(1), r.y + r.h] {
                    if y < self.h
                        && self.tile(x, y) == Tile::Floor
                        && x > 0
                        && x + 1 < self.w
                        && self.tile(x - 1, y) == Tile::Wall
                        && self.tile(x + 1, y) == Tile::Wall
                    {
                        doors.push((x, y));
                    }
                }
            }
            for y in r.y..r.y + r.h {
                for x in [r.x.wrapping_sub(1), r.x + r.w] {
                    if x < self.w
                        && self.tile(x, y) == Tile::Floor
                        && y > 0
                        && y + 1 < self.h
                        && self.tile(x, y - 1) == Tile::Wall
                        && self.tile(x, y + 1) == Tile::Wall
                    {
                        doors.push((x, y));
                    }
                }
            }
        }
        for (x, y) in doors {
            if !self.in_room(x, y) {
                self.set(x, y, Tile::Door);
            }
        }
    }

    /// Tiles reachable from the first room's centre.
    pub fn reachable(&self) -> usize {
        let (sx, sy) = self.rooms[0].center();
        let mut seen = vec![false; self.w * self.h];
        let mut stack = vec![(sx, sy)];
        seen[sy * self.w + sx] = true;
        let mut n = 0;
        while let Some((x, y)) = stack.pop() {
            n += 1;
            let nb = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            for (nx, ny) in nb {
                if nx < self.w
                    && ny < self.h
                    && !seen[ny * self.w + nx]
                    && self.tile(nx, ny).walkable()
                {
                    seen[ny * self.w + nx] = true;
                    stack.push((nx, ny));
                }
            }
        }
        n
    }

    pub fn walkable_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.walkable()).count()
    }

    /// Roll a table for each room except the entrance room.
    pub fn populate(&self, table: &RandomTable, rng: &mut Rng) -> Vec<(usize, String)> {
        self.rooms
            .iter()
            .skip(1)
            .map(|r| (r.id, table.roll(rng).1.to_string()))
            .collect()
    }

    pub fn render(&self) -> String {
        let mut o = String::with_capacity((self.w + 1) * self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                o.push(self.tile(x, y).glyph());
            }
            o.push('\n');
        }
        o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_for_many_seeds() {
        for seed in 1..=60u64 {
            let d = Dungeon::generate(60, 24, 9, &mut Rng::new(seed));
            assert_eq!(d.reachable(), d.walkable_count(), "seed {seed}");
            assert!(!d.rooms.is_empty());
        }
    }

    #[test]
    fn rooms_inside_and_disjoint() {
        for seed in 1..=40u64 {
            let d = Dungeon::generate(50, 20, 8, &mut Rng::new(seed));
            for (i, r) in d.rooms.iter().enumerate() {
                assert!(r.x >= 1 && r.y >= 1 && r.x + r.w < d.w && r.y + r.h < d.h);
                assert!(r.w >= 3 && r.h >= 3);
                for o in &d.rooms[i + 1..] {
                    assert!(!r.overlaps(o, 1));
                }
            }
            // border stays wall
            for x in 0..d.w {
                assert_eq!(d.tile(x, 0), Tile::Wall);
                assert_eq!(d.tile(x, d.h - 1), Tile::Wall);
            }
            for y in 0..d.h {
                assert_eq!(d.tile(0, y), Tile::Wall);
                assert_eq!(d.tile(d.w - 1, y), Tile::Wall);
            }
        }
    }

    #[test]
    fn stairs_and_render() {
        let d = Dungeon::generate(60, 24, 8, &mut Rng::new(5));
        let s = d.render();
        assert_eq!(s.lines().count(), d.h);
        assert!(s.lines().all(|l| l.chars().count() == d.w));
        assert_eq!(s.matches('<').count(), 1);
        if d.rooms.len() > 1 {
            assert_eq!(s.matches('>').count(), 1);
        }
        assert!(s.contains('.'));
    }

    #[test]
    fn deterministic_and_varied() {
        let a = Dungeon::generate(60, 24, 8, &mut Rng::new(3)).render();
        let b = Dungeon::generate(60, 24, 8, &mut Rng::new(3)).render();
        let c = Dungeon::generate(60, 24, 8, &mut Rng::new(4)).render();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn tiny_sizes_are_clamped() {
        let d = Dungeon::generate(1, 1, 0, &mut Rng::new(1));
        assert!(d.w >= Dungeon::MIN_W && d.h >= Dungeon::MIN_H);
        assert_eq!(d.reachable(), d.walkable_count());
    }

    #[test]
    fn reasonable_room_count_and_populate() {
        let d = Dungeon::generate(80, 30, 10, &mut Rng::new(9));
        assert!(d.rooms.len() >= 4, "{}", d.rooms.len());
        let t = RandomTable::weighted("R", &[(1, "empty"), (1, "monster")]).unwrap();
        let p = d.populate(&t, &mut Rng::new(1));
        assert_eq!(p.len(), d.rooms.len() - 1);
    }
}
