# Domni și arcași (`domni_si_arcasi`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play domni_si_arcasi --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 9×8. Unități (hp/mișcare/rază/zaruri-forță/armură): c = Călăreț 3/5/1/3/1 [C]; s = Sergent 2/3/1/2/1 [F]; a = Arcaș 1/3/4/2/0; d = Domn 4/3/1/4/2. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: la sfârșitul turei ții punctele marcate cu * (mai multe unități în jur decât adversarul) și aduni 9 puncte sau ai mai multe după 10 runde. Încărcare: +1 zar după o mutare de cel puțin 2 pași. Flanc: +1 zar dacă un aliat e lângă țintă. C = încărcare. F = flanc. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Lords and Archers.
