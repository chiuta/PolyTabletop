# Itinerar (turism pe tablă, rezervi destinații) (`itinerar`)

Categorie: Zaruri. Doi jucători în registru (Jucătorul 1 contra Jucătorului 2 sau calculatorului); motorul știe și 3–4 locuri (`Itinerar::with_players`, folosit în teste). Pornire: `polytabletop play itinerar --seed 1`. Sursa implementării: `polytabletop-games/src/itinerar.rs`.

> Rezumat din `polytabletop list`: Joc original, 28 de câmpuri (7 regiuni × 3 destinații + aeroporturi, tombolă, vamă, bac, hotel de lux, taxă de turist): arunc, apoi mut (zarurile 2d6 sunt automate); pe o destinație liberă: rezerv (taxa de intrare) sau las; pe una ocupată plătești taxa de cazare. La final: construiesc N (cazare, 3 niveluri), rezerv N (la distanță, preț ×1,5, o dată pe tură), zbor (între aeroporturi, 60) sau gata. Colecție completă de regiune = taxe ×2; dublă = încă o aruncare, a treia dublă = vamă (pierzi tura); lux (nivel gratuit la Hotelul de lux). Faliment = eliminare; după 40 de runde câștigă averea mai mare. Informație publică.

## Scop

Rămâi ultimul jucător fără faliment sau, după 40 de runde, ai cea mai mare avere (bani + prețurile destinațiilor + nivelurile construite).

## Pregătire

Tablă circulară cu 28 de câmpuri: 7 regiuni (Litoral, Munții, Lacurile, Orașul vechi, Podgoriile, Deșertul, Arhipelagul) de câte 3 destinații, separate de 7 câmpuri speciale: Aeroport Nord (start), Tombolă, Vamă, Bac, Aeroport Sud (zonă de odihnă și aeroport), Hotel de lux, Taxa de turist. Fiecare jucător pornește cu 1200 de lei din Aeroport Nord; la fiecare trecere prin Aeroport Nord (sau sosire pe el) încasezi 100.

## Desfășurarea turei

1. `arunc` — se aruncă automat două zaruri 2d6 (se văd în starea jocului). `mut` — pionul avansează și se rezolvă câmpul. (Pasul dublu e intenționat: AI-ul nu poate „vedea” rezultatul zarurilor înainte de a alege.)
2. Destinație liberă (dacă îți permiți taxa de intrare = prețul): `rezerv` sau `las`. Destinație a altui jucător: plătești **taxa de cazare**.
3. La sfârșit, dacă ai ce face: `construiesc N` (+1 nivel de cazare pe destinația N, maximum 3), `rezerv N` (rezervare la distanță a unei destinații libere la preț ×1,5, o singură dată pe tură), `zbor` (dacă stai pe un aeroport: mergi la celălalt contra 60 și tura se încheie) sau `gata`. Dacă nu există decât `gata`, tura se închide singură.
4. Dublă = încă o aruncare; a treia dublă la rând = vamă (pionul merge la Vamă și pierzi tura următoare).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`, `mut`.
* `rezerv` / `las` (destinația pe care ai aterizat), `lux` / `las` (Hotelul de lux).
* `construiesc N`, `rezerv N` (N = numărul câmpului, vezi tabla), `zbor`, `gata`.

### Exemple

1. La început: mutări legale (1): `arunc`; apoi (1): `mut`.
2. Pe o destinație liberă: `rezerv`, `las`.
3. După aterizare, cu bani: `construiesc 1`, `rezerv 5`, `gata`.

## Reguli speciale și variante alese

* Taxa de cazare = (preț ÷ 5) × [1, 3, 5, 8] pentru nivelurile 0–3; dacă același jucător deține toate cele 3 destinații ale regiunii, taxa se dublează. Costul unui nivel depinde de regiune (40–100).
* Tombolă: carte de eveniment (premiu, cupon, reparații, asigurare, zbor gratuit la următorul aeroport, festival = încasezi 25 de la fiecare, cină = plătești 25 fiecăruia, ocol = înapoi 3 câmpuri).
* Bac: plătești 15 și treci 3 câmpuri înainte. Hotelul de lux: `lux` = 60 pentru un nivel gratuit pe una din unitățile tale. Taxa de turist: 10% din bani, între 30 și 120.
* Dacă nu poți plăti, ți se vând nivelurile (la jumătate din cost); dacă tot nu ajunge, ești eliminat și proprietățile revin băncii.
* Fără licitații, ipoteci sau schimburi.

## Punctaj

Averea = bani + prețurile destinațiilor + costul nivelurilor construite.

## Sfârșitul jocului

Rămâne un singur jucător neeliminat, sau se termină runda 40: câștigă averea mai mare (egal = remiză). Plasă de siguranță: 8000 de mutări.

## Note despre AI

AI-ul alege cu alpha-beta de adâncime 1. Zarurile se aruncă la `arunc` și efectul lor se rezolvă abia la `mut`, deci AI-ul nu vede aruncările viitoare. Nivelurile: Easy alege aproape aleator; Normal evaluează averea (bani, prețuri, niveluri, colecții de regiune); Hard/Expert evaluează și randamentul (taxe așteptate × șansa de aterizare × runde rămase) și riscul (taxe probabile în următoarele 12 câmpuri, pericol de faliment).

Informația este publică: jocul nu are nevoie de `set_viewer` și nu este în `HOTSEAT_SAFE`.

## Diferențe față de regulile tradiționale

Joc nou, nu o copie a „Moșii”: tablă de 28 de câmpuri cu regiuni de 3, rezervare la distanță cu preț majorat, cazare pe 3 niveluri fără a cere colecția completă, zbor între aeroporturi, vamă cu pierderea turei, tombolă și hotel de lux proprii; fără închisoare, gări sau case ca în „Moșii”.
