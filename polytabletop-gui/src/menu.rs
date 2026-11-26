//! Meniul de jocuri: grupare pe categorii și căutare (fără diacritice).

pub fn fold(s: &str) -> String {
    s.chars()
        .flat_map(|c| c.to_lowercase())
        .map(|c| match c {
            'ă' | 'â' | 'á' | 'à' | 'ä' => 'a',
            'î' | 'í' | 'ï' => 'i',
            'ș' | 'ş' | 'ś' => 's',
            'ț' | 'ţ' => 't',
            'é' | 'è' | 'ê' => 'e',
            'ö' | 'ó' => 'o',
            'ü' | 'ú' => 'u',
            c => c,
        })
        .collect()
}

/// `items` = (id, titlu, categorie). Întoarce (categorie, indici) în ordinea
/// primei apariții a categoriei; doar elementele care se potrivesc cu `query`
/// (toate cuvintele trebuie găsite în titlu/id/categorie).
pub fn groups(items: &[(&str, &str, &str)], query: &str) -> Vec<(String, Vec<usize>)> {
    let q = fold(query);
    let words: Vec<&str> = q.split_whitespace().collect();
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();
    for (i, (id, title, cat)) in items.iter().enumerate() {
        let hay = format!("{} {} {}", fold(id), fold(title), fold(cat));
        if !words.iter().all(|w| hay.contains(w)) {
            continue;
        }
        match out.iter_mut().find(|(c, _)| c == cat) {
            Some((_, v)) => v.push(i),
            None => out.push((cat.to_string(), vec![i])),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const IT: [(&str, &str, &str); 4] = [
        ("chess", "Șah", "Abstract"),
        ("go", "Go (9×9)", "Abstract"),
        ("septica", "Septica", "Cărți"),
        ("barbut", "Barbut", "Zaruri"),
    ];

    #[test]
    fn all_grouped() {
        let g = groups(&IT, "");
        assert_eq!(g.len(), 3);
        assert_eq!(g[0], ("Abstract".to_string(), vec![0, 1]));
    }

    #[test]
    fn diacritics_insensitive() {
        assert_eq!(groups(&IT, "sah")[0].1, vec![0]);
        assert_eq!(groups(&IT, "ȘAH")[0].1, vec![0]);
        assert_eq!(groups(&IT, "carti")[0].0, "Cărți");
        assert!(groups(&IT, "inexistent").is_empty());
        assert_eq!(groups(&IT, "abstract go")[0].1, vec![1]);
    }
}
