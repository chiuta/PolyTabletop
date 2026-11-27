# Case și ierni (`case_si_ierni`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play case_si_ierni --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 8×8. Unități (hp/mișcare/rază/zaruri-forță/armură): c = Cavaler 4/4/1/3/2 [C]; l = Lăncier 2/3/1/2/1 [F]; a = Arcaș 1/3/4/2/0; b = Bastard 3/3/1/3/1. Pe tură poți activa 3 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul aruncă zaruri în mutarea forțată „tira”: fiecare zar peste armură rănește cu 1. Scop: la sfârșitul turei ții punctele marcate cu * (mai multe unități în jur decât adversarul) și aduni 10 puncte sau ai mai multe după 9 runde. Încărcare: +1 zar după o mutare de cel puțin 2 pași. Flanc: +1 zar dacă un aliat e lângă țintă. C = încărcare. F = flanc. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Houses and Winters.
