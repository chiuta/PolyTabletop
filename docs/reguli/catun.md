# Cătun (variantă originală) (`catun`)

Cătun: plasare de muncitori și un sat de cărți, variantă originală pentru doi.

## Scop

Ai scorul mai mare (punctele cărților și ale jetoanelor + prosperitate).

## Pregătire

Resurse: Lemn (L), Rășină (R), Piatră (P), Mure (M). Fiecare jucător își face un cătun din cel mult 15 cărți (Locuitori și Clădiri). Fiecare jucător își parcurge singur anotimpurile Iarnă → Primăvară → Vară → Toamnă, cu 2, 3, 4 și 6 muncitori. Limita mâinii: 8 cărți.

## Desfășurarea unei ture

O tură = o singură acțiune: `lucru a` (pui un muncitor pe locația a–h; câte un muncitor al tău pe locație; locațiile „exclusive” nu mai primesc alt muncitor până la pregătire; `lucru g L` — Hanul cere și alegerea unei resurse); `joc 3` / `piata 5` — joci o carte din mână / din piață (8 cărți vizibile), plătind costul; `sezon` — te pregătești pentru anotimpul următor: muncitorii se întorc, primești muncitorii noi, producătoarele lucrează (Iarnă→Primăvară și Vară→Toamnă), în Primăvară→Vară tragi 2 cărți. `sezon` în Toamnă te scoate din joc.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `lucru a` — muncitor pe locația a (a–h)
* `lucru g L` — la Han, cu resursa L
* `joc 3` — joci cartea 3 din mână
* `piata 1` — iei cartea 1 din piață
* `sezon` — treci la anotimpul următor

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play catun --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `piata 1` — cumperi Pescarul din piață.
2. `lucru h` — muncitor pe locația h (Biserica: 1 punct + 1 M).
3. `lucru g P` — muncitor pe Han (g), alegând resursa Piatră.

## Reguli speciale și variante alese

Locații: a Tăietura (excl., 3 L), b Cariera (excl., 2 P), c Rășinăria (excl., 2 R), d Poiana (2 M), e Pădurea mică (1 L + 1 R), f Târgul (2 cărți), g Hanul (excl., 1 carte + 1 resursă la alegere), h Biserica (excl., 1 punct + 1 M). Tipuri de cărți: Producătoare, Daruri, Reduceri, Prosperitate și simple.

## Punctaj

Punctele cărților și ale jetoanelor + prosperitate. Egalitate: mai multe resurse.

## Sfârșitul jocului

Jocul se termină când ambii au terminat (au ieșit din Toamnă).

## Note despre AI și informația ascunsă

Informație ascunsă: pachetul și mâna adversarului. AI-ul nu citește piața (se reumple la întâmplare) și numără doar cărțile din mână.

## Diferențe față de regulile tradiționale

Joc original (mecanică generică).
