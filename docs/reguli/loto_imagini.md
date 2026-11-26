# Loto cu imagini (Picture lotto) (`loto_imagini`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play loto_imagini --seed 1`. Sursa implementării: `polytabletop-games/src/loto_imagini.rs`.

> Rezumat din `polytabletop list`: Cartonașe 3×3 cu imagini din 24; tragi pe rând din sac, acoperi imaginea de pe cartonaș sau o lași; primul cu cartonașul plin strigă loto. Mutări: trage, acopera, pas.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Loto cu imagini (Picture lotto), 2 jucători — variantă pe cartonașe 3×3.


* 24 de imagini (cuvinte); fiecare jucător are un cartonaș cu 9 imagini diferite, public.

* La rândul tău tragi din sac o imagine (`trage`). Dacă imaginea e pe cartonașul tău poți să o

  acoperi (`acopera`) sau să o lași (`pas`); apoi, dacă ea e și pe cartonașul adversarului, el

  poate la fel să o acopere. Cine acoperă primul tot cartonașul (9 imagini) strigă «loto» și câștigă;

  dacă sacul se golește, câștigă cine are mai multe imagini acoperite (egal = remiză).

* Fiecare jucător alege pe rând cine trage: cel care a tras ultimul cedează tragerea.


Mutări: `trage`, `acopera`, `pas`.
