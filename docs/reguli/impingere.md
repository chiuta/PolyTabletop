# Împingere pe pătrate (7×7) (`impingere`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Abalone”. Pornire: `polytabletop play impingere --seed 1`. Sursa implementării: `polytabletop-games/src/impingere.rs`.

> Reguli: Tablă 7×7, 10 bile pe jucător. Muți un grup de 1–3 bile aliniate cu un pas; pe lungime poți împinge 1 sau 2 bile adverse dacă grupul tău e mai numeros și dincolo e loc liber sau margine; lateral, toate destinațiile trebuie să fie libere. Câștigă cine scoate 5 bile; remiză după 100 de semi-mutări fără împingere. Mutări: a1>n, a1-c3>e (grup a1–c3 spre est).
