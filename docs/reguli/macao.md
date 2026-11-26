# Macao (`macao`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play macao --seed 1`. Sursa implementării: `polytabletop-games/src/macao.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 5 în mână: culoare sau valoare. 2 / 3 se adună (taie cu alt 2 / 3, altfel tragi); 4 sare peste rând; 7 merge peste orice și alege culoarea (7h:s); As cere culoarea (Ah:d). Fără carte bună tragi una. Câștigă cine rămâne fără cărți (nu cu carte specială). Mutări: 9h, trage, sar, pas.

## Scop

Rămâi primul fără cărți în mână.

## Pregătire

Pachet de 52 de cărți. Fiecare primește 5 cărți; o carte obișnuită este întoarsă pe masă. Jucătorul 1 începe.

## Desfășurarea turei

Joci o carte de aceeași culoare sau valoare ca ultima carte de pe masă (sau culoarea cerută). Dacă nu ai carte potrivită, tragi una din stoc și o poți juca dacă se potrivește; altfel spui `pas`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Carte: `9h`, `Kd` (valoare + culoare h/d/s/c).
* 7 sau As cu culoarea cerută: `7h:s`, `Ah:d` (h inimă roșie, d romb, s inimă neagră, c treflă).
* `trage` (o carte), `trage N` (penalizarea acumulată), `sar` (suporți un 4), `pas`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `5s`. Introduci `5s`; răspunsul calculatorului (în ordine): `Ks`.
2. Mutări legale (1): `trage`. Introduci `trage`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `pas`. Introduci `pas`; răspunsul calculatorului (în ordine): `Kc`.

## Reguli speciale și variante alese

* 2 și 3: adversarul trage 2/3 cărți dacă nu taie cu alt 2 sau 3; penalizările se adună (2+3 = 5).
* 4: adversarul sare peste rând dacă nu pune și el un 4 (`sar`).
* 7: se joacă peste orice și alege culoarea.
* As: se joacă pe aceeași culoare sau peste alt As și cere o culoare.
* Nu poți încheia cu o carte specială (2, 3, 4, 7, As): trebuie să tragi.
* Stocul gol se reface din cărțile jucate (fără ultima).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine rămâne fără cărți câștigă. Siguranță: după 1000 de mutări câștigă cine are mai puține cărți (egal = remiză).

## Note despre AI

Informație ascunsă (stoc și mâna adversarului). Vezi doar mâna ta. AI-ul este alpha-beta de adâncime 1 pe o evaluare care citește doar informația vizibilă. Trasul de carte este automat din stoc, amestecat cu generatorul propriu.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Regulile macao variază mult. Neimplementate: As care sare peste rând, Valetul care cere culoare, Popa de roșu +5, jokeri, 4 anulează penalizarea 2/3, 7 anulează penalizarea, strigarea „Macao!” cu penalizare de 5 cărți.
