# Ținuturi (variantă originală) (`tinuturi`)

Ținuturi: domino de teritorii cu coroane, variantă originală pentru doi; regat 9×9 care încape în 7×7.

## Scop

Construiești un regat cu scor mai mare.

## Pregătire

Castelul în `e5`. Șase teritorii: Câmp (C), Pădure (P), Apă (A), Pășune (S), Mlaștină (M), Stâncă (R); unele pătrate poartă coroane. Setul are 48 de piese-domino (generate de noi); într-un joc se folosesc 24. Rânduri de câte 4 piese, în ordinea numerelor.

## Desfășurarea unei ture

Runda 0: se aleg piesele primului rând în ordinea J1, J2, J2, J1 (`alege 3`). Apoi se dezvăluie rândul următor (mutare forțată `revelare`). Piesele rândului curent se joacă în ordinea numerelor: proprietarul o așază (`pune e6-e7`) sau, doar dacă nu are nicio așezare legală, o aruncă (`descarta`), apoi își alege o piesă din rândul următor.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `alege 4` — alegi piesa 4 din rând
* `revelare` — mutare forțată: se dezvăluie rândul
* `pune e6-e7` — așezi prima jumătate pe e6, a doua pe e7
* `descarta` — arunci piesa, doar dacă nu ai nicio așezare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tinuturi --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `alege 4` — alegi piesa 4 (J1 are alegerile 1 și 4 în prima rundă).
2. `alege 1` — alegi piesa 1.
3. `revelare` — mutare forțată: se dezvăluie rândul următor.

## Reguli speciale și variante alese

Așezare legală: ambele pătrate libere, regatul rămâne în 7×7 și cel puțin un pătrat se lipește pe lat de castel sau de un pătrat de același teritoriu.

## Punctaj

Fiecare regiune conexă de același teritoriu = (număr de pătrate) × (coroane). +5 Armonie (nicio piesă aruncată), +10 Regat central. Egalitate: regiunea cea mai mare, apoi cele mai multe coroane.

## Sfârșitul jocului

După ultimul rând de piese.

## Note despre AI și informația ascunsă

Informația e deschisă; singura întâmplare e ordinea pachetului (`revelare`). AI-ul nu citește pachetul.

## Diferențe față de regulile tradiționale

Joc original; piesele sunt generate de noi.
