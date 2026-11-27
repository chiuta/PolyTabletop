# Coridorul încercărilor (`coridorul_incercarilor`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play coridorul_incercarilor --seed 1`. Sursa implementării: `polytabletop-games/src/zr_arunc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: cinci monștri (viață 3,4,5,4,6) pe un coridor comun. Pe rând lovești: usor (85%, 1), mediu (60%, 2), greu (35%, 3; ratarea costă 1 punct), apoi tira. Cine dă lovitura finală ia recompensa egală cu viața inițială. La final contează punctele. Mutări: usor, mediu, greu, tira.

Detalii: Tensiune: lovitura care lasă monstrul aproape mort îi oferă adversarului ocazia să ia recompensa.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Corridor of Trials.
