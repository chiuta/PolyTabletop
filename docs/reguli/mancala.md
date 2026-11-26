# Mancala (Kalah) (`mancala`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mancala --seed 1`. Sursa implementării: `polytabletop-games/src/mancala.rs`.

> Rezumat din `polytabletop list`: 6 gropi și un depozit; semănatul ajunge în depozit = încă o mutare; groapă goală proprie = captură. Mutări: 1–6 (groapa proprie aleasă).

## Scop

Ai mai multe semințe în depozitul tău la sfârșit.

## Pregătire

Două rânduri de câte 6 gropi, câte un depozit pe jucător, 4 semințe în fiecare groapă.

## Desfășurarea turei

Alegi o groapă proprie nevidă și semeni semințele una câte una, în sens trigonometric, inclusiv în depozitul propriu (nu și în cel al adversarului).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul gropii proprii: `1`–`6`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (6): `1`, `2`, `3`, `4`, `5`, `6`. Introduci `1`; răspunsul calculatorului (în ordine): `5`.
2. Mutări legale (6): `1`, `2`, `3`, `4`, `5`, `6`. Introduci `4`; răspunsul calculatorului (în ordine): `3`; `4`.
3. Mutări legale (5): `1`, `2`, `3`, `5`, `6`. Introduci `6`; răspunsul calculatorului (în ordine): `5`; `4`.

## Reguli speciale și variante alese

* Ultima sămânță în depozitul propriu: mută din nou.
* Ultima sămânță într-o groapă proprie goală: capturezi groapa opusă.

## Punctaj

Semințele din depozite.

## Sfârșitul jocului

Când toate gropile unei părți sunt goale, semințele rămase se adună în depozitul părții respective; câștigă cine are mai multe în depozit.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 9.

Stil AI în cod: alpha-beta cu adâncime fixă 9.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Varianta Kalah (nu Oware).
