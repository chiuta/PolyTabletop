# Salve și baionete (`salve_si_baionete`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play salve_si_baionete --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 10×8. Unități (hp/mișcare/rază/zaruri-forță/armură): l = Linie 3/2/3/2/0; g = Grenadier 3/3/1/3/1 [C]; t = Tun 2/1/6/3/0 [H]; h = Husar 3/5/1/2/0 [F]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul are efect fix: forță minus armură (minim 1), fără zaruri. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 10 runde (sau 18 puncte). Obstacolele (#) blochează tragerea de la distanță. H = nu atacă după mișcare. C = încărcare. F = flanc. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Volleys and Bayonets.
