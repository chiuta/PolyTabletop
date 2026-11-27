# Scuturi și raze (`scuturi_si_raze`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play scuturi_si_raze --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă hexagonală 10×8. Unități (hp/mișcare/rază/zaruri-forță/armură): s = Scutier 5/3/3/4/3; r = Rapid 3/5/3/3/1 [L]; m = Medical 4/3/1/1/1 [M]. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul are efect fix: forță minus armură (minim 1), fără zaruri. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 11 runde. L = zburător. M = vindecă aliații vecini. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Shields and Beams.
