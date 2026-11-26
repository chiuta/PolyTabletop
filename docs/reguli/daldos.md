# Daldøs (`daldos`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play daldos --seed 1`. Sursa implementării: `polytabletop-games/src/daldos.rs`.

> Rezumat din `polytabletop list`: Daldøs (reconstituire): tablă-barcă cu 3 rânduri, 12 piese, două zaruri cu 4 fețe; un 1 «dalează» o piesă, apoi piesele parcurg circuitul și scot piesele adverse. Mutări: dal, h5+3.

`HOTSEAT_SAFE`: nu (fără informație privată).

## Reguli implementate (varianta aleasă)

Daldøs (Norvegia), 2 jucători — reconstituire din regulile publice.


* Tablă în formă de barcă cu trei rânduri: rândul fiecărui jucător (12 găuri) și rândul din mijloc (13).

  Fiecare are 12 piese în picioare („nedalate”) în rândul său.

* Două zaruri cu patru fețe (1 = „dal”, 2, 3, 4). Cu un 1 poți **dala** piesa nedalată cea mai apropiată de

  pupă: se culcă și avansează un pas (prima dală intră în rândul din mijloc). Piesele dalate parcurg: rândul

  propriu spre pupă → rândul din mijloc spre prova → rândul adversarului de la prova spre pupă → apoi din nou

  rândul din mijloc și tot așa, în cerc; nu se întorc în rândul propriu.

* Fiecare zar mută o piesă dalată; poți folosi ambele zaruri pe aceeași piesă (aterizarea de la primul zar

  capturează și ea, variantă dată de unele surse). Nu poți sări sau ateriza peste piese proprii; piesele

  adversarului se pot sări, iar o piesă adversă din gaura unde ajungi (inclusiv una „în picioare” din rândul

  lui) este scoasă. Trebuie să folosești ambele zaruri dacă se poate, apoi unul; altfel pierzi mutarea.

  Două dale (1 și 1) dau o aruncare în plus.

* Câștigă cine rămâne cu adversarul fără piese (după 600 de ture câștigă cine are mai multe piese).


Mutări: `dal` (dalezi piesa de pe pupă), `h5+3` (piesa dalată din rândul propriu, gaura 5, cu zarul 3),

`c7+2` (piesă pe circuit, poziția 7). Rândul propriu `h0`–`h11` (h0 la prova), circuitul `c0`–`c24` (c0–c12 =

mijloc de la pupă la prova, c13–c24 = rândul adversarului de la prova la pupă).
