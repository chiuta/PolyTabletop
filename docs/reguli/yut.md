# Yut (Yunnori) (`yut`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play yut --seed 1`. Sursa implementării: `polytabletop-games/src/yut.rs`.

> Rezumat din `polytabletop list`: 4 bețe-zar (arunc): 1 plat = do, 2 = gae, 3 = geol, 4 = yut (încă o aruncare), 0 = mo (5, încă o aruncare). 4 piese pe jucător; rezultatele se folosesc în orice ordine; grupurile de pe același nod merg împreună; capturarea trimite la start și dă o aruncare. Scurtături din colțurile 5 și 10 (centrul 22). Câștigă cine scoate toate piesele. Mutări: arunc, do:start, gae:7, mo:22.

## Scop

Scoți primul toate cele 4 piese.

## Pregătire

Patru bețe-zar, fiecare cade cu fața plată sau rotundă (50%). Plate: 1 = Do (1), 2 = Gae (2), 3 = Geol (3), 4 = Yut (4, aruncă din nou), 0 = Mo (5, aruncă din nou). Fără Back-do. Fiecare jucător are 4 piese („mal”). Tablă: cerc de 19 noduri, nodul 0 start/final, colțuri 5, 10, 15, cu scurtături 20–24 din colțul 5 și 25–28 din colțul 10 (centrul 22).

## Desfășurarea turei

Arunci cu `arunc` (zarul e în starea jocului). Rezultatele aruncate se păstrează și se pot folosi în orice ordine, fiecare mută o piesă (sau un grup).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`.
* Rezultat + piesă: `do:start` (piesă din start), `gae:7` (piesa de pe nodul 7), `mo:22`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `do:start`. Introduci `do:start`; răspunsul calculatorului (în ordine): `arunc`; `do:start`; `arunc`; `gae:start`.
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Scurtătura se ia doar când piesa începe mutarea dintr-un colț (5 sau 10); din centru se merge direct spre final.
* Piesele proprii de pe același nod formează un grup care se mută împreună.
* Aterizarea pe piese adverse le trimite la start și dă o aruncare în plus.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Câștigă cine scoate primul toate cele 4 piese (piesa care trece de ultimul nod a ieșit).

## Note despre AI

Zarurile sunt aruncate în stare; mutarea forțată `arunc` e separată, deci AI-ul nu vede aruncările viitoare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Fără Back-do (pas înapoi), omis intenționat.
