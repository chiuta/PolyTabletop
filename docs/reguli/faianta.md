# Faianța (variantă originală) (`faianta`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play faianta --seed 1`. Sursa implementării: `polytabletop-games/src/faianta.rs`.

> Rezumat din `polytabletop list`: Draft de plăcuțe pe modele, 2 jucători: iei toate plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru (primul ia jetonul de start, −1) și le pui pe o linie de model (1–5, o singură culoare, nu dacă peretele are deja culoarea) sau pe podea (−1…−3). La sfârșitul rundei (mutarea forțată decontare) liniile pline urcă pe perete 5×5 și punctează adiacența; jocul se oprește la un rând complet; bonus rând +2, coloană +7, culoare +10. Mutări: f3 T 2, c O p, decontare.

## Scop

Aduni cele mai multe puncte completând peretele 5×5 cu plăcuțe.

## Pregătire

Cinci culori de plăcuțe (T turcoaz, O ocru, G grena, I indigo, A alb), câte 20, într-o pungă. Fiecare rundă: 5 manufacturi primesc câte 4 plăcuțe din pungă.

## Desfășurarea turei

Pe rând, un jucător ia toate plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru. Cine ia primul din centru ia jetonul de start (cade pe podea, −1) și începe runda următoare. Plăcuțele se pun pe una din cele 5 linii de model (capacități 1–5; o linie ține o singură culoare; nu poți pune o culoare dacă rândul de perete corespunzător o are deja) sau pe podea.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Din manufactură: `f3 T 2` (manufactura 3, culoarea T, linia 2).
* Din centru: `c O p` (`p` = podea).
* `decontare` (forțată, la sfârșitul rundei).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (102): `f1 T 1`, `f1 T 2`, `f1 T 3`, `f1 T 4`, `f1 T 5`, `f1 T p`, `f1 O 1`, `f1 O 2`, `f1 O 3`, `f1 O 4` …. Introduci `f1 T 1`; răspunsul calculatorului (în ordine): `f3 A 2`.
2. Mutări legale (65): `f2 T 2`, `f2 T 3`, `f2 T 4`, `f2 T 5`, `f2 T p`, `f2 G 2`, `f2 G 3`, `f2 G 4`, `f2 G 5`, `f2 G p` …. Introduci `f5 T 4`; răspunsul calculatorului (în ordine): `f4 T 1`.
3. Mutări legale (29): `f2 T 2`, `f2 T 3`, `f2 T 4`, `f2 T 5`, `f2 T p`, `f2 G 2`, `f2 G 3`, `f2 G 5`, `f2 G p`, `f2 A 2` …. Introduci `c A p`; răspunsul calculatorului (în ordine): `c I 3`.

## Reguli speciale și variante alese

* Podeaua penalizează −1, −1, −2, −2, −2, −3, −3.
* Culoarea c a rândului r merge pe coloana (c+r) mod 5 a peretelui.

## Punctaj

La `decontare`, fiecare linie plină trimite o plăcuță pe perete și punctează: 1 dacă e izolată, altfel lungimea șirului orizontal și/sau vertical. Bonus final: +2 rând complet, +7 coloană completă, +10 culoare completă.

## Sfârșitul jocului

Când cineva completează un rând al peretelui.

## Note despre AI

Informație vizibilă pentru ambii; singura întâmplare este extragerea din pungă. AI-ul (adâncime 2) joacă pe informația publică.

Stil AI în cod: alpha-beta cu adâncime fixă 2.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Variantă originală: draft de plăcuțe; nume, culori și texte proprii.
