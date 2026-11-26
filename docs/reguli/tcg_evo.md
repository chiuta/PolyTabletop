# Cărți colecționabile: evoluție și energie (Făpturi de Câmp) (`tcg_evo`)

Cărți colecționabile cu evoluție și energie („Seria Făpturilor de Câmp”), 2 jucători, set original de 40+ cărți.

## Scop

Iei toate cele 3 cărți-premiu, lași adversarul fără făpturi sau îl lași fără pachet.

## Pregătire

Pachet pregătit de 30 de cărți (J1 „Pădurea”, J2 „Vulcanul”), 5 cărți în mână (se reamestecă automat până apare o făptură de bază), 3 cărți-premiu. Pregătire: `activ N` (făptura activă), `banca N`, `gata`.

## Desfășurarea unei ture

`pesca` (forțat; pachet gol = pierdere), apoi oricâte dintre: `banca N`, `evolueaza N T` (nu în prima tură și nu pe o făptură nouă/evoluată în tura asta), `energie N T` (una pe tură), `joaca N [T]` (ajutoare; un singur „sprijin” pe tură), `retragere bK` (o dată pe tură; se aruncă energii cât costul), apoi `atac 1|2` (încheie tura) sau `fin`. J1 nu atacă în tura 1.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `activ 5` — pui cartea 5 din mână ca făptură activă
* `banca 4` — pui cartea 4 pe bancă
* `gata` — termini pregătirea
* `pesca` — tragi o carte (forțat)
* `evolueaza N T` — evoluezi
* `energie N T` — atașezi energia N făpturii T
* `joaca N [T]` — joci un ajutor, cu țintă opțională
* `retragere bK` — retragi activa, înlocuind-o cu bK
* `atac 1 / atac 2` — atacul 1 sau 2
* `promova bK` — după eliminare, promovezi făptura bK
* `fin` — încheie tura

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tcg_evo --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `activ 5` — pui Pietricel (cartea 5) ca activ.
2. `banca 4` — pui Mugurel (cartea 4) pe bancă.
3. `banca 1` — pui Mugurel (cartea 1) pe bancă; urmează `gata`.

## Reguli speciale și variante alese

Ținte: `a` (activa mea), `b1..b5` (banca mea), `e1..e5` (banca adversă). Slăbiciune: daunele se dublează dacă elementul atacatorului e cel de care e slabă ținta (Foc>Frunză, Apă>Foc, Fulger>Apă, Piatră>Fulger și Obișnuit, Frunză>Piatră). Efecte: vindecare, daune la bancă, recul, aruncare de energie, pescuire, „Amețit”.

## Punctaj

Cărțile-premiu: 3; fiecare făptură activă eliminată dă adversarului un premiu.

## Sfârșitul jocului

Câștigă cine ia toate premiile, cine lasă adversarul fără făpturi, sau cine nu rămâne fără pachet.

## Note despre AI și informația ascunsă

`render` arată doar mâna J1. AI-ul (alpha-beta adâncime 1) citește doar informația vizibilă (câmp public, mâna proprie, NUMĂRUL cărților adverse); atacurile disponibile sunt creditate ca potențial.

## Diferențe față de regulile tradiționale

Set original de cărți, reguli „scrise de noi” din mecanici generice; nu replică un joc existent.
