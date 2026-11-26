# Cassino (capturi) (`casino`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play casino --seed 1`. Sursa implementării: `polytabletop-games/src/casino.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 4 pe masă, 4 în mână. Lași o carte (5h) sau capturezi (5h x 2c 3d): grupuri de cărți identice sau cu suma egală cu a ta (A=1, J/Q/K doar identice). Puncte pe împărțire: cărți 3, pici 1, 10♦ 2, 2♠ 1, ași 1, mături 1; se joacă până la 11. Fără construiri. Informație ascunsă: vezi doar mâna ta.

## Scop

Ajungi primul la 11 puncte (pe împărțiri).

## Pregătire

52 de cărți, 4 pe masă, 4 în mână; când mâinile se golesc se împart încă 4 (6 ture pe împărțire). Valori: A = 1, 2–10 cât scriu, J = 11, Q = 12, K = 13.

## Desfășurarea turei

Pui o carte din mână: o lași pe masă (`5h`) sau capturezi (`5h x 2c 3d`): cartea ta ia orice grupuri de cărți de pe masă, fiecare grup fiind o carte de aceeași valoare ori un set cu suma egală cu valoarea cărții tale (doar pentru A–10; J, Q, K iau doar cărți identice). Cartea jucată merge și ea în grămada ta.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Lași: `5h`.
* Capturezi: `5h x 2c 3d`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (6): `6c`, `6c x 6d`, `Ks`, `8c`, `8c x 2d 6d`, `Kc`. Introduci `6c`; răspunsul calculatorului (în ordine): `5s x 2d 3d`.
2. Mutări legale (3): `Ks`, `8c`, `Kc`. Introduci `8c`; răspunsul calculatorului (în ordine): `6h x 6d 6c`.
3. Mutări legale (2): `Ks`, `Kc`. Introduci `Kc`; răspunsul calculatorului (în ordine): `4h`.

## Reguli speciale și variante alese

* Fără construiri (builds) și fără dubla valoare a asului.
* Măturare (masa rămâne goală după captură) = 1 punct. La finalul împărțirii, cine a capturat ultimul ia ce a rămas pe masă (fără punct de măturare).

## Punctaj

Pe împărțire: cele mai multe cărți 3 (egal: 0), cele mai multe pici 1, 10♦ „Cassino mare” 2, 2♠ „Cassino mic” 1, fiecare as 1, fiecare măturare 1.

## Sfârșitul jocului

Se joacă împărțiri până cineva are cel puțin 11 puncte (cel mai mare scor câștigă; egal = încă o împărțire; maxim 8 împărțiri).

## Note despre AI

Informație ascunsă: vezi doar mâna ta. AI-ul evaluează doar grămezile capturate, masa și scorurile (publice); nu citește mâna adversarului și nici stocul.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără construiri și fără dubla valoare a asului.
