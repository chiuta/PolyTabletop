# Ordine de bătaie (infanterie, zaruri de ordine) (`ordine`)

Ordine de bătaie: infanterie cu zaruri de ordine și suprimare, pe hartă 12×8.

## Scop

Ai mai multe puncte de obiectiv după 6 ture (apoi mai mulți oameni rămași) sau elimini armata adversă.

## Pregătire

Fiecare parte: Ofițer, 2 Grupe, Mitralieră, Mortier. Hartă 12×8 cu ziduri (`##`), păduri (`::`, acoperire) și 2 obiective (`()`).

## Desfășurarea unei ture

Câte un „zar de ordine” pe unitate vie într-un săculeț. Mutarea forțată `trage` extrage un zar la întâmplare: partea extrasă activează o unitate care nu a acționat și îi dă un ORDIN. Tura se termină când săculețul e gol.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `trage` — extrage un zar de ordine (forțat)
* `B fire xW` — stă și trage
* `B adv d4 xW` — avans: mută ≤ mv și poate trage
* `B run d4` — mută ≤ 2×mv, nu trage (cu xV lipit = asalt)
* `B amb` — ambuscadă
* `B rally` — scoate 2 marcaje de suprimare
* `B down` — la sol
* `arunca` — aruncă zarurile

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play ordine --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `trage` — extragi un zar de ordine (mutare forțată).
2. `B adv d4 xZ` — Grupa B avansează la d4 și trage în Mortierul Z.
3. `arunca` — `arunca` rezolvă zarurile (B a provocat 2 pierderi lui Z).

## Reguli speciale și variante alese

Suprimare: o unitate cu marcaje trebuie să treacă un test de ordin (2d6 ≤ moral − marcaje, +1 dacă un Ofițer prieten e la ≤ 6) ca să execute ordinul; altfel merge la sol. Foc: lovire la d6 ≥ 4 (modificatori), salvare d6 ≥ 5; Mitraliera și Mortierul nu trag după avans; Mortierul (distanță 3–12) trage indirect. Asalt: ambele părți lovesc simultan; cine pierde mai mulți oameni ia 2 marcaje.

## Punctaj

La sfârșitul turei, cine are mai mulți oameni din unități fără marcaje la ≤ 1 de obiectiv ia 1 punct.

## Sfârșitul jocului

După 6 ture: puncte, apoi oameni rămași; dispariția armatei adverse câștigă imediat.

## Note despre AI și informația ascunsă

Informație completă; zarurile și extragerea zarului de ordine se fac la mutările forțate `trage` / `arunca`, cu zaruri a căror sămânță se schimbă la `clone`, deci AI-ul nu poate anticipa. AI = alpha-beta adâncime 1 cu valori așteptate.

## Diferențe față de regulile tradiționale

Joc original („scrise de noi”); fără abateri declarate.
