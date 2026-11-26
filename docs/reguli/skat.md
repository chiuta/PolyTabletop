# Skat (2 jucători) (`skat`)

Skat adaptat pentru doi jucători, cu 32 de cărți, licitație pe scara tradițională și meci de 6 mâini.

## Scop

Câștigi puncte jucând ca declarant (cu 61 din 120 de puncte de carte) sau învingând declarantul; câștigă cine are mai multe puncte după 6 mâini.

## Pregătire

32 de cărți (7 8 9 10 J Q K A; `c` ♣, `s` ♠, `h` ♥, `d` ♦). Fiecare primește 10 cărți, 2 merg în skat; celelalte 10 rămân ascunse și nu se joacă. Cine nu a împărțit conduce prima levată.

## Desfășurarea unei ture

Licitația: cine a împărțit spune valori din scara 18 20 22 23 24 27 30 33 35 36 40 44 45 46 48 50 54 55 59 60 (`licit N`); celălalt `tin` (ține) sau `pas`. Cine rămâne în licitație este declarantul. Declarantul ia skat-ul (`skat`) și pune jos 2 cărți (`pune 10h Jc`) sau joacă din mână (`hand`, valorează mai mult). Apoi alege jocul: `trefla`, `pica`, `cupa`, `caro` (atu = cei 4 juniori + culoarea), `grand` (atu doar juniorii) sau `null`. Se joacă 10 levate cu culoare obligatorie (juniorii aparțin atuului).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `licit 18` — licitezi 18
* `tin` — accepți valoarea curentă
* `pas` — renunți
* `skat` — declarantul ia cele două cărți din skat
* `pune 10h Jc` — pui jos două cărți
* `hand` — joci din mână, fără skat
* `trefla / pica / cupa / caro / grand / null` — alegi jocul
* `Jh` — joci o carte (rang + culoare)
* `reimparte` — se re-împarte (mutare forțată)
* `urmatoarea` — mâna următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play skat --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `licit 18` — în seed 1 Jucătorul 2 a dat pas; tu licitezi 18 și devii declarant.
2. `hand` — joci „din mână” (fără skat), valoare mai mare.
3. `caro` — alegi jocul caro: atu sunt cei 4 juniori și culoarea caro.

## Reguli speciale și variante alese

Baze: trefla 12, pica 11, cupa 10, caro 9, grand 24, null 23 (din mână 35). Matadori = „cu/fără” juniorii și atuul consecutiv. Schneider = 90+ sau cel mult 30 de puncte; schwarz = toate/nicio levată. Dacă licitantul dă pas din start, celălalt poate declara `licit 18` sau da pas (re-împărțire, numără ca mână).

## Punctaj

Valoarea = bază × (matadori + 1 + mână + schneider + schwarz). Câștig: +valoarea; pierdere: −2×valoarea. Dacă valoarea e mai mică decât licitația, jocul e pierdut („supralicitat”). Puncte de carte: A 11, 10 10, K 4, Q 3, J 2; declarantul are nevoie de 61 din 120.

## Sfârșitul jocului

Meciul are 6 mâini; câștigă cine are mai multe puncte (egal = remiză).

## Note despre AI și informația ascunsă

Informație ascunsă: se vede doar mâna Jucătorului 1; skat-ul rămâne ascuns până îl ia declarantul. AI-ul (alpha-beta adâncime 1) citește doar mâna proprie, licitația și punctele din levate; cărțile din skat luate nu intră în evaluarea mutării `skat`. Re-împărțirea este mutare forțată.

## Diferențe față de regulile tradiționale

Doi jucători în loc de trei (10 cărți nejucate), fără ouvert, Ramsch, Kontra/Re și schneider/schwarz anunțate; meci de 6 mâini, nu 36 de jocuri.
