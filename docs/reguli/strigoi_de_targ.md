# Strigoi de târg (`strigoi_de_targ`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play strigoi_de_targ --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 7×7. Unități (hp/mișcare/rază/zaruri-forță/armură): f = Fantomă 2/4/1/2/0 [L]; l = Lampadar 3/3/3/2/1; p = Păpușar 3/3/1/3/1 [K]; c = Clovn 2/4/1/3/0 [C]. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: la sfârșitul turei ții punctele marcate cu * (mai multe unități în jur decât adversarul) și aduni 8 puncte sau ai mai multe după 9 runde. Un 6 la zar rănește dublu. Lovitura reușită împinge ținta (sau o rănește în plus dacă nu are loc). L = zburător. C = încărcare. K = împinge. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Ghosts of the Fairground.
