# Roata Destinului (parcurs de viață) (`roata`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play roata --seed 1`. Sursa implementării: `polytabletop-games/src/roata.rs`.

> Rezumat din `polytabletop list`: Variantă originală: roata (1–6) e afișată; mutarea avansez alege ramura la bifurcații (facultate/ucenicie, afacere/salariu) și, pe pătratul Casă, cumpar/nu. Plată la fiecare trecere, copii și casă valorează la final. Scor = bani + 30 casă + 8 per copil. Mutări: avansez, avansez facultate, avansez ucenicie, avansez afacere, avansez salariu, avansez cumpar.

## Scop

Ai cel mai mare scor la final: bani + 30 (casa) + 8 pentru fiecare copil.

## Pregătire

Fiecare jucător pornește de la Start și alege între Facultate (3 ani plătiți, apoi salariu mare) și Ucenicie (plecare rapidă, salariu mic). Drumul se unește, are o a doua bifurcație (Afacere riscantă sau Salariu stabil) și se termină la Pensie.

## Desfășurarea turei

Roata (1–6) este rotită automat și afișată; mutarea `avansez` o folosește și alege ramura la bifurcații. Pe pătratele Casă poți cumpăra casa dacă ai banii.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `avansez` (drum fără bifurcație).
* La bifurcații se adaugă eticheta ramurii: `avansez facultate` / `avansez ucenicie` (prima bifurcație) și `avansez afacere` / `avansez salariu` (a doua).
* Pe pătratul Casă se adaugă `cumpar` sau `nu`: `avansez cumpar`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `avansez facultate`, `avansez ucenicie`. Introduci `avansez facultate`; răspunsul calculatorului (în ordine): `avansez facultate`.
2. Mutări legale (1): `avansez`. Introduci `avansez`; răspunsul calculatorului (în ordine): `avansez`.
3. Mutări legale (1): `avansez`. Introduci `avansez`; răspunsul calculatorului (în ordine): `avansez afacere`.

## Reguli speciale și variante alese

* Efectele de salariu (Plată, Angajare) și taxele de școlarizare se aplică de câte ori treci peste ele; celelalte (cheltuieli, premii, copii, casă) doar când te oprești pe ele.
* Cine ajunge la Pensie iese; celălalt joacă până la capăt.

## Punctaj

Scor final = bani + 30 (casa) + 8 pentru fiecare copil.

## Sfârșitul jocului

Când ambii au ajuns la Pensie; câștigă scorul mai mare, egal = remiză.

## Note despre AI

Totul este determinist în afară de roată (afișată înainte), deci AI-ul nu vede nimic ascuns.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Variantă originală a jocurilor „drumul vieții”.
