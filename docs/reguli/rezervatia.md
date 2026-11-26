# Rezervația (variantă originală) (`rezervatia`)

Rezervația: construirea unei rezervații naturale cu cărți-animale și sponsori, variantă originală pentru doi.

## Scop

Ai scorul mai mare (apel + 2 × conservare); la 50 jocul se încheie la sfârșitul rundei.

## Pregătire

Fiecare jucător are cinci cărți-acțiune într-un rând; cartea din locul `k` (1–5) are puterea `k`. După ce joci o acțiune, cartea ei trece pe locul 1, iar celelalte urcă un loc. Etichete: R prădători, C copitate, Z zburătoare, M mici. Habitate: Pădure, Munte, Luncă.

## Desfășurarea unei ture

Acțiuni (puterea `k` = locul cărții): Faună — `animal 2 e1` (cu `k ≥ 4` încă un animal sau `gata`); Amenajare — `construieste padure 3`, `chiosc`, `pavilion`; Sponsori — `sponsor 3` sau `finantare`; Cărți — `carti` sau `carti v2`; Conservare — `proiect 1` sau `donatie`. Pauză: fiecare acțiune adaugă `k` la ritmul tău; la 9 încasezi venitul (5 + apel/10 + chioșcuri + sponsori + animale cu venit).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `animal 2 e1` — joci animalul 2 în tarcul e1
* `construieste lunca 2` — construiești un tarc de Luncă de mărime 2
* `chiosc / pavilion` — amenajări
* `sponsor 1` — joci sponsorul 1
* `finantare` — primești k+2 bani
* `carti / carti v1` — tragi cărți / iei cartea v1 din vitrină
* `proiect 1 / donatie` — conservare
* `gata` — termini jucatul animalelor

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play rezervatia --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `construieste lunca 2` — construiești un tarc de Luncă de mărime 2.
2. `carti v1` — iei cartea v1 din vitrină și tragi restul.
3. `sponsor 1` — joci sponsorul 1.

## Reguli speciale și variante alese

Construire: mărime ≤ `k`, costă 2·mărime+1 (cu 1 mai puțin la `k ≥ 4`); chioșc `k ≥ 2` (+1 venit, +1 apel); pavilion `k ≥ 4` (+4 apel). Proiect de conservare: nivel ≤ `k`, cere animale cu eticheta proiectului; fiecare jucător poate susține fiecare proiect o dată; donație: plătești `8 − k`, +1 conservare.

## Punctaj

Scor = apel + 2 × conservare.

## Sfârșitul jocului

Cine ajunge la 50 încheie jocul la sfârșitul rundei (J2 are mereu tura finală); cel mult 30 de ture pe jucător. Egal: mai mulți bani, apoi remiză.

## Note despre AI și informația ascunsă

Informație ascunsă: pachetul și mâna adversarului; AI-ul nu le citește.

## Diferențe față de regulile tradiționale

Joc original; tema, cărțile și textele sunt ale noastre.
