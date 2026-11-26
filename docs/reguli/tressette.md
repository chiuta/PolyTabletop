# Tressette (Italia) (`tressette`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tressette --seed 1`. Sursa implementării: `polytabletop-games/src/tressette.rs`.

> Rezumat din `polytabletop list`: Tressette (40 de cărți italiene, 2 jucători): obligatoriu să răspunzi la culoare, ordinea 3 2 A R C F 7 6 5 4, fără atu; puncte A 1, 3/2/R/C/F ⅓; ultima levată +1. Mutări: 3d, Rs, Cc.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Tressette (Italia), 2 jucători, din regulile publice ale variantei „a due”.


* Pachet italian de 40 de cărți (denari, coppe, spade, bastoni). Ordinea (de la cea mai mare): 3, 2, As,

  Re, Cavallo, Fante, 7, 6, 5, 4. Nu există atu.

* Valori (în treimi de punct): As 3, iar 2, 3, Re, Cavallo, Fante câte 1; ultima levată +3 (adică 1 punct);

  restul 0. Un joc se ține până când un jucător adună 21 de puncte (63 de treimi) la sfârșitul unei mâini.

* Fiecare primește 10 cărți; restul (20) formează stocul. Câștigătorul unei levate trage primul din stoc,

  apoi celălalt, și conduce următoarea. **Culoarea se răspunde obligatoriu doar după ce stocul s-a golit**;

  până atunci poți arunca orice. Levata o ia cea mai mare carte din culoarea condusă.

* Nu sunt implementate acuzele (napoletana etc.) și nici punctajul în „zeci” pe tabelă.


Mutări: `3d`, `Rs`, `Cc` (rang: 3 2 A R C F 7 6 5 4; culoare: d denari, c coppe, s spade, b bastoni),

`urmatoarea` pentru mâna următoare. Informație ascunsă: vezi `p3_comun`.
