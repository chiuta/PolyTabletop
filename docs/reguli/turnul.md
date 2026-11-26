# Turnul Șovăitor (turn de blocuri) (`turnul`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play turnul --seed 1`. Sursa implementării: `polytabletop-games/src/turnul.rs`.

> Rezumat din `polytabletop list`: Simulare discretă, deterministă: 12 straturi de 3 blocuri; iei un bloc de sub stratul complet de sus și îl pui deasupra. Turnul stă dacă centrul de masă de deasupra fiecărui strat cade în învelișul blocurilor lui rămase; cine îl prăbușește pierde. Mutări: ia 4b pune a.

## Scop

Nu prăbuși turnul: cine îl face să cadă pierde.

## Pregătire

12 straturi de câte 3 blocuri (sloturile `a`, `b`, `c`; stratul 1 e jos). Blocul are lățimea 1 și lungimea 3, iar straturile se rotesc alternativ cu 90°.

## Desfășurarea turei

Scoți un bloc dintr-un strat aflat sub cel mai înalt strat complet și îl așezi sus: într-un slot liber al stratului de sus dacă nu e complet, altfel începi un strat nou.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `ia 4b pune a`: stratul 4, slotul b, pe slotul a al stratului de sus.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (99): `ia 1a pune a`, `ia 1a pune b`, `ia 1a pune c`, `ia 1b pune a`, `ia 1b pune b`, `ia 1b pune c`, `ia 1c pune a`, `ia 1c pune b`, `ia 1c pune c`, `ia 2a pune a` …. Introduci `ia 1a pune a`; răspunsul calculatorului (în ordine): `ia 1c pune c`.
2. Mutări legale (31): `ia 1b pune b`, `ia 2a pune b`, `ia 2b pune b`, `ia 2c pune b`, `ia 3a pune b`, `ia 3b pune b`, `ia 3c pune b`, `ia 4a pune b`, `ia 4b pune b`, `ia 4c pune b` …. Introduci `ia 6c pune b`; răspunsul calculatorului (în ordine): `ia 6a pune a`.
3. Mutări legale (64): `ia 1b pune b`, `ia 1b pune c`, `ia 2a pune b`, `ia 2a pune c`, `ia 2b pune b`, `ia 2b pune c`, `ia 2c pune b`, `ia 2c pune c`, `ia 3a pune b`, `ia 3a pune c` …. Introduci `ia 12c pune c`; răspunsul calculatorului (în ordine): `ia 12a pune b`.

## Reguli speciale și variante alese

* Turnul stă dacă, pentru fiecare strat, centrul de masă al tuturor blocurilor de deasupra cade în învelișul convex al blocurilor rămase ale stratului (marginea inclusă). Un strat gol nu sprijină nimic.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Dacă turnul nu mai stă după scoatere sau după așezare, cel care a mutat pierde.

## Note despre AI

Jocul este determinist, fără noroc. AI-ul caută 2 plies și minimizează mutările sigure ale adversarului.

Stil AI în cod: alpha-beta cu adâncime fixă 2.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Simulare discretă a stabilității (centru de masă și înveliș convex), nu fizica reală a jocului cu blocuri de lemn.
