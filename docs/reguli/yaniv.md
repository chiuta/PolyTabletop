# Yaniv (Israel) (`yaniv`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play yaniv --seed 1`. Sursa implementării: `polytabletop-games/src/yaniv.rs`.

> Rezumat din `polytabletop list`: Yaniv: arunci o carte, un set sau o secvență, apoi tragi din stoc sau talon; strigi «yaniv» cu cel mult 7 puncte în mână; dacă adversarul are mai puțin sau egal ești «asaf». Mutări: yaniv, 5h,5s-stoc, 5h-ia:7c.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Yaniv (Israel), 2 jucători, din regulile publice.


* 54 de cărți (cu 2 jokeri). Fiecare primește 5; restul = stoc, cu o carte întoarsă ca început al

  grămezii de aruncate. Valori: As 1, 2–10 cât scrie, J/Q/K 10, joker 0.

* La rândul tău: (1) dacă ai cel mult 7 puncte în mână poți spune **Yaniv** și mâna se termină; (2) altfel

  arunci un **set**: o carte, 2–4 cărți de același rang sau o **scară** de cel puțin 3 cărți succesive din

  aceeași culoare (As jos, K sus); jokerul ține loc de orice; (3) tragi o carte: din stoc sau una din

  **capetele** setului aruncat de adversar (la grupuri: oricare carte; la scări: prima sau ultima).

* Yaniv reușit (ai strict mai puțin decât adversarul): iei 0, el își adaugă punctele din mână. **Asaf**

  (adversarul are la fel sau mai puțin): tu iei 30 + punctele tale, el 0.

* Se joacă până cineva depășește 100 de puncte; câștigă cine are mai puține (egal = remiză). O mână

  care depășește 80 de ture se anulează.


Mutări: `yaniv`; `5h,5s-stoc`, `5h-ia:7c` (setul aruncat – cărți despărțite prin virgulă –, apoi de unde tragi);

`urmatoarea` pentru mâna următoare. Jokerul se scrie `Jk`. Informație ascunsă: vezi `p3_comun`.
