# Carosate de fier (`carosate_de_fier`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play carosate_de_fier --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 10×10. Unități (hp/mișcare/rază/zaruri-forță/armură): t = Tanc 4/3/4/3/2 [H]; i = Infanterie 2/2/3/2/1; m = Mersător 6/2/5/4/2 [HB]; a = Aviator 2/7/2/2/0 [L]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul are efect fix: forță minus armură (minim 1), fără zaruri. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 11 runde (sau 24 puncte). Obstacolele (#) blochează tragerea de la distanță. H = nu atacă după mișcare. L = zburător. B = explozie. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Iron Chariots.
