# Flote cu aburi (`flote_cu_aburi`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play flote_cu_aburi --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă hexagonală 10×8. Unități (hp/mișcare/rază/zaruri-forță/armură): n = Monitor 6/2/4/4/3 [HB]; v = Vapor rapid 3/4/3/3/1; z = Zeppelin 3/3/2/3/1 [L]; t = Tender 3/3/1/1/1 [M]. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 10 runde. Obstacolele (#) blochează tragerea de la distanță. Un 6 la zar rănește dublu. H = nu atacă după mișcare. L = zburător. M = vindecă aliații vecini. B = explozie. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Steam Fleets.
