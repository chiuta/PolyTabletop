# Războinici de fiord (`razboinici_de_fiord`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play razboinici_de_fiord --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 8×6. Unități (hp/mișcare/rază/zaruri-forță/armură): g = Garda 2/3/1/3/1 [F]; b = Berserkul 3/4/1/4/0 [C]; l = Lăncierul 2/3/2/2/1; a = Arcașul 1/3/4/2/0. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: elimini oastea adversă sau ai mai multe puncte de pierderi după 10 runde. Un 6 la zar rănește dublu. Flanc: +1 zar dacă un aliat e lângă țintă. C = încărcare. F = flanc. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Fjord Warbands.
