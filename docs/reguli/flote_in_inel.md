# Flote în inel (`flote_in_inel`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play flote_in_inel --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă hexagonală 9×9. Unități (hp/mișcare/rază/zaruri-forță/armură): c = Cruiser 5/3/4/3/2; e = Escortă 2/5/2/2/0 [L]; d = Dronă 1/6/1/2/0 [L]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: la sfârșitul turei ții punctele marcate cu * (mai multe unități în jur decât adversarul) și aduni 9 puncte sau ai mai multe după 9 runde. Obstacolele (#) blochează tragerea de la distanță. Lovitura reușită împinge ținta (sau o rănește în plus dacă nu are loc). L = zburător. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Fleets in the Ring.
