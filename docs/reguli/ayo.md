# Ayo (Ayoayo, Yoruba) (`ayo`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ayo --seed 1`. Sursa implementării: `polytabletop-games/src/ayo.rs`.

> Rezumat din `polytabletop list`: 2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric cu semănat în lanț (ultima într-o groapă ocupată = continui) și groapa de start sărită. Captură: ultima sămânță într-o groapă goală proprie ia groapa opusă (fără sămânța ultimă). Dacă adversarul nu mai are semințe, ești obligat să-l hrănești; dacă nu poți, jocul se termină. Mutări: 1–6.

## Scop

Capturezi mai multe semințe decât adversarul.

## Pregătire

2 × 6 gropi, 4 semințe în fiecare (48 în total).

## Desfășurarea turei

Alegi o groapă proprie nevidă și semeni în sens trigonometric, câte o sămânță în fiecare groapă; groapa din care semeni se sare. Dacă ultima sămânță cade într-o groapă care avea deja semințe, le ridici pe toate și semeni mai departe (lanț), până când ultima cade într-o groapă goală.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul gropii proprii: `1`–`6`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (6): `1`, `2`, `3`, `4`, `5`, `6`. Introduci `1`; răspunsul calculatorului (în ordine): `1`.
2. Mutări legale (5): `1`, `2`, `3`, `5`, `6`. Introduci `3`; răspunsul calculatorului (în ordine): `4`.
3. Mutări legale (4): `2`, `3`, `4`, `5`. Introduci `5`; răspunsul calculatorului (în ordine): `6`.

## Reguli speciale și variante alese

* Captură: ultima sămânță într-o groapă goală proprie ia groapa opusă (fără sămânța ultimă). Pe partea adversă nu se capturează.
* Hrănire: dacă adversarul nu mai are semințe, ești obligat să muți astfel încât să-i dai semințe; dacă nu poți, jocul se termină și îți păstrezi semințele de pe partea ta.

## Punctaj

Semințele capturate.

## Sfârșitul jocului

Dacă la mutare nu mai ai semințe, jocul se termină și adversarul ia ce a rămas pe partea lui. După 500 de mutări fiecare ia semințele de pe partea lui.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 7.

Stil AI în cod: alpha-beta cu adâncime fixă 7.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Penalizarea pentru „nehrănire” nu există (mutarea de hrănire e pur și simplu obligatorie).
* Lanțul de semănat e limitat la 2000 de semințe.
