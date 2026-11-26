# Ordine și reacții (skirmish cu reacții) (`reactii`)

Ordine și reacții: skirmish cu 4 soldați pe parte (Lider, Pușcaș, Lunetist, Asalt) pe hartă 10×10, cu zaruri d20.

## Scop

Ai mai multe puncte de obiectiv după 8 runde sau elimini adversarul.

## Pregătire

Hartă 10×10 cu ziduri, acoperiri și 2 obiective. Partea activă are atâtea ORDINE câți soldați vii are (+1 dacă Liderul trăiește). Zarurile se aruncă la `arunca`.

## Desfășurarea unei ture

Un ordin merge unui soldat (oricare, de mai multe ori): mută ≤ mv și/sau trage într-o țintă vizibilă; `fin` încheie tura. În tura adversarului, după ce soldatul activ a ajuns pe pătratul final, fiecare soldat inamic cu linie de vizibilitate liberă decide pe rând: `V pas`, `V tira` sau, dacă e ținta declarată, `V esiva`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `A>d4 xV` — mută și trage
* `A xV` — trage
* `A>d4` — mută
* `fin` — încheie tura
* `V pas` — nu reacționezi
* `V tira` — tragere de reacție
* `V esiva` — esivare (doar ținta declarată)
* `arunca` — aruncă zarurile

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play reactii --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `D>i7 xV` — Asaltul (D) mută la i7 și trage în Liderul V.
2. `arunca` — `arunca` rezolvă zarurile.
3. `C>i2 xW` — Lunetistul (C) mută la i2 și trage în W.

## Reguli speciale și variante alese

Tragerea reușește la d20 ≤ îndemânarea + modificatorul benzii de distanță (≤3 scurtă, ≤6 medie, mai mult lungă); un rezultat egal cu ținta e CRITIC. Față în față câștigă cel mai mare rezultat reușit (critic bate simplu, egal = nimic; la esivare egalul îl avantajează pe apărător). O lovitură necritică e salvată dacă d20 ≤ armura țintei (+4 în acoperire). Simplificare: vizibilitatea se verifică doar pe pătratul final.

## Punctaj

La sfârșitul rundei obiectivul e al celui cu mai mulți soldați la ≤ 1 pătrat: 1 punct.

## Sfârșitul jocului

8 runde; apoi puncte, apoi răni; eliminarea adversarului câștigă imediat.

## Note despre AI și informația ascunsă

Informație completă; AI = alpha-beta adâncime 1 cu valori așteptate (calcul exact al rezultatelor față în față, fără zaruri).

## Diferențe față de regulile tradiționale

Joc original; simplificare declarată: vizibilitatea reacțiilor se verifică doar pe pătratul final.
