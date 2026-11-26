# Puzzle cu piese (Jigsaw) (`jigsaw`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play jigsaw --seed 1`. Sursa implementării: `polytabletop-games/src/jigsaw.rs`.

> Rezumat din `polytabletop list`: Puzzle 4×4 în întrecere: pe rând așezi o piesă care se potrivește cu vecinele (margini plate pe cadru); punctele = 1 + vecinii îmbinați, ultima piesă +3. Mutări: p7r2c3.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Puzzle cu piese (Jigsaw, Marea Britanie), 2 jucători în întrecere — adaptare pe ture a puzzle-ului.


* Puzzle 4×4 = 16 piese. Fiecare piesă are patru margini: `0` = margine plată (marginea cadrului), iar o

  margine interioară are un cod unic (+k pe o piesă, −k pe vecina cu care se îmbină), amestecat la start.

* Pe rând, fiecare jucător așază **o piesă** într-un loc liber, dacă: (1) toate marginile ei se potrivesc cu

  vecinele deja așezate (suma codurilor = 0), (2) marginile plate ale piesei cad exact pe marginea

  cadrului, (3) locul e în **colț** sau lipit de o piesă deja așezată (puzzle-ul crește din cadru).

* Puncte: 1 + numărul de vecini deja așezați cu care se îmbină piesa; cine așază ultima piesă primește +3.

  La final câștigă cine are mai multe puncte (egal = remiză).


Mutări: `p7r2c3` (piesa 7 la rândul 2, coloana 3, numărând de la 1).
