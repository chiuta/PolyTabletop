# Dou dizhu (2 jucători) (`doudizhu`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play doudizhu --seed 1`. Sursa implementării: `polytabletop-games/src/dou_dizhu.rs`.

> Rezumat din `polytabletop list`: 54 de cărți (3…K, A, 2, jokeri X/Y); 17 fiecare, 20 ascunse nefolosite. Combinații: single, pereche, trio (+1 sau +pereche), scară de 5+ (34567), scară de perechi 3+ (334455), bombă (5555), rachetă (XY). Răspunzi cu același tip și lungime, mai mare, sau bombă/rachetă, ori pas. Cine rămâne fără cărți câștigă runda; cel mai bun din 3. Fără avioane și careu+2. Mutări: 5, 55, 555+3, 34567, 5555, XY, pas.

## Scop

Rămâi primul fără cărți în mână; cel mai bun din 3 runde.

## Pregătire

54 de cărți (3…K, A, 2 și doi jokeri; culoarea nu contează). Ordine: 3 < 4 < … < K < A < 2 < jokerul mic (X) < jokerul mare (Y). Fiecare primește 17 cărți; celelalte 20 rămân ascunse și nefolosite.

## Desfășurarea turei

Cine conduce joacă o combinație; celălalt trebuie să dea una de același tip și lungime, mai mare, sau o bombă ori racheta, ori trece. Dacă adversarul trece, cel care a jucat ultimul conduce din nou.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Single `5`, pereche `55`, trio `555`, trio + single `555+3`, trio + pereche `555+33`.
* Scară de cel puțin 5 cărți: `34567`; scară de cel puțin 3 perechi: `334455`.
* Bombă `5555`, rachetă `XY`, `pas`. (10 se scrie `T`.)

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (22): `3`, `4`, `44`, `5`, `55`, `6`, `66`, `7`, `8`, `T` …. Introduci `3`; răspunsul calculatorului (în ordine): `8`.
2. Mutări legale (8): `T`, `J`, `K`, `2`, `X`, `Y`, `XY`, `pas`. Introduci `X`; răspunsul calculatorului (în ordine): `pas`.
3. Mutări legale (17): `4`, `44`, `5`, `55`, `6`, `66`, `7`, `8`, `T`, `J` …. Introduci `445566`; răspunsul calculatorului (în ordine): `556677`.

## Reguli speciale și variante alese

* Fără licitație (nu există stăpân); conduce J1 în runda 1, J2 în runda 2, J1 în a 3-a.

## Punctaj

Câte o victorie pe rundă.

## Sfârșitul jocului

După 3 runde câștigă cine a câștigat mai multe.

## Note despre AI

Informație ascunsă: vezi doar mâna ta și numărul de cărți ale adversarului. AI-ul alege după propria mână (estimare a numărului de ture și a cărților mari păstrate).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără avioane (trio-uri consecutive) și fără careu + 2 cărți.
* Doi jucători, fără licitație; cele 20 de cărți rămase nu se folosesc.
