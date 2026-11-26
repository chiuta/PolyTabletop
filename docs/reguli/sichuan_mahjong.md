# Mahjong Sichuan (`sichuan_mahjong`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sichuan_mahjong --seed 1`. Sursa implementării: `polytabletop-games/src/mj2.rs`.

> Rezumat din `polytabletop list`: Mahjong Sichuan pentru 2 jucători: 108 piese (fără onoruri), fiecare declară o culoare lipsă (lipsa m|s|p) și o aruncă întâi; fără chi, doar pung, kong și câștig; jocul se oprește la primul câștig. Mutări: lipsa m, trage, d5m, pung, kong, mahjong, pas.

`HOTSEAT_SAFE`: da.

## Reguli implementate (varianta aleasă)

Motor comun pentru Mahjong Sichuan și Mahjong oficial chinez (Guobiao), versiuni pentru 2 jucători.


* **Sichuan** (`Variant::Sichuan`): 108 piese (trei culori × 9 × 4; fără onoruri și fără flori). Fiecare

  jucător declară la început **culoarea lipsă** (`lipsa m|s|p`) și trebuie să arunce întâi piesele din ea;

  o mână cu piese din culoarea lipsă nu poate câștiga. Nu se poate lua o piesă pentru secvență (fără chi);

  pung, kong și câștig sunt permise. Mână câștigătoare: 4 seturi + o pereche sau șapte perechi; fără minimum de

  fani (se afișează totuși: tragere proprie +1, toate pungurile +1, o singură culoare +2, șapte perechi +2).

  În original jocul continuă după primul câștig („bătălia sângeroasă”); aici se încheie la primul câștig.

* **Guobiao** (`Variant::Guobiao`): 144 de piese (cu 8 flori, înlocuite automat din zidul mort). Mâna

  câștigătoare trebuie să aibă **cel puțin 8 puncte** fără flori (fiecare floare +1 peste). Implementăm un

  subset documentat al celor 81 de combinații oficiale: Patru vânturi mari 88, Trei dragoni mari 88, Doar

  onoruri 64, Doar capete 64, Patru pungs ascunse 64, Trei dragoni mici 64, Capete și onoruri 32, Șapte perechi

  24, Culoare întreagă 24, Pungs pare 24, Scară pură 16, Trei pungs ascunse 16, Toate cincii 16, Triplu pung 16,

  Jumătate de culoare 6, Toate pungurile 6, Scară mixtă 8, Triplă scară mixtă 8, Pung dragon 2, Vânt dominant 2,

  Vânt propriu 2, Toate secvențele 2, Mână ascunsă 2 (4 dacă și din tragere), Toate simple 2, Tragere proprie 1,

  Kong expus 1, Kong ascuns 2, Fără onoruri 1, Pung de capete/onoruri 1, Mână găină 8 (nicio altă combinație).

  Restul combinațiilor oficiale (șiruri duble, nouă porți etc.) nu sunt implementate.

* Ambele: tragi (`trage`), arunci (`d5m`) sau declari `mahjong`; adversarul poate lua piesa aruncată (`pung`,

  `kong`, în Guobiao și `chi2m3m`, apoi arunci) sau `mahjong`. Zidul e ascuns; mână remiză la zid terminat.


Mutări: `lipsa m`, `trage`, `trage-moarta`, `d5m`, `mahjong`, `kong5m`, `pung`, `kong`, `chi2m3m`, `pas`.
