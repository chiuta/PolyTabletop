# Echipa de aventurieri (`echipa_de_aventurieri`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play echipa_de_aventurieri --seed 1`. Sursa implementării: `polytabletop-games/src/zr_trei.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: alegi adâncimea 1–3 („nivel n”), apoi se aruncă 6 zaruri-aventurieri (luptător, mag, preot, hoț, erou, pergament) și n+1 monștri de 4 tipuri; relansezi până la 2 ori și potrivești monștrii. Fiecare monstru neînvins costă 1 viață; experiența = înfrânți × adâncime (+bonus). 6 ture. Mutări: nivel n, tira, relansez i,j, fin.

Detalii: Eroul și pergamentul sunt jokeri; monștrii se aruncă o singură dată.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Adventurer Party.
