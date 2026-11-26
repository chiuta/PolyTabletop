# Tarot francez (78 de cărți, 2 jucători) (`tarot`)

Tarot francez cu 78 de cărți, adaptat pentru doi jucători (contractul, chien-ul și zero-sum între doi).

## Scop

Ca declarant, adună suficiente puncte de carte pentru a atinge contractul (în funcție de numărul de oudlers); ca apărător, împiedici contractul.

## Pregătire

78 de cărți: 56 de culoare (`c d h s`: 1–10, V valet, C cavaler, D damă, R rege), 21 de atuuri `T1`…`T21` și Scuza `Ex`. Oudlers: T1, T21, Ex. Fiecare primește 36 de cărți, 6 merg în chien. Cine nu a împărțit vorbește primul.

## Desfășurarea unei ture

Licitație într-un singur tur: `pas`, `petite`, `garde`, `garde-sans`, `garde-contre` (multiplicatori 1/2/4/6). Al doilea trebuie să liciteze mai sus. Dacă ambii dau pas, se re-împarte. Cine ia primește chien-ul (`chien`, mutare forțată) și pune jos 6 cărți (`ecart Vh`, una câte una). Apoi se joacă levate cu culoare obligatorie, tăiere cu atu și urcare la atu; câștigătorul conduce.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pas / petite / garde / garde-sans / garde-contre` — licitația
* `chien` — declarantul ia chien-ul (forțat)
* `ecart Vh` — pui jos Valetul de cupă
* `T21 / 1c / Rs` — joci o carte
* `Ex` — joci Scuza
* `reimparte` — re-împărțire
* `urmatoarea` — mâna următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tarot --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `garde-sans` — licitezi „garde sans” (chien-ul rămâne pentru cine a luat, multiplicator 4).
2. `chien` — preiei chien-ul.
3. `1c` — joci 1 de treflă.

## Reguli speciale și variante alese

Scuza se joacă oricând, nu câștigă levata și rămâne în grămada celui care a jucat-o; se omite schimbul cu o carte mică și regula Scuzei la ultima levată. Chien-ul se pune jos fără regi și fără oudlers; atuuri doar dacă nu e altceva. La `petite` chien-ul se arată; la `garde` nu e vizibil apărării.

## Punctaj

Numărătoare în jumătăți de puncte, 91 în total: rege 4,5, damă 3,5, cavaler 2,5, valet 1,5, oudler 4,5, restul 0,5. Contract: 56 / 51 / 41 / 36 de puncte cu 0 / 1 / 2 / 3 oudlers. Scor mână = (25 + diferența + 10 pentru petit au bout) × multiplicator, zero-sum. Petit au bout: T1 în ultima levată câștigată de cine l-a jucat.

## Sfârșitul jocului

Meciul are 3 mâini; câștigă cine are mai multe puncte (egal = remiză).

## Note despre AI și informația ascunsă

Informație ascunsă: se vede doar mâna ta; chien-ul (la `garde`) și cărțile puse jos nu sunt vizibile apărării. AI-ul (alpha-beta adâncime 1) nu citește mâna adversarului. `chien` și re-împărțirea sunt mutări forțate.

## Diferențe față de regulile tradiționale

Doi jucători (36+36+6 cărți), toate cărțile se joacă; fără poignée, chelem și anunțuri; fără schimbul Scuzei.
