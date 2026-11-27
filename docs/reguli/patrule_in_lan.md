# Patrule în lan (`patrule_in_lan`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play patrule_in_lan --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 9×7. Unități (hp/mișcare/rază/zaruri-forță/armură): f = Pușcaș 2/3/4/2/0; m = Mitralior 2/2/6/3/0 [H]; c = Comandant 3/3/2/2/1 [F]; s = Sanitar 2/3/1/1/0 [M]. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: la sfârșitul turei ții punctele marcate cu * (mai multe unități în jur decât adversarul) și aduni 9 puncte sau ai mai multe după 9 runde. Obstacolele (#) blochează tragerea de la distanță. Terenul dificil (~) dă +1 armură. H = nu atacă după mișcare. M = vindecă aliații vecini. F = flanc. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Patrols in the Wheat Field.
