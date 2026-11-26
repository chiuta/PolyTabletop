# Oina (`oina`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oina --seed 1`. Sursa implementării: `polytabletop-games/src/oina.rs`.

> Rezumat din `polytabletop list`: Oina stilizată: 6 reprize cu 3 arși; bătăușul alege lovitura, apărătorul poziția, apoi se aruncă; mingea prinsă în zona aleasă arde bătăușul, altfel aleargă și marchează 1/2/4. Mutări: usoara, medie, puternica, aproape, mijloc, departe, arunca.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Oina (jocul românesc cu bâta și mingea), 2 jucători — adaptare simplificată în reprize.


Oina reală se joacă în două echipe de 11 pe un teren cu linie de start, linie de var și tarc; aici

o versiune stilizată (variantă aleasă de noi): 3 reprize pentru fiecare, cu rolurile alternând;

o repriză durează până la 3 «arși» (ieșiri).

* Bătăușul alege lovitura: `usoara` (greșește 10%), `medie` (25%) sau `puternica` (45%).

* Apoi apărătorul alege unde stă: `aproape`, `mijloc` sau `departe`.

* `arunca` rezolvă: dacă bătăușul a greșit, e ars. Altfel mingea cade în zona dată de forța loviturii

  (ușoară: aproape 70% / mijloc 30%; medie: aproape 25% / mijloc 55% / departe 20%; puternică:

  mijloc 25% / departe 75%). Dacă apărătorul stă exact în zona mingii o prinde din zbor și bătăușul e ars;

  altfel bătăușul aleargă: aproape = 1 punct, mijloc = 2, departe = 4 (bătăușul trece prin tarc și înapoi).

* Câștigă cine are mai multe puncte după 6 reprize, egal = remiză.


Mutări: `usoara`/`medie`/`puternica`, apoi `aproape`/`mijloc`/`departe`, apoi `arunca`.
