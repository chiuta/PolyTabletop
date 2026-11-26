# Tarneeb (2 jucători) (`tarneeb`)

Tarneeb (Orientul Mijlociu), variantă pentru doi jucători, cu licitație pe levate.

## Scop

Ca licitator, iei cel puțin atâtea levate cât ai licitat; ajungi primul la 31 de puncte.

## Pregătire

52 de cărți, 13 fiecăruia, 26 nefolosite și ascunse. Ordinea: A (cea mai mare) … 2. Mâna se pornește cu `imparte`.

## Desfășurarea unei ture

Licitația: numărul de levate (7–13), urcând cu 1 sau 2 la fiecare tură; după un `pas` licitația se termină; dacă primul spune pas, dealerul e obligat să liciteze 7. Câștigătorul numește atuul (`atu h`) și conduce prima levată. Se răspunde la culoare, atu bate tot.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `imparte` — împarte
* `pas` — renunți
* `7 … 13` — licitezi numărul de levate
* `atu h` — numești atuul (h/d/s/c)
* `Ah` — joci o carte

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tarneeb --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `7` — licitezi 7 levate.
2. `pas` — după licitația adversarului (8) dai `pas`; licitația se încheie.
3. `6c` — joci 6 de treflă (atu numit de adversar: treflă).

## Reguli speciale și variante alese

Prima persoană care spune pas încheie licitația.

## Punctaj

Dacă licitatorul a luat cel puțin cât a licitat, primește numărul de levate luate (16 dacă a licitat 13 și l-a făcut); altfel pierde cât a licitat. Adversarul primește levatele luate (+3 bonus dacă licitatorul a căzut).

## Sfârșitul jocului

Prima persoană la 31 de puncte câștigă.

## Note despre AI și informația ascunsă

AI-ul (alpha-beta adâncime 1) citește doar propria mână.

## Diferențe față de regulile tradiționale

Doi jucători în loc de patru, 26 de cărți nefolosite.
