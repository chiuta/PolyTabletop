# Titani de ceață (`titani_de_ceata`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play titani_de_ceata --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă hexagonală 9×9. Unități (hp/mișcare/rază/zaruri-forță/armură): t = Titan 9/2/5/5/3 [HBK]; s = Scutier 3/3/2/2/1 [M]; e = Explorator 3/5/3/3/1 [L]. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 9 runde. Obstacolele (#) blochează tragerea de la distanță. Un 6 la zar rănește dublu. H = nu atacă după mișcare. L = zburător. M = vindecă aliații vecini. B = explozie. K = împinge. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Titans of the Mist.
