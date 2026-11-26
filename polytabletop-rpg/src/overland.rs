//! Hex-crawl / overland map generator with weather and travel time.
//!
//! Hexes use "odd-r" offset coordinates (odd rows are shifted half a hex to
//! the right in the ASCII render).

use crate::gen::pick_weighted;
use polytabletop_core::rng::Rng;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terrain {
    Water,
    Plains,
    Forest,
    Hills,
    Mountain,
    Swamp,
    Desert,
}

impl Terrain {
    pub fn symbol(self) -> char {
        match self {
            Terrain::Water => '~',
            Terrain::Plains => '.',
            Terrain::Forest => 'f',
            Terrain::Hills => 'n',
            Terrain::Mountain => 'M',
            Terrain::Swamp => 's',
            Terrain::Desert => ':',
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Terrain::Water => "water",
            Terrain::Plains => "plains",
            Terrain::Forest => "forest",
            Terrain::Hills => "hills",
            Terrain::Mountain => "mountain",
            Terrain::Swamp => "swamp",
            Terrain::Desert => "desert",
        }
    }
    /// Travel cost in "plains hexes"; `None` = impassable on foot.
    pub fn cost(self) -> Option<u32> {
        match self {
            Terrain::Water => None,
            Terrain::Plains => Some(1),
            Terrain::Forest | Terrain::Hills | Terrain::Desert => Some(2),
            Terrain::Swamp => Some(3),
            Terrain::Mountain => Some(4),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feature {
    None,
    Town,
    Ruin,
    Lair,
}

impl Feature {
    pub fn symbol(self) -> Option<char> {
        match self {
            Feature::None => None,
            Feature::Town => Some('T'),
            Feature::Ruin => Some('R'),
            Feature::Lair => Some('L'),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn parse(s: &str) -> Option<Season> {
        Some(match s.to_lowercase().as_str() {
            "spring" | "primavara" | "primăvară" => Season::Spring,
            "summer" | "vara" | "vară" => Season::Summer,
            "autumn" | "fall" | "toamna" | "toamnă" => Season::Autumn,
            "winter" | "iarna" | "iarnă" => Season::Winter,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weather {
    Clear,
    Cloudy,
    Rain,
    Storm,
    Fog,
    Snow,
    Heat,
}

impl Weather {
    pub fn name(self) -> &'static str {
        match self {
            Weather::Clear => "clear",
            Weather::Cloudy => "cloudy",
            Weather::Rain => "rain",
            Weather::Storm => "storm",
            Weather::Fog => "fog",
            Weather::Snow => "snow",
            Weather::Heat => "heat",
        }
    }
    /// Movement-cost multiplier in percent.
    pub fn cost_pct(self) -> u32 {
        match self {
            Weather::Clear | Weather::Cloudy => 100,
            Weather::Rain | Weather::Fog | Weather::Heat => 125,
            Weather::Snow => 150,
            Weather::Storm => 200,
        }
    }
    pub fn roll(season: Season, rng: &mut Rng) -> Weather {
        // clear, cloudy, rain, storm, fog, snow, heat
        let w: [u32; 7] = match season {
            Season::Spring => [30, 25, 25, 7, 10, 3, 0],
            Season::Summer => [40, 15, 12, 10, 3, 0, 20],
            Season::Autumn => [20, 30, 22, 8, 18, 2, 0],
            Season::Winter => [18, 22, 5, 5, 10, 40, 0],
        };
        [
            Weather::Clear,
            Weather::Cloudy,
            Weather::Rain,
            Weather::Storm,
            Weather::Fog,
            Weather::Snow,
            Weather::Heat,
        ][pick_weighted(&w, rng)]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HexMap {
    pub w: usize,
    pub h: usize,
    pub terrain: Vec<Terrain>,
    pub features: Vec<Feature>,
}

fn smooth(v: &[f64], w: usize, h: usize) -> Vec<f64> {
    let mut o = vec![0.0; v.len()];
    for y in 0..h {
        for x in 0..w {
            let mut s = v[y * w + x] * 2.0;
            let mut n = 2.0;
            for (nx, ny) in HexMap::raw_neighbors(x, y, w, h) {
                s += v[ny * w + nx];
                n += 1.0;
            }
            o[y * w + x] = s / n;
        }
    }
    o
}

/// Spreads values back to 0..1 so thresholds keep their meaning after blur.
fn normalize(v: &mut [f64]) {
    let (lo, hi) = v
        .iter()
        .fold((f64::MAX, f64::MIN), |(l, h), &x| (l.min(x), h.max(x)));
    let r = (hi - lo).max(1e-9);
    for x in v.iter_mut() {
        *x = (*x - lo) / r;
    }
}

impl HexMap {
    pub fn raw_neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
        let (xi, yi) = (x as i64, y as i64);
        let d: [(i64, i64); 6] = if y.is_multiple_of(2) {
            [(-1, 0), (1, 0), (-1, -1), (0, -1), (-1, 1), (0, 1)]
        } else {
            [(-1, 0), (1, 0), (0, -1), (1, -1), (0, 1), (1, 1)]
        };
        d.iter()
            .filter_map(|&(dx, dy)| {
                let (nx, ny) = (xi + dx, yi + dy);
                (nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h)
                    .then_some((nx as usize, ny as usize))
            })
            .collect()
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        HexMap::raw_neighbors(x, y, self.w, self.h)
    }

    pub fn generate(w: usize, h: usize, rng: &mut Rng) -> HexMap {
        let (w, h) = (w.clamp(3, 80), h.clamp(3, 60));
        let mut elev: Vec<f64> = (0..w * h)
            .map(|_| rng.below(1000) as f64 / 1000.0)
            .collect();
        let mut moist = elev.clone();
        for m in moist.iter_mut() {
            *m = rng.below(1000) as f64 / 1000.0;
        }
        for _ in 0..3 {
            elev = smooth(&elev, w, h);
            moist = smooth(&moist, w, h);
        }
        normalize(&mut elev);
        normalize(&mut moist);
        let terrain: Vec<Terrain> = (0..w * h)
            .map(|i| {
                let (e, m) = (elev[i], moist[i]);
                if e < 0.25 {
                    Terrain::Water
                } else if e > 0.85 {
                    Terrain::Mountain
                } else if e > 0.68 {
                    Terrain::Hills
                } else if m > 0.7 && e < 0.4 {
                    Terrain::Swamp
                } else if m > 0.5 {
                    Terrain::Forest
                } else if m < 0.2 {
                    Terrain::Desert
                } else {
                    Terrain::Plains
                }
            })
            .collect();
        let mut features = vec![Feature::None; w * h];
        for i in 0..w * h {
            if terrain[i] == Terrain::Water {
                continue;
            }
            let r = rng.below(100);
            features[i] = match (terrain[i], r) {
                (Terrain::Plains | Terrain::Forest, 0..=2) => Feature::Town,
                (_, 3..=4) => Feature::Ruin,
                (Terrain::Hills | Terrain::Mountain | Terrain::Swamp, 5..=6) => Feature::Lair,
                _ => Feature::None,
            };
        }
        HexMap {
            w,
            h,
            terrain,
            features,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Terrain {
        self.terrain[y * self.w + x]
    }
    pub fn feature(&self, x: usize, y: usize) -> Feature {
        self.features[y * self.w + x]
    }
    pub fn count(&self, t: Terrain) -> usize {
        self.terrain.iter().filter(|&&x| x == t).count()
    }

    /// Cheapest path (inclusive of both ends) and its cost in plains hexes.
    pub fn path(
        &self,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Option<(Vec<(usize, usize)>, u32)> {
        if from.0 >= self.w || to.0 >= self.w || from.1 >= self.h || to.1 >= self.h {
            return None;
        }
        let idx = |p: (usize, usize)| p.1 * self.w + p.0;
        if self.at(to.0, to.1).cost().is_none() || self.at(from.0, from.1).cost().is_none() {
            return None;
        }
        let mut dist = vec![u32::MAX; self.w * self.h];
        let mut prev = vec![usize::MAX; self.w * self.h];
        let mut heap = BinaryHeap::new();
        dist[idx(from)] = 0;
        heap.push(Reverse((0u32, idx(from))));
        while let Some(Reverse((d, i))) = heap.pop() {
            if d > dist[i] {
                continue;
            }
            if i == idx(to) {
                break;
            }
            let (x, y) = (i % self.w, i / self.w);
            for (nx, ny) in self.neighbors(x, y) {
                let Some(c) = self.at(nx, ny).cost() else {
                    continue;
                };
                let j = ny * self.w + nx;
                if d + c < dist[j] {
                    dist[j] = d + c;
                    prev[j] = i;
                    heap.push(Reverse((d + c, j)));
                }
            }
        }
        if dist[idx(to)] == u32::MAX {
            return None;
        }
        let mut p = vec![];
        let mut cur = idx(to);
        while cur != usize::MAX {
            p.push((cur % self.w, cur / self.w));
            cur = prev[cur];
        }
        p.reverse();
        Some((p, dist[idx(to)]))
    }

    pub fn render(&self, path: &[(usize, usize)]) -> String {
        let mut s = String::new();
        for y in 0..self.h {
            if y % 2 == 1 {
                s.push(' ');
            }
            for x in 0..self.w {
                let c = if path.contains(&(x, y)) {
                    '*'
                } else {
                    self.feature(x, y)
                        .symbol()
                        .unwrap_or_else(|| self.at(x, y).symbol())
                };
                s.push(c);
                s.push(' ');
            }
            s.truncate(s.trim_end().len());
            s.push('\n');
        }
        s.push_str("~ apă  . câmpie  f pădure  n dealuri  M munți  s mlaștină  : deșert  T oraș  R ruină  L bârlog  * traseu\n");
        s
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Journey {
    pub days: u32,
    /// Hours of travel on the last day (8 h marching day).
    pub last_day_hours: u32,
    pub total_hours: u32,
    pub weather: Vec<Weather>,
}

/// Simulates marching along `path_cost` (plains-hex units): the party covers
/// `pace` plains hexes a day, divided by the day's weather multiplier.
pub fn travel(path_cost: u32, pace: u32, season: Season, rng: &mut Rng) -> Journey {
    let pace = f64::from(pace.max(1));
    let mut left = f64::from(path_cost);
    let mut weather = vec![];
    let mut days = 0;
    let mut last_hours = 0;
    while left > 1e-9 && days < 1000 {
        let wx = Weather::roll(season, rng);
        weather.push(wx);
        days += 1;
        let cover = pace * 100.0 / f64::from(wx.cost_pct());
        if left <= cover {
            last_hours = ((left / cover) * 8.0).ceil() as u32;
            left = 0.0;
        } else {
            left -= cover;
            last_hours = 8;
        }
    }
    Journey {
        days,
        last_day_hours: last_hours,
        total_hours: days.saturating_sub(1) * 8 + last_hours,
        weather,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_are_symmetric() {
        for y in 0..5 {
            for x in 0..5 {
                for (nx, ny) in HexMap::raw_neighbors(x, y, 5, 5) {
                    assert!(HexMap::raw_neighbors(nx, ny, 5, 5).contains(&(x, y)));
                }
            }
        }
    }
}
