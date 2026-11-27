# Armate de buzunar (`armate_de_buzunar`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play armate_de_buzunar --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 11×9. Unități (hp/mișcare/rază/zaruri-forță/armură): i = Infanterie 2/2/2/2/1; t = Tanc 4/4/3/3/2; a = Artilerie 2/1/6/4/0 [H]; z = Zburător 3/7/2/2/0 [L]. Pe tură poți activa 5 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul are efect fix: forță minus armură (minim 1), fără zaruri. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 12 runde (sau 22 puncte). H = nu atacă după mișcare. L = zburător. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Pocket Armies.
