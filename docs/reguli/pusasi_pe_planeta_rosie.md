# Pușcași pe planeta roșie (`pusasi_pe_planeta_rosie`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pusasi_pe_planeta_rosie --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 9×8. Unități (hp/mișcare/rază/zaruri-forță/armură): p = Pușcaș 2/3/5/2/0; e = Exo-armură 5/2/3/4/2 [H]; s = Săritor 2/5/1/3/0 [LC]; m = Medic 2/3/1/1/0 [M]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 10 runde. Terenul dificil (~) dă +1 armură. Un 6 la zar rănește dublu. H = nu atacă după mișcare. L = zburător. M = vindecă aliații vecini. C = încărcare. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Riflemen on the Red Planet.
