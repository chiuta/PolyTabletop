# Memory (perechi) (`memory`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play memory --seed 1`. Sursa implementării: `polytabletop-games/src/memory.rs`.

> Rezumat din `polytabletop list`: 24 de cărți (12 perechi A–L), 6 coloane × 4 rânduri. O tură = două întoarceri (mutări separate): pereche = punct și mută din nou, altfel cărțile se întorc și mută adversarul. AI-ul are memorie limitată (memorează 70% din cărțile văzute, uită cu timpul). Mutări: a1–f4.

## Scop

Găsești mai multe perechi decât adversarul.

## Pregătire

24 de cărți (12 perechi, simbolurile A–L), în 6 coloane (a–f) × 4 rânduri (1–4), cu fața în jos. Pachetul este amestecat cu generatorul jocului.

## Desfășurarea turei

O tură = două întoarceri, fiecare o mutare separată (`b3`, apoi `e1`). Pereche găsită: cărțile ies din joc, ia un punct și mută din nou; altfel cărțile se întorc cu fața în jos și mută adversarul.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Poziția cărții: `a1`–`f4`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (24): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `a2`, `b2`, `c2`, `d2` …. Introduci `a1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (23): `b1`, `c1`, `d1`, `e1`, `f1`, `a2`, `b2`, `c2`, `d2`, `e2` …. Introduci `a3`; răspunsul calculatorului (în ordine): `e4`; `d3`.
3. Mutări legale (24): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `a2`, `b2`, `c2`, `d2` …. Introduci `f4`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Câte un punct pe pereche.

## Sfârșitul jocului

Când toate perechile au fost găsite; câștigă cine are mai multe (6–6 = remiză).

## Note despre AI

AI-ul are memorie limitată: fiecare carte descoperită intră în memoria lui doar cu probabilitatea 70%, iar la fiecare tură ratată fiecare intrare se poate șterge (8%). Alege doar din memoria lui și din prima carte deja descoperită; nu citește niciodată cărțile necunoscute.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
