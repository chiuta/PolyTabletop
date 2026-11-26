# Drumul Prăjiturilor (cursă de culori) (`prajituri`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play prajituri --seed 1`. Sursa implementării: `polytabletop-games/src/prajituri.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 58 de pătrate colorate, pachet de cărți (simple, duble, locuri), scurtături și pătrate lipicioase. Cartea e afișată; singura mutare este trag. Primul în Palatul de zahăr câștigă.

## Scop

Ajungi primul în Palatul de zahăr (pătratul 57).

## Pregătire

Drum de 58 de pătrate (0 = start, 57 = Palatul de zahăr). Culoarea pătratului i este (i + i/6) % 6 (roșu, portocaliu, galben, verde, albastru, mov). Pachet de 39 de cărți: câte 4 simple și 2 duble pentru fiecare din cele 6 culori, plus 3 cărți „loc”.

## Desfășurarea turei

Cartea se trage automat și se vede în afișaj; singura mutare este `trag`. Carte simplă: avansezi la prima culoare identică după poziția curentă; dublă: la a doua. Dacă nu mai există, ajungi în Palat.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trag`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): `trag`.
2. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): `trag`.
3. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): `trag`.

## Reguli speciale și variante alese

* Cărți „loc”: Peștera de ciocolată → 18, Pădurea de bomboane → 33, Castelul de caramel → 46 (poate trimite și înapoi).
* Scurtături: 8→22, 25→41, 36→49. Pătrate lipicioase: 14, 31, 44 (pierzi următoarea tură).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Primul în Palatul de zahăr câștigă.

## Note despre AI

Nu există alegeri reale (o singură mutare), deci AI-ul nu poate „vedea” nimic ascuns. Jocul depinde doar de seed.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Mecanică originală („trage o carte de culoare și mergi la următorul pătrat de acea culoare”).
