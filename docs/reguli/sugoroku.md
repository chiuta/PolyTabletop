# Sugoroku (cursă cu casete) (`sugoroku`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sugoroku --seed 1`. Sursa implementării: `polytabletop-games/src/sugoroku.rs`.

> Rezumat din `polytabletop list`: Drum 0–40, două zaruri (arunc), alegi un zar sau suma (avansez N). Atingi exact 40 ca să câștigi (surplusul te întoarce). Casete: 4 +3, 9 −3, 12 și 28 pierzi tura, 15 și 36 încă o aruncare, 18 +5, 21 schimbi locul cu adversarul, 25 −6, 30 +4, 33 −8, 38 −10 (fără lanț). Mutări: arunc, avansez 7.

## Scop

Ajungi exact pe pătratul 40.

## Pregătire

Drum de 40 de pași (0 = start, 40 = final). Două zaruri.

## Desfășurarea turei

`arunc` (forțată) aruncă cele două zaruri; apoi alegi un zar, celălalt sau suma cu `avansez N`. Aceasta este singura decizie reală.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`.
* `avansez 3`, `avansez 5`, `avansez 8` (zar, zar sau sumă).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (3): `avansez 3`, `avansez 5`, `avansez 8`. Introduci `avansez 5`; răspunsul calculatorului (în ordine): `arunc`; `avansez 10`.
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Dacă treci peste 40, te întorci pe același număr de pași.
* Casete (o singură dată, fără lanț): 4 → +3, 9 → −3, 12 = pierzi tura, 15 = mai arunci, 18 → +5, 21 = schimbi locul cu adversarul, 25 → −6, 28 = pierzi tura, 30 → +4, 33 → −8, 36 = mai arunci, 38 → −10.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Câștigă cine ajunge primul exact pe 40.

## Note despre AI

Zarurile sunt aruncate în stare cu generator propriu; AI-ul nu vede aruncările viitoare și ia o decizie reală la alegerea zarului/sumei.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Variantă proprie de cursă pe casete, nu sugoroku tradițional cu poduri.
