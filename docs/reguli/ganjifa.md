# Ganjifa (levate) (`ganjifa`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ganjifa --seed 1`. Sursa implementării: `polytabletop-games/src/ganjifa.rs`.

> Rezumat din `polytabletop list`: 48 de cărți (4 culori: Taj T, Shamsher S, Surkh R, Chang C; Mir M, Vazir V, 1–10). La T și S 10 e cel mai mare număr, la R și C ordinea e inversată (1 cel mai mare). 12 cărți fiecare; cartea de sus din stoc arată atu. 12 levate, culoare obligatorie, atu bate. Mai multe levate câștigă. Mutări: TM, S10, R1, CV.

## Scop

Câștigi mai multe levate din cele 12 (7 sunt suficiente).

## Pregătire

48 de cărți: 4 culori (Taj `T`, Shamsher `S`, Surkh `R`, Chang `C`), fiecare cu Mir (`M`), Vazir (`V`) și numerele 1–10. La `T` și `S` ordinea numerelor crește (10 e cel mai mare), la `R` și `C` este inversată (1 e cel mai mare). Mir și Vazir bat numerele. 12 cărți fiecare; cartea de deasupra stocului se întoarce și arată atuul (hukm).

## Desfășurarea turei

12 levate; conduce J1 în prima, câștigătorul unei levate conduce următoarea. Trebuie să dai din culoarea cerută dacă ai; altfel poți juca orice. Levata o ia cel mai mare atu, altfel cea mai mare carte din culoarea cerută.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Culoare + valoare: `TM`, `S10`, `R1`, `CV`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (12): `T2`, `T5`, `TV`, `TM`, `S3`, `S4`, `S5`, `S10`, `SV`, `SM` …. Introduci `T2`; răspunsul calculatorului (în ordine): `T3`; `S9`.
2. Mutări legale (6): `S3`, `S4`, `S5`, `S10`, `SV`, `SM`. Introduci `S10`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (10): `T5`, `TV`, `TM`, `S3`, `S4`, `S5`, `SV`, `SM`, `RV`, `C8`. Introduci `C8`; răspunsul calculatorului (în ordine): `C4`; `C9`.

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Numărul de levate.

## Sfârșitul jocului

După a 12-a levată câștigă cine are mai multe.

## Note despre AI

Informație ascunsă: vezi doar mâna ta; AI-ul judecă după propria mână și cărțile jucate.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă pentru doi jucători în spiritul lui Hukm/ganjifa mogulă.
