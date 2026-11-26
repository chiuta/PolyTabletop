# Table (Backgammon) (`backgammon`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play backgammon --seed 1`. Sursa implementării: `polytabletop-games/src/backgammon.rs`.

> Rezumat din `polytabletop list`: Două zaruri aruncate automat; 15 piese, bară, bear-off, dubla se joacă de 4 ori, fără cub de dublare. Un pas = un zar; trebuie jucate cât mai multe zaruri. Numerotarea e a jucătorului la mutare. Mutări: 13/7, bar/22, 3/off.

## Scop

Scoți primul toate cele 15 piese de pe tablă (bear-off).

## Pregătire

24 de puncte, 15 piese pe jucător în poziția standard, bară pentru piesele lovite. Numerotarea punctelor este mereu cea a jucătorului la mutare: 24 = punctul cel mai îndepărtat, 1 = ultimul punct din casă.

## Desfășurarea turei

Zarurile se aruncă automat imediat după fiecare tură și rezultatul apare în afișaj. Fiecare mutare este un pas cu un zar; dubla se joacă de patru ori. Dacă nu ai nicio mutare legală, tura trece automat.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Punct de plecare/punct de sosire: `13/7`.
* Din bară: `bar/22`.
* Scoatere din joc: `3/off`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `8/3`, `13/8`. Introduci `8/3`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (2): `8/3`, `13/8`. Introduci `13/8`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (2): `8/3`, `13/8`. Introduci `13/8`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Trebuie jucate cât mai multe zaruri; când se poate juca doar unul, îl joci pe cel mai mare.
* Fără cub de dublare și fără gammon/backgammon.

## Punctaj

Fără punctaj: primul care scoate cele 15 piese câștigă.

## Sfârșitul jocului

Când un jucător a scos toate piesele.

## Note despre AI

Zarurile sunt informație publică (nu se consideră informație privată), aruncate de joc cu generatorul propriu. AI-ul alege pasul cu cea mai bună evaluare (pip count, piese singure, puncte făcute) la adâncime 1 și nu „vede” zarurile viitoare. La același `--seed` jocul este reproductibil.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Fără cub de dublare și fără scor gammon/backgammon.
