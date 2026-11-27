# Aterizare peste oraș (`aterizare_peste_oras`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play aterizare_peste_oras --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 9×9. Unități (hp/mișcare/rază/zaruri-forță/armură): a = Aeronavă 3/6/1/1/1 [L]; t = Tanc 4/3/4/3/2 [H]; s = Soldat 2/3/3/2/0; w = Walker 4/2/3/3/2 [K]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: duci unități pe rândul de la capătul opus (ies din joc, câte 2 puncte); primul la 8 puncte sau cu mai multe după 12 runde câștigă. Obstacolele (#) blochează tragerea de la distanță. H = nu atacă după mișcare. L = zburător. K = împinge. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Landing over the City.
