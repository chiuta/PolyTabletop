# Balada vitejilor (`balada_vitejilor`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play balada_vitejilor --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 7×7. Unități (hp/mișcare/rază/zaruri-forță/armură): e = Erou 4/3/1/4/1 [F]; t = Tâlhar 2/4/1/2/0; a = Arcaș 1/3/4/2/0; m = Menestrel 2/3/1/1/0 [M]. Pe tură poți activa 2 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 12 runde. Un 6 la zar rănește dublu. Flanc: +1 zar dacă un aliat e lângă țintă. M = vindecă aliații vecini. F = flanc. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Ballad of the Brave.
