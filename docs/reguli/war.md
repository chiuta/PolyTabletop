# Război (War) (`war`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play war --seed 1`. Sursa implementării: `polytabletop-games/src/war.rs`.

> Rezumat din `polytabletop list`: Război: fiecare întoarce cartea de sus, cea mai mare ia ambele; la egalitate «război» (cărți acoperite și una descoperită). Câștigă cine ia toate cărțile (limită de mutări, apoi mai multe cărți). Mutări: joc.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Războiul (War / Bătălia, global), 2 jucători, din regulile publice ale jocului de cărți.


* Pachet de 52 amestecat, 26 de cărți fiecăruia, cu fața în jos. La fiecare mutare ambii întorc cartea

  de sus; cea mai mare (2 < … < A) ia ambele cărți, care merg la baza teancului câștigătorului, într-o

  ordine aleatorie (ca să nu apară bucle).

* Egalitate = **război**: fiecare pune 3 cărți cu fața în jos și apoi una cu fața în sus; cea mai mare ia

  toată grămada; la egalitate războiul se repetă. Dacă cuiva nu-i ajung cărțile, pune ce are (ultima

  carte e cea cu fața în sus); dacă rămâne fără cărți, pierde.

* Câștigă cine are toate cărțile. Limita noastră: după 1000 de mâini câștigă cine are mai multe cărți.

* Nu există decizii: singura mutare este `joc` (AI-ul nu are ce alege).


Mutări: `joc`.
