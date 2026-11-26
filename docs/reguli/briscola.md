# Briscola (Italia) (`briscola`)

Joc italian de levate pentru doi, cu pachet de 40 de cărți și un singur atu (briscola) stabilit de ultima carte a pachetului. Categorie: cărți, informație ascunsă.

## Scop

Aduni mai mult de 60 din cele 120 de puncte din pachet, câștigând levate cu cărți valoroase (Asso, Trei, Re).

## Pregătire

Pachet italian de 40 de cărți: culorile `d` (denari), `c` (coppe), `s` (spade), `b` (bastoni); rangurile, de la cel mai mic la cel mai mare, sunt 2 4 5 6 7 F(ante) C(avallo) R(e) 3 A(sso). Fiecare jucător primește 3 cărți; următoarea carte se întoarce și arată culoarea atu (briscola), rămânând jos ca ultima carte a pachetului (se trage ultima). Pachetul amestecat depinde de `--seed`. Cu `--you one` vezi doar mâna Jucătorului 1.

## Desfășurarea unei ture

Se joacă pe tururi de câte două cărți: cine conduce pune o carte, celălalt răspunde cu orice carte (nu e obligatoriu să răspunzi la culoare). Levata o ia cartea mai mare de aceeași culoare cu cea condusă sau, în lipsă, un atu. Câștigătorul ia ambele cărți, trage primul o carte din pachet, apoi o trage și celălalt, iar câștigătorul conduce levata următoare. Când pachetul se golește se joacă mai departe cu cele 3 cărți din mână, fără tras.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `7d` — joci cartea 7 de denari: rang (2 4 5 6 7 F C R 3 A) urmat de litera culorii (d c s b)
* `Ad` — Asso de denari (11 puncte)
* `3s` — Trei de spade (10 puncte)
* `Rb` — Regele de bastoni (4 puncte)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play briscola --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `2c` — joci Doiul de coppe (carte fără puncte), o carte de sacrificiu.
2. `Rb` — joci Regele de bastoni (4 puncte); tastezi rangul și apoi litera culorii.
3. `7s` — joci 7 de spade; în joc rămân cărțile rămase în mână, afișate după `Mâna ta`.

## Reguli speciale și variante alese

Cele 34 de cărți rămase după împărțire și atu formează pachetul de tras; atuul rămâne vizibil jos (se trage ultimul). Nu există obligația de a răspunde la culoare și nici de a tăia cu atu. Cartea Trei (3) este a doua ca putere, după Asso.

## Punctaj

Puncte din cărțile câștigate: A 11, 3 10, R 4, C 3, F 2, restul 0 (120 în total). Afișajul arată `Puncte: J1 x – J2 y`.

## Sfârșitul jocului

Când pachetul și mâinile s-au golit, câștigă cine are peste 60 de puncte; 60–60 este remiză.

## Note despre AI și informația ascunsă

Informație ascunsă: mâna adversarului și pachetul nu apar în afișaj; `render` arată doar mâna Jucătorului 1 (`set_viewer`). AI-ul folosește alpha-beta cu adâncime 1 și citește doar mâna proprie, atuul și punctele câștigate; cărțile trase după ultima levată nu intră în evaluare (nu privește viitorul). Tragerea din pachet se face automat după fiecare levată; nu e mutare introdusă de om.

## Diferențe față de regulile tradiționale

Joc pentru doi jucători, după regulile publice; antetul sursei nu declară alte simplificări (nu apare schimbul cărții de atu și nu există joc pe echipe).
