//! Graf de puncte și linii pentru jocurile cu mutare pe puncte și salt (Sholo Guti,
//! Aadu Puli Attam): vecinii sunt punctele consecutive de pe o linie, iar un salt
//! trece peste un vecin pe punctul următor al ACELEIAȘI linii.

#[derive(Clone)]
pub struct LineGraph {
    pub pos: Vec<(i32, i32)>,
    pub adj: Vec<Vec<usize>>,
    /// pentru fiecare punct: (peste, spre)
    pub jumps: Vec<Vec<(usize, usize)>>,
}

impl LineGraph {
    pub fn new(pos: Vec<(i32, i32)>, lines: &[Vec<usize>]) -> Self {
        let n = pos.len();
        let mut adj = vec![Vec::new(); n];
        let mut jumps = vec![Vec::new(); n];
        for l in lines {
            for w in l.windows(2) {
                if !adj[w[0]].contains(&w[1]) {
                    adj[w[0]].push(w[1]);
                }
                if !adj[w[1]].contains(&w[0]) {
                    adj[w[1]].push(w[0]);
                }
            }
            for w in l.windows(3) {
                if !jumps[w[0]].contains(&(w[1], w[2])) {
                    jumps[w[0]].push((w[1], w[2]));
                }
                if !jumps[w[2]].contains(&(w[1], w[0])) {
                    jumps[w[2]].push((w[1], w[0]));
                }
            }
        }
        LineGraph { pos, adj, jumps }
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pos.is_empty()
    }

    /// Desen text: fiecare punct e înlocuit cu un jeton de 2 caractere dat de `tok`.
    pub fn draw(&self, tok: impl Fn(usize) -> String) -> String {
        let mut ys: Vec<i32> = self.pos.iter().map(|p| p.1).collect();
        ys.sort_unstable();
        ys.dedup();
        ys.reverse();
        let minx = self.pos.iter().map(|p| p.0).min().unwrap_or(0);
        let mut out = String::new();
        for y in ys {
            let mut line: Vec<char> = Vec::new();
            for (i, p) in self.pos.iter().enumerate() {
                if p.1 != y {
                    continue;
                }
                let col = ((p.0 - minx) * 3) as usize + 2;
                let t: Vec<char> = tok(i).chars().collect();
                if line.len() < col + t.len() {
                    line.resize(col + t.len(), ' ');
                }
                for (k, ch) in t.iter().enumerate() {
                    line[col + k] = *ch;
                }
            }
            out.push_str(line.iter().collect::<String>().trim_end());
            out.push('\n');
        }
        out
    }
}
