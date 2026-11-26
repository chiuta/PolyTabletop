# Pachisi (cursă cu cauri) (`pachisi`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pachisi --seed 1`. Sursa implementării: `polytabletop-games/src/pachisi.rs`.

> Rezumat din `polytabletop list`: 6 cauri: 1 deschis = 10, 2 = 2, 3 = 3, 4 = 4, 5 = 25, 6 = 6, niciunul = 12; 6, 10, 12 și 25 sunt grațioase (încă o aruncare și poți scoate o piesă din bază). Pistă de 48 de pătrate, castele sigure din 6 în 6, două piese proprii = bloc, aterizarea pe o piesă singură o captură (+ aruncare). Ținta exactă de pe pistă; din culoarul de acasă orice aruncare ≥ distanța. Mutări: intră, sau progresul piesei (ex. 12).

## Scop

Aduci primul toate cele 4 piese în centru.

## Pregătire

Pistă comună de 48 de pătrate; Unu pornește de la 0, Doi de la 24. Progresul unei piese: 0 = în bază, 1–47 = pe pistă (1 = pătratul de start), 48–53 = culoarul de acasă, 54 = centrul (țintă). 4 piese fiecare.

## Desfășurarea turei

Se aruncă 6 cauri în stare (numărul de cauri cu deschiderea în sus dă mutarea: 1 → 10, 2 → 2, 3 → 3, 4 → 4, 5 → 5, 6 → 6, niciunul → 25). Valorile 6, 10 și 25 sunt „grațioase”: dau dreptul la încă o aruncare (chiar dacă nu ai mutare) și permit scoaterea unei piese din bază (`intră`; piesa nu avansează în aceeași mutare). Dacă nu ai nicio mutare, tura trece automat.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `intră` (scoate o piesă din bază pe pătratul de start).
* Progresul piesei pe care o muți: `12`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `intră`. Introduci `intră`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `1`. Introduci `1`; răspunsul calculatorului (în ordine): `5`.
3. Mutări legale (1): `4`. Introduci `4`; răspunsul calculatorului (în ordine): `8`.

## Reguli speciale și variante alese

* Pătratele 0, 6, 12, …, 42 sunt sigure („castele”): piesele nu se capturează și pot fi piese ale ambilor.
* Pe un pătrat nesigur, aterizarea pe o piesă singură a adversarului o trimite în bază și îți dă încă o aruncare.
* Două piese ale aceluiași jucător pe un pătrat de pistă formează un bloc: adversarul nu poate ateriza acolo (poate trece peste).
* De pe pistă ținta se atinge exact; din culoarul de acasă orice aruncare ≥ distanța duce în centru.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Câștigă cine aduce primul toate cele 4 piese în centru.

## Note despre AI

Cauri sunt informație publică. AI-ul (adâncime 1) evaluează progresul total și expunerea; nu vede aruncarea următoare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`; are elemente aleatoare, dar acestea se consideră publice pentru categoria Zaruri, deci `--you both` afișează avertisment, dar `hint` rămâne disponibil.

## Diferențe față de regulile tradiționale

* Doar cauri (fără varianta cu zaruri); pistă scurtă de 48 de pătrate în loc de crucea de 96; blocurile se pot sări; un singur rând de „castele”.
