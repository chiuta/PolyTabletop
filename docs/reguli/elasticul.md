# Elasticul (`elasticul`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play elasticul --seed 1`. Sursa implementării: `polytabletop-games/src/elasticul.rs`.

> Rezumat din `polytabletop list`: Elasticul urcă în 7 niveluri; la fiecare nivel alegi salt ușor sau greu (șanse și puncte diferite), apoi sari; cine greșește e eliminat. Mutări: usor, greu, sari.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Elasticul (Chinese jump rope / „elastic”), 2 jucători — joc de sărituri pe niveluri.


* Elasticul urcă prin 7 niveluri (1 glezne, 2 genunchi, 3 coapse, 4 talie, 5 piept, 6 subsuori, 7 gât).

* La fiecare nivel sare întâi J1, apoi J2. Alegi `usor` (șanse 92% − 8%·nivel, câștigi nivelul în puncte)

  sau `greu` (șanse 80% − 10%·nivel, câștigi 2·nivel+1 puncte); apoi forțata `sari` aruncă zarul.

* Cine greșește e eliminat și nu mai sare. Jocul se termină când ambii sunt eliminați sau după nivelul 7;

  mai multe puncte câștigă, egal = remiză.


Mutări: `usor` | `greu`, apoi `sari`.
