# Purcei pe masă (`purcei_pe_masa`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play purcei_pe_masa --seed 1`. Sursa implementării: `polytabletop-games/src/zr_risc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: arunci doi purcei (poziții: stânga, dreapta, spate, picioare, urechi). Doi pe aceeași parte = 1 punct; pe părți opuse = pierzi punctele turei; altfel spate 5, picioare 10, urechi 15 (suma, dublă dacă sunt la fel). Continui sau încasezi; primul la 50 câștigă. Mutări: tira, arunc, opresc.

Detalii: Probabilitățile pozițiilor (35/30/20/10/5) sunt publice; pragul rațional de oprire e în jur de 18–20 de puncte în tură.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Pigs on the Table.
