# Hochei pe masă (ture) (`hochei_masa`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Air hockey”. Pornire: `polytabletop play hochei_masa --seed 1`. Sursa implementării: `polytabletop-games/src/hochei_masa.rs`.

> Reguli: Masă 7×11, porți late de 3 celule. Fiecare are o paletă în jumătatea lui. O tură: muți paleta un pas (8 direcții) sau stai; dacă pucul e în celula paletei sau vecină, îl lovești: primește viteza paletei spre puc (dublată) plus mișcarea paletei, max. 2 pe axă. Pucul parcurge celulele, ricoșează de pereți și de paleta adversă; trecut de linia de capăt pe poartă = gol. Viteza scade cu 1 pe tură. Primul la 3 goluri câștigă; după 300 de semi-mutări, cine are mai multe. Mutări: n, s, e, v, ne, nv, se, sv, stau.
