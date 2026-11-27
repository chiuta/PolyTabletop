# Oști pe o filă (`osti_pe_o_fila`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play osti_pe_o_fila --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 9×9. Unități (hp/mișcare/rază/zaruri-forță/armură): p = Pușcaș 2/3/5/2/1; m = Mercenar 3/3/1/3/1; o = Ogar 5/2/1/4/2 [H]; r = Cercetaș 1/5/3/1/0 [L]. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 9 runde (sau 16 puncte). Obstacolele (#) blochează tragerea de la distanță. H = nu atacă după mișcare. L = zburător. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Armies on One Sheet.
