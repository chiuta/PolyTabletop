# Falangă și călărime (`falange_si_calarime`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play falange_si_calarime --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 10×7. Unități (hp/mișcare/rază/zaruri-forță/armură): f = Falangist 4/2/1/3/3 [H]; c = Cavalerist 3/5/1/3/1 [C]; p = Praștiaș 1/3/3/1/0; e = Elefant 6/3/1/4/2 [KC]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul are efect fix: forță minus armură (minim 1), fără zaruri. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 11 runde (sau 20 puncte). Încărcare: +1 zar după o mutare de cel puțin 2 pași. Flanc: +1 zar dacă un aliat e lângă țintă. H = nu atacă după mișcare. C = încărcare. K = împinge. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Phalanx and Cavalry.
