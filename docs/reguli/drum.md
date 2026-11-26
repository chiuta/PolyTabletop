# Drum de pietre (5×5, stive) (`drum`)

Drum de pietre: construcție de drumuri cu stive pe o tablă 5×5 (joc de tip „drum de pietre”), implementare originală.

## Scop

Formează un drum (lanț ortogonal de pietre proprii plate sau căpetenii, la vârful stivelor) între două margini opuse.

## Pregătire

Fiecare jucător are 21 de pietre plate și o piatră-căpetenie (capstone). Prima mutare a fiecăruia: pui o piatră plată a ADVERSARULUI pe un pătrat gol.

## Desfășurarea unei ture

Pui o piatră din rezervă pe un pătrat gol — plată (`c3`), în picioare / zid (`sc3`) sau căpetenia (`cc3`) — sau muți o stivă care îți are la vârf piatra: iei de sus 1–5 pietre și le lași pe o linie dreaptă (`+` sus, `-` jos, `<` stânga, `>` dreapta), cel puțin una pe fiecare pătrat parcurs. Notație: `[nr]pătrat dir [lăsate]`, ex. `c3>` (o piatră) sau `3c3>21` (3 pietre: 2 pe d3, 1 pe e3).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `c3` — pui o piatră plată pe c3
* `sc3` — pui un zid pe c3
* `cc3` — pui căpetenia pe c3
* `c3>` — muți o piatră din c3 spre dreapta
* `3c3>21` — iei 3 pietre din c3: 2 pe d3, 1 pe e3

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play drum --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `e1` — prima mutare: pui o piatră plată a adversarului pe e1.
2. `b3` — pui o piatră plată pe b3.
3. `b4` — pui o piatră plată pe b4.

## Reguli speciale și variante alese

Zidurile și căpetenia blochează mutarea stivelor; doar o căpetenie lăsată SINGURĂ ca ultimă piatră peste un zid îl turtește (devine plat). Căpetenia nu poate fi acoperită.

## Punctaj

Nu există scor; numărul de pietre plate la vârf decide la final.

## Sfârșitul jocului

Câștigă cine închide un drum (dacă o mutare face drum ambilor, câștigă cel care a mutat). Dacă tabla se umple, cineva își epuizează rezerva sau se ating 150 de semi-mutări, câștigă cine are mai multe pietre plate la vârf (egal = remiză).

## Note despre AI și informația ascunsă

Informație completă; alpha-beta cu adâncire iterativă cu buget de timp.

## Diferențe față de regulile tradiționale

Joc „de tip drum de pietre” pe 5×5; denumirile sunt generice.
