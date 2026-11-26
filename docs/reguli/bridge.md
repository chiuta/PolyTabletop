# Bridge duplicat (2 jucători) (`bridge`)

Bridge duplicat pentru doi jucători cu mână moartă; fiecare jucător conduce un cuplu care își vede toate cele 26 de cărți.

## Scop

Obții scorul combinat mai bun pe cele două table jucate cu aceeași donă.

## Pregătire

Aceeași donă se joacă de două ori (duplicat): la tabla 1 J1 conduce Nord–Sud (și J2 Est–Vest), la tabla 2 rolurile se schimbă. Fiecare vede ambele mâini ale cuplului lui.

## Desfășurarea unei ture

Licitație: cele două părți licitează pe rând (Nord–Sud începe): `1c`…`7n`, `pass`, `x` (contra), `xx` (recontra). Licitația se termină după un `pass` care urmează unei licitații/contre; dacă primele două chemări sunt pass, tabla e „aruncată” (0). Declarant: Sud (la N–S) sau Vest (la E–V); mâna moartă (Nord / Est) se arată după prima carte, iar declarantul o joacă. Se conduce la stânga declarantului; se răspunde la culoare.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `1h … 7n` — licitație (c, d, h, s, n = fără atu)
* `pass / x / xx` — pass, contra, recontra
* `Ah` — joci o carte din mâna al cărei rând e
* `10s` — 10 de pică

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play bridge --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `pass` — dai pass.
2. `x` — contrezi licitația adversarilor (1n).
3. `pass` — dai pass după recontra.

## Reguli speciale și variante alese

Nimeni nu e vulnerabil. Cine joacă a doua oară același set de cărți își poate aminti de la prima tablă (natura duplicatului cu o singură donă).

## Punctaj

Scor duplicat (nimeni vulnerabil): puncte pe levată (minor 20, major 30, fără atu 40+30…, ×2 contrat, ×4 recontrat), bonus manșă (300 dacă punctele de contract ≥ 100, altfel 50), șlem mic 500, șlem mare 1000, levate peste contract, bonus de „insultă” (50/100), căderi. Scor combinat = scor N–S tabla 1 − scor N–S tabla 2 (văzut de J1).

## Sfârșitul jocului

După cele două table câștigă scorul combinat mai bun.

## Note despre AI și informația ascunsă

`render` arată doar mâinile deținute de J1 și mâna moartă când e vizibilă; AI-ul citește doar mâinile lui și mâna moartă.

## Diferențe față de regulile tradiționale

Nu există parteneri separați (un jucător conduce două mâini); doar duplicat cu o singură donă și fără vulnerabilitate.
