# Congkak (Malaezia) (`congkak`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play congkak --seed 1`. Sursa implementării: `polytabletop-games/src/congkak.rs`.

> Rezumat din `polytabletop list`: 2 × 7 case, 7 semințe în fiecare, depozit pe jucător; semănat trigonometric, depozitul adversarului se sare. Ultima în depozit = încă o mutare; în casă ocupată = ridici și continui; în casă goală proprie = capturezi casa opusă plus sămânța ultimă (opusa goală: doar sămânța ultimă); în casă goală adversă = tura se termină. Runda se încheie când jucătorul la mutare nu mai are semințe; apoi casele se reumplu din depozite (7 fiecare), cele nerefăcute sunt arse. Cine nu mai poate umple nicio casă pierde. Mutări pe rând, nu simultan. Mutări: 1–7.

## Scop

Aduni semințe în depozitul propriu; pierde cine nu mai poate umple nicio casă la reumplere.

## Pregătire

2 × 7 case, câte un depozit („rumah”) pe jucător, 7 semințe în fiecare casă (98 în total). Jucătorul 1 începe prima rundă, apoi începe cel cu mai puține semințe în depozit.

## Desfășurarea turei

Alegi o casă proprie nevidă și semeni în sens trigonometric (pe partea proprie de la stânga la dreapta, depozitul propriu, apoi partea adversă), sărind depozitul adversarului.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul casei proprii: `1`–`7`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (7): `1`, `2`, `3`, `4`, `5`, `6`, `7`. Introduci `1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (6): `2`, `3`, `4`, `5`, `6`, `7`. Introduci `5`; răspunsul calculatorului (în ordine): `5`; `3`; `1`; `5`; `5`; `6`; `6`; `7`; `7`; `1`; `3`; `2`; `3`; `7`; `1`; `3`; `7`; `1`.
3. Mutări legale (3): `2`, `3`, `5`. Introduci `5`; răspunsul calculatorului (în ordine): `3`.

## Reguli speciale și variante alese

* Ultima sămânță în depozitul propriu: încă o mutare.
* Ultima sămânță într-o casă ocupată (de oricare parte): ridici toate semințele ei și continui.
* Ultima sămânță într-o casă goală proprie: ea și casa opusă (dacă are semințe) merg în depozit; dacă opusa e goală, doar sămânța ultimă.
* Ultima sămânță într-o casă goală a adversarului: tura se termină.
* Runda se termină când jucătorul la mutare nu mai are semințe; apoi casele se reumplu din depozit, de la stânga la dreapta, câte 7; casele care nu se pot umple sunt „arse” (nu se mai folosesc).

## Punctaj

Semințele din depozite.

## Sfârșitul jocului

Cine nu poate umple nicio casă pierde meciul; după 40 de runde meciul se decide după depozite.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 6.

Stil AI în cod: alpha-beta cu adâncime fixă 6.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Jucătorii mută pe rând, nu simultan (în original pornesc simultan, iar în runda a doua începe perdantul).
* Casele arse nu se pot reface; un semănat în lanț e limitat la 2000 de semințe.
