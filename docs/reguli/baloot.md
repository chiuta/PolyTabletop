# Baloot (2 jucători) (`baloot`)

Baloot (Arabia Saudită), variantă pentru doi jucători, cu 32 de cărți.

## Scop

Ca cumpărător, aduni mai multe puncte decât adversarul; ajungi primul la 50.

## Pregătire

32 de cărți (7–A); fiecare primește 8, o carte se întoarce (culoarea propusă în runda 1); 15 cărți rămân nefolosite. `imparte` pornește o mână nouă.

## Desfășurarea unei ture

Licitația: runda 1 — `hokm` (atu = culoarea întoarsă), `sun` (fără atu) sau `pas`; runda 2 — `hokm h|d|s|c` (altă culoare), `sun` sau `pas`; dacă ambii dau pas în runda 2, se dă din nou. Apoi 8 levate: la hokm răspunzi la culoare; dacă n-ai culoarea și ai atu, ești obligat să tai; la atu cerut, dacă poți, urci peste atu-ul adversarului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `imparte` — începe mâna
* `pas` — renunți
* `hokm` — atu = culoarea întoarsă
* `hokm h` — în runda 2: atu cupă; `d`, `s`, `c` analog
* `sun` — fără atu
* `Jh` — joci o carte

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play baloot --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `sun` — cumperi „sun” (fără atu).
2. `Ah` — joci Asul de cupă.
3. `10h` — joci 10 de cupă.

## Reguli speciale și variante alese

Valori: Sun — A 11, 10 10, K 4, Q 3, J 2, restul 0. Hokm — în atu J 20, 9 14, A 11, 10 10, K 4, Q 3; ordinea în atu: J, 9, A, 10, K, Q, 8, 7. Ultima levată +10.

## Punctaj

Hokm = puncte/10 (rotunjit), Sun = puncte/5 (dublu). Cumpărătorul care pierde dă tot adversarului. „Baloot” (K+Q de atu jucate de același jucător) = +2.

## Sfârșitul jocului

Primul la 50 câștigă.

## Note despre AI și informația ascunsă

AI-ul (alpha-beta adâncime 1) citește doar propria mână.

## Diferențe față de regulile tradiționale

Fără proiecte (sira, 50, 100, 400) și fără dublare; doi jucători, 15 cărți nefolosite.
