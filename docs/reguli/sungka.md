# Sungka (Filipine) (`sungka`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sungka --seed 1`. Sursa implementării: `polytabletop-games/src/sungka.rs`.

> Rezumat din `polytabletop list`: 7 gropi și un depozit pe jucător, 7 semințe în fiecare groapă. Semănat în sens trigonometric prin depozitul propriu, sărind depozitul adversarului. Ultima sămânță în depozit = încă o mutare; în groapă ocupată = ridici și continui; în groapă goală proprie = capturezi groapa opusă; în groapă goală adversă = tura se termină. O singură rundă, fără gropi „arse”, mutări pe rând; câștigă cine are peste 49 în depozit sau mai mult la final. Mutări: 1–7 (groapa proprie, de la stânga la dreapta).

## Scop

Ai peste 49 de semințe în depozitul propriu sau mai multe la final.

## Pregătire

2 × 7 gropi și câte un depozit („casă”) pe jucător, 7 semințe în fiecare groapă (98 în total). Jucătorul 1 începe.

## Desfășurarea turei

Alegi o groapă proprie nevidă și semeni în sens trigonometric: pe partea proprie de la stânga la dreapta, depozitul propriu, apoi partea adversă, sărind depozitul adversarului.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul gropii proprii, de la stânga la dreapta: `1`–`7`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (7): `1`, `2`, `3`, `4`, `5`, `6`, `7`. Introduci `1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (6): `2`, `3`, `4`, `5`, `6`, `7`. Introduci `5`; răspunsul calculatorului (în ordine): `5`; `3`; `1`; `5`; `5`; `6`; `1`; `2`.
3. (`--seed 2`) Mutări legale (7): `1`, `2`, `3`, `4`, `5`, `6`, `7`. Introduci `1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Ultima sămânță în depozitul propriu: încă o mutare.
* Ultima sămânță într-o groapă ocupată (de oricare parte): ridici toate semințele ei și semănatul continuă.
* Ultima sămânță într-o groapă goală proprie: capturezi groapa opusă (dacă are semințe) împreună cu sămânța ultimă.
* Ultima sămânță într-o groapă goală adversă: tura se termină.

## Punctaj

Semințele din depozite.

## Sfârșitul jocului

Jocul se termină când jucătorul la mutare nu mai are semințe (adversarul își adaugă semințele rămase pe partea lui) sau când cineva are peste 49 în depozit.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 8. Lanțurile de ture suplimentare sunt lungi: calculatorul poate juca mai multe mutări la rând, iar partida se poate decide în foarte puține mutări ale tale (un exemplu de mai sus provine dintr-o a doua partidă, cu `--seed 2`).

Stil AI în cod: alpha-beta cu adâncime fixă 8.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* O singură rundă (fără gropi „arse” și fără runda a doua; acestea sunt implementate doar la Congkak).
* Jucătorii mută pe rând, nu simultan.
* Un semănat în lanț e limitat la 2000 de semințe puse.
