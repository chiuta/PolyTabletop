# Euchre (2 jucători) (`euchre`)

Euchre pentru doi jucători, cu 24 de cărți (9–A) și bauer-uri (valeți speciali).

## Scop

Ajungi primul la 10 puncte; ca maker trebuie să iei cel puțin 3 din 5 levate.

## Pregătire

24 de cărți (9 10 J Q K A). Fiecare primește 5 cărți, o carte se întoarce („upcard”), restul nu se folosesc. Nu se joacă cu dummy.

## Desfășurarea unei ture

Runda 1: non-dealerul, apoi dealerul, spun `pas` sau `up` (atu = culoarea întoarsă; dealerul ia cartea și dă una înapoi cu `dis`). Runda 2: culoarea întoarsă e interzisă; non-dealerul poate da `pas`, dealerul e obligat să numească o culoare („stick the dealer”). Apoi se joacă 5 levate cu culoare obligatorie; atu bate tot.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pas` — nu iei
* `up` — atu = culoarea întoarsă
* `nume h` — numești culoarea h (h/d/s/c)
* `dis 9h` — dealerul dă o carte
* `Jh` — joci o carte
* `imparte` — mâna următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play euchre --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `up` — ordonezi atu: pică, cartea întoarsă (în seed 1 de pică).
2. `Js` — joci Valetul de pică (bauer, atu).
3. `Qs` — joci Dama de pică.

## Reguli speciale și variante alese

Atu: Valetul de atu (bower dreapta) e cel mai mare, apoi Valetul de aceeași culoare-soră (bower stânga, care ESTE atu), apoi A, K, Q, 10, 9. Dealerul ia upcard-ul și dă jos o carte.

## Punctaj

Maker: 3–4 levate = 1 punct, 5 = 2 puncte; altfel („euchre”) adversarul ia 2 puncte.

## Sfârșitul jocului

Primul la 10 puncte câștigă.

## Note despre AI și informația ascunsă

Mâna adversarului nu apare. AI-ul (alpha-beta adâncime 1) citește doar propria mână, cartea întoarsă și scorul public.

## Diferențe față de regulile tradiționale

Variantă simplă pentru doi: fără dummy; cărțile rămase nefolosite stau deoparte.
