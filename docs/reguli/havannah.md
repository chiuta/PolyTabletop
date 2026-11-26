# Havannah (`havannah`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play havannah --seed 1`. Sursa implementării: `polytabletop-games/src/havannah.rs`.

> Rezumat din `polytabletop list`: Havannah (Marea Britanie) pe tabla hexagonală cu latura 8: câștigi cu un inel, un pod (legi două colțuri) sau o furcă (legi trei laturi). Poți folosi «swap» după prima mutare. Mutări: h8, swap.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Havannah (Marea Britanie), 2 jucători, din regulile publice.


* Tablă hexagonală cu latura de 8 hexagoane (169 de câmpuri; varianta „baza 8”, uzuală online).

* Se pun pietre pe rând, J1 (X) începe. Regula plăcintei: după prima piatră a lui J1, J2 poate

  spune `swap` (preia piatra; J1 mută din nou).

* Câștigă primul care formează: un **inel** (lanț închis ce înconjoară cel puțin un câmp), un

  **pod** (un lanț care atinge două colțuri) sau o **furcă** (un lanț care atinge trei laturi

  diferite; colțurile nu sunt laturi). Tabla plină fără câștigător = remiză.


Mutări: `h8` (literă = coloană a–o, număr = rând 1–15 în tabla rombică), `swap`.
