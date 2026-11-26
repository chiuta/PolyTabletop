# Boggle (`boggle`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play boggle --seed 1`. Sursa implementării: `polytabletop-games/src/boggle.rs`.

> Rezumat din `polytabletop list`: Boggle: tablă 4×4 de litere; pe rând scrii câte un cuvânt de cel puțin 3 litere format din cuburi vecine, valid în dicționar și încă negăsit; punctele cresc cu lungimea. Mutări: casa, pas.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Boggle (SUA), 2 jucători, adaptare pe ture cu dicționarul românesc mic al jocului „Litere”.


* Grilă 4×4 de litere (generată din seed, cu cel puțin 20 de cuvinte valide). Un cuvânt se formează din

  litere vecine (inclusiv pe diagonală), fără a refolosi aceeași căsuță în același cuvânt, de cel puțin

  3 litere, fără diacritice (ca în `Litere`).

* Pe rând, fiecare jucător revendică **un cuvânt** găsit în grilă (fiecare cuvânt se poate revendica o

  singură dată). Puncte: lungimea − 2 (3 litere = 1, 4 = 2, 5 = 3 …). Câte 10 revendicări maxim pentru

  fiecare; `pas` îți cedează rândul, iar două pasuri la rând încheie jocul.

* Cuvintele valide pot fi listate cu `?` (lista mutărilor legale) — pentru un joc fără ajutor nu folosi `?`.


Mutări: `casa` (cuvântul scris fără diacritice), `pas`.
