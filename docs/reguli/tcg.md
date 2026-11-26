# Cărți colecționabile (Ținutul Cenușiu) (`tcg`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tcg --seed 1`. Sursa implementării: `polytabletop-games/src/tcg.rs`.

> Rezumat din `polytabletop list`: Motor generic de cărți colecționabile cu 60 de cărți originale (resurse, creaturi, vrăji); pachete pregătite de 30 de cărți (J1 Cetatea, J2 Furtuna), 20 de viață, 5 cărți în mână. Tură: pescuire (forțată), faza principală, fin. O resursă pe tură (+1 mană). Creaturile noi nu atacă (Grabă = da); cuvinte-cheie: Zbor, Grabă, Armură, Venin, Primul, Străjer. Atac: ataca 1,3; apărătorul alege pe rând bloc N sau fara. Pachet gol la pescuit = pierdere. Informație ascunsă: vezi doar mâna J1. Mutări: pesca, joaca 2, joaca 3 e1 (țintă: e=inamic, m=al meu, fata), ataca 1,2, bloc 1, fara, fin.

## Scop

Aduci viața adversarului la 0 sau îl lași fără pachet (pachet gol la pescuit = pierdere).

## Pregătire

Fiecare primește un pachet pregătit de 30 de cărți (J1 „Cetatea”, J2 „Furtuna”) din setul original „Ținutul Cenușiu” (60 de cărți: resurse, creaturi, vrăji), 20 de viață și 5 cărți în mână. Mâna maximă este 7; cel mult 6 creaturi pe câmp.

## Desfășurarea turei

Pescuire (mutare forțată `pesca`; J1 nu pescuiește în prima tură), faza principală (joci cărți, ataci) și `fin`. O resursă (cost 0) pe tură; fiecare crește rezerva de mană cu 1 (maxim 10), care se reîncarcă la începutul turei tale.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `pesca`, `fin`.
* `joaca N` (cartea N din mână), cu țintă pentru vrăji: `joaca 3 e1` (e = creatură inamică, m = creatura mea, `fata` = jucătorul advers).
* `ataca 1,3` (declari atacatorii), apoi apărătorul: `bloc N` sau `fara`, pe rând pentru fiecare atacator.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `pesca`. Introduci `pesca`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (4): `fin`, `joaca 1`, `joaca 4`, `joaca 5`. Introduci `joaca 4`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `fin`. Introduci `fin`; răspunsul calculatorului (în ordine): `pesca`; `joaca 5`; `joaca 3`; `ataca 1`.

## Reguli speciale și variante alese

* Cuvinte-cheie: Zbor (blocat doar de zburători), Grabă (poate ataca imediat), Armură (-1 la fiecare lovitură), Venin (orice lovitură ucide), Primul (lovește înainte), Străjer (nu se înclină la atac).
* Creaturile proaspăt jucate nu pot ataca (fără Grabă). Neblocat, atacatorul lovește jucătorul. Daunele se șterg la sfârșitul turei.

## Punctaj

Viața jucătorilor (20 la start).

## Sfârșitul jocului

Viața unui jucător ajunge la 0, sau un jucător nu poate pescui.

## Note despre AI

Pachetele și mâinile sunt în stare; afișajul arată doar mâna lui J1. AI-ul (adâncime 1) vede doar câmpul public, mâna proprie (toate cărțile din mână se evaluează la fel) și numărul cărților adversarului. Pescuirea este mutare forțată.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Cărți și mecanici originale; nu reproduc niciun joc comercial.
