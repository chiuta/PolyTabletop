# Chemarea hoardelor (`chemarea_hoardelor`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chemarea_hoardelor --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 8×7. Unități (hp/mișcare/rază/zaruri-forță/armură): h = Hoardă 2/3/1/3/0 [C]; s = Șaman 2/3/3/2/0 [M]; m = Monstru 5/3/1/4/1 [K]; a = Aruncător 1/4/4/1/0. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 9 runde (sau 14 puncte). Un 6 la zar rănește dublu. Încărcare: +1 zar după o mutare de cel puțin 2 pași. M = vindecă aliații vecini. C = încărcare. K = împinge. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Call of the Hordes.
