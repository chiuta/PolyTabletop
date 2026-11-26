# Umbra Urbei (urmărire cu bilete) (`umbra`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play umbra --seed 1`. Sursa implementării: `polytabletop-games/src/umbra.rs`.

> Rezumat din `polytabletop list`: Variantă originală, J1 = 4 detectivi, J2 = Umbra (ascunsă; apare după mutările 4, 9, 14, 19). Hartă de 30 de stații pe jos/tram/metro cu bilete limitate; Umbra are 3 bilete negre. Mutări: d2 17 tram, x 12 jos. Detectivii câștigă prinzând sau blocând Umbra; Umbra, rezistând 22 de mutări. Se afișează stațiile posibile.

## Scop

J1 (patru detectivi) prinde sau blochează Umbra; J2 (Umbra) rezistă 22 de mutări.

## Pregătire

Hartă „Orașul Cenușiu” cu 30 de stații (grila 6×5), pe trei rețele: jos (străzi), tram (diagonale lungi) și metro (stațiile 2, 9, 16, 21, 29). Fiecare detectiv are 10 bilete jos, 8 tram, 4 metro. Umbra are 12 jos, 8 tram, 4 metro și 3 bilete negre (ascund tipul de transport). Umbra pornește dintr-una din 6 stații centrale (9, 10, 15, 16, 21, 22).

## Desfășurarea turei

J1 mută pe rând fiecare detectiv (un detectiv fără mutare e sărit), apoi J2 mută Umbra. Detectivii nu pot sta pe aceeași stație.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Detectiv: `d2 17 tram` (detectivul 2 pe stația 17 cu tramvaiul).
* Umbra: `x 12 jos`, `x 12 negru`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `d1 2 jos`, `d1 7 jos`, `d1 8 tram`. Introduci `d1 2 jos`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (2): `d2 5 jos`, `d2 12 jos`. Introduci `d2 12 jos`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (2): `d3 26 jos`, `d3 19 jos`. Introduci `d3 19 jos`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Umbra se arată după mutările 4, 9, 14 și 19; altfel se vede doar biletul folosit.
* Detectivii câștigă ajungând pe stația Umbrei sau blocând-o; Umbra câștigă dacă rezistă celor 22 de mutări sau dacă toți detectivii rămân fără mutări.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Prindere/blocare (detectivii) sau 22 de mutări / detectivi blocați (Umbra).

## Note despre AI

Poziția Umbrei este ținută în stare, dar afișajul o arată doar la reveniri și la final; în plus, afișajul arată mulțimea stațiilor posibile, calculată din biletele folosite și din revelări. AI-ul detectiv folosește doar această mulțime; AI-ul Umbră își știe poziția și vede detectivii.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără bilet dublu; biletele detectivilor nu merg la Umbră.
