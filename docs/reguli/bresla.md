# Bresla (variantă originală) (`bresla`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bresla --seed 1`. Sursa implementării: `polytabletop-games/src/bresla.rs`.

> Rezumat din `polytabletop list`: Construire de pachet pentru doi: 7 Bani + 3 Parcele, mână de 5. Tură: acțiune (joc sat), cumparaturi (comorile se numără automat), cumpar <carte>, gata. 14 cărți: bani/argint/aur, parcelă/moșie/domeniu (1/3/6 p), acțiuni Sat, Meșteșug, Târg, Sărbătoare, Tăietor, Laborator, Atelier (iau <carte>), Cămătar (adversarul aruncă la 3 cărți: dau <carte>). Sfârșit: Domeniile epuizate sau 3 rezerve goale; câștigă cine are mai multe puncte. Informație ascunsă: ordinea pachetului și mâna adversarului.

## Scop

Ai cele mai multe puncte în tot pachetul la final.

## Pregătire

Fiecare începe cu 10 cărți (7 Bani, 3 Parcele), amestecate; tragi 5. Rezerva comună are 14 tipuri de cărți.

## Desfășurarea turei

Faza de acțiune (1 acțiune) și faza de cumpărare (1 cumpărare); apoi arunci mâna și ce ai jucat și tragi 5 cărți noi (aruncatele se reamestecă când pachetul se golește).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `joc <carte>` (de exemplu `joc sat`), `cumparaturi` (încheie acțiunile; comorile din mână se numără automat).
* `cumpar <carte>`, `gata` (încheie tura).
* Efecte: `iau <carte>` (Atelier), `dau <carte>` (Cămătar).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `cumparaturi`. Introduci `cumparaturi`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (8): `cumpar argint`, `cumpar parcela`, `cumpar sat`, `cumpar mestesug`, `cumpar taietor`, `cumpar atelier`, `cumpar camatar`, `gata`. Introduci `cumpar taietor`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `gata`. Introduci `gata`; răspunsul calculatorului (în ordine): `cumparaturi`; `cumpar argint`; `gata`.

## Reguli speciale și variante alese

* Comori: Ban (1, cost 0), Argint (2, cost 3), Aur (3, cost 6). Puncte: Parcelă (1p, cost 2), Moșie (3p, cost 5), Domeniu (6p, cost 8).
* Acțiuni: Sat (3): +1 carte, +2 acțiuni; Meșteșug (4): +3 cărți; Târg (5): +1 carte, +1 acțiune, +1 cumpărare, +1 monedă; Sărbătoare (5): +2 acțiuni, +1 cumpărare, +2 monede; Tăietor (3): +1 cumpărare, +2 monede; Laborator (5): +2 cărți, +1 acțiune; Atelier (3): primești o carte cu cost până la 4; Cămătar (4): +2 monede, adversarul aruncă până rămâne cu 3 cărți.

## Punctaj

Punctele Parcelelor, Moșiilor și Domeniilor din tot pachetul.

## Sfârșitul jocului

Când Domeniile sunt epuizate sau trei rezerve sunt goale (verificat la sfârșitul turei) sau după 100 de ture; câștigă cine are mai multe puncte (egal: mai puține ture).

## Note despre AI

Informație ascunsă: ordinea pachetului și mâna adversarului. AI-ul alege după o euristică statică și nu vede cărțile pe care le-ar trage.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Variantă originală de construire de pachet; cărți și nume proprii.
