# Mozaic (plăcuțe cu drumuri, cetăți, mănăstiri) (`mozaic`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mozaic --seed 1`. Sursa implementării: `polytabletop-games/src/mozaic.rs`.

> Rezumat din `polytabletop list`: Joc original, 65 de plăcuțe: tragi o plăcuță și o lipești cu muchiile potrivite (F câmp, R drum, C cetate; pune(x,y)r1), apoi poți pune un pion pe un drum, o cetate sau o mănăstire (drum:N, oras:E, manastire, sau fara). Cetate închisă 2/plăcuță (+2 per scut), drum 1/plăcuță, mănăstire 9; majoritatea ia punctele, la final se numără și ce-i neterminat. Fără câmpuri/fermieri. Dacă plăcuța nu se potrivește: arunca.

## Scop

Aduni cele mai multe puncte din cetăți, drumuri și mănăstiri.

## Pregătire

65 de plăcuțe originale. Muchiile unei plăcuțe: `F` câmp, `R` drum, `C` cetate, în ordinea N E S V. Plăcuța de start este în (0,0); x spre est, y spre nord. Fiecare jucător are 7 pioni.

## Desfășurarea turei

Tragi o plăcuță și o lipești de cele existente cu muchiile identice, cu rotația 0–3 sferturi de tură în sens orar. Dacă nu se poate așeza nicăieri: `arunca`. Apoi poți pune un pion pe un element al plăcuței (dacă acel element, cu tot ce e legat de el, n-are pion) sau `fara`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `pune(x,y)rK`, de exemplu `pune(1,0)r1`.
* Pion: `drum:N`, `oras:E`, `manastire`, `fara`.
* `arunca` (plăcuța nu se potrivește nicăieri).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (4): `pune(0,-1)r2`, `pune(0,1)r1`, `pune(0,1)r2`, `pune(0,1)r3`. Introduci `pune(0,-1)r2`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (2): `fara`, `oras:E`. Introduci `oras:E`; răspunsul calculatorului (în ordine): `pune(0,1)r2`; `oras:S`.
3. Mutări legale (9): `pune(-1,-1)r1`, `pune(-1,0)r0`, `pune(-1,0)r2`, `pune(-1,0)r3`, `pune(0,-2)r0`, `pune(1,-1)r3`, `pune(1,0)r0`, `pune(1,0)r1`, `pune(1,0)r2`. Introduci `pune(1,0)r2`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Fără câmpuri/fermieri: pionii se pun doar pe drumuri, cetăți și mănăstiri.

## Punctaj

Cetate terminată: 2 pe plăcuță + 2 pe scut; drum terminat: 1 pe plăcuță; mănăstire înconjurată de 8 plăcuțe: 9. Majoritatea pionilor ia punctele (egalitate: ambii); pionii se întorc. La final se numără și elementele neterminate (cetate 1 pe plăcuță + 1 pe scut, drum 1 pe plăcuță, mănăstire 1 + vecinii).

## Sfârșitul jocului

Când plăcuțele s-au terminat; câștigă cine are mai multe puncte.

## Note despre AI

Plăcuțele se trag la întâmplare (reproductibil cu `--seed`); AI-ul (adâncime 2) nu cunoaște ordinea pachetului și la evaluare ignoră plăcuța trasă pentru adversar.

Stil AI în cod: alpha-beta cu adâncime fixă 2.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Fără câmpuri/fermieri; set de plăcuțe original, numele și textele proprii.
