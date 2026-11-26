# Prinde coada măgarului (`pin_the_tail`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pin_the_tail --seed 1`. Sursa implementării: `polytabletop-games/src/pin_the_tail.rs`.

> Rezumat din `polytabletop list`: Pe un câmp 7×5 alegi o țintă, te învârți (eroare aleatoare) și prinzi coada: 5 puncte exact, 3, 2, 1 după distanță; 5 runde. Mutări: c3, apoi invart.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Prinde coada măgarului (Pin the Tail on the Donkey, SUA), 2 jucători — joc de petrecere simulat.


* Pe un câmp 7×5 (coloane a–g, rânduri 1–5) coada trebuie prinsă la locul ei, marcat cu `T` și

  schimbat în fiecare rundă. Cel cu ochii legați își alege direcția (celula spre care merge), dar

  se învârte: pe fiecare axă greșește cu 0 (50%), ±1 (20% fiecare) sau ±2 (5% fiecare).

* Puncte: aterizare exactă 5; la 1 pătrat (pe orice axă, diagonala contează) 3; la 2 pătrate 2; la 3 pătrate 1;

  mai departe 0. O aterizare în afara câmpului se oprește la marginea lui.

* 5 runde pentru fiecare; câștigă cine are mai multe puncte (egal = remiză).

* Mutarea se face în doi pași: alegi celula-țintă (`c3`), apoi forțata `invart`, ca AI-ul să nu vadă

  rezultatul aleator înainte să aleagă.


Mutări: `c3`, apoi `invart`.
