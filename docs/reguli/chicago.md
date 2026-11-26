# Chicago (zaruri) (`chicago`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chicago --seed 1`. Sursa implementării: `polytabletop-games/src/chicago.rs`.

> Rezumat din `polytabletop list`: Chicago: 11 runde cu ținta 2…12; aruncând două zaruri cu suma egală cu ținta iei atâtea puncte; opțional păstrezi un zar și rearunci. Mutări: arunc, tin 1, tin 2, gata, rearunc.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Chicago (zaruri, Europa/SUA), 2 jucători, din regulile publice.


* 11 runde; în runda `r` ținta este `r + 1` (2, 3, …, 12). La rândul tău arunci două zaruri (`arunc`); dacă

  suma e egală cu ținta, ia **atâtea puncte cât e ținta** (nu se dublează), altfel 0.

* Variantă „rearuncare” (Double Roll): dacă prima aruncare nu a nimerit ținta poți **păstra un zar** (`tin 1`

  sau `tin 2`, cel din stânga/dreapta) și rearunci celălalt o singură dată, sau renunți (`gata`).

* Aruncările sunt mutări forțate (`arunc`, `rearunc`), deci AI-ul nu cunoaște rezultatul înainte să aleagă.

  Câștigă cine are mai multe puncte după 11 runde.


Mutări: `arunc`, `tin 1`, `tin 2`, `gata`, `rearunc`.
