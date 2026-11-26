# Pente (`pente`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pente --seed 1`. Sursa implementării: `polytabletop-games/src/pente.rs`.

> Rezumat din `polytabletop list`: Pente (SUA) pe tabla 19×19: aliniezi 5 pietre sau capturezi 5 perechi (încadrezi exact două pietre adverse). Prima mutare a lui J1 trebuie să fie în centru, iar a doua piatră a lui J1 la cel puțin 3 intersecții de centru. Mutări: k10.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Pente (SUA), 2 jucători, din regulile publice.


* Tablă 19×19 (intersecții). Se pun pietre pe rând; J1 (X) începe, cu prima piatră obligatoriu în

  centru (k10). Regula de turneu: a doua piatră a lui J1 trebuie să fie la cel puțin trei

  intersecții de centru.

* Captură: două pietre adverse consecutive (exact două) pe o linie, încadrate la capete de pietrele

  tale, sunt scoase. Intrarea voluntară într-o poziție încadrată nu e captură.

* Câștigă cine face cinci sau mai multe pietre în linie sau cine capturează cinci perechi

  (zece pietre). Tabla plină sau 600 de mutări = remiză.

* Mutările legale sunt limitate la intersecțiile aflate la cel mult două pași de o piatră existentă

  (la fel ca la Gomoku): nu pierde nimic practic și ține AI-ul rapid.


Mutări: `k10` (coloane a–s fără i, rânduri 1–19).
