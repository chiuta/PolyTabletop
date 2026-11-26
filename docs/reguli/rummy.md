# Remi (Rummy, 13 cărți) (`rummy`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play rummy --seed 1`. Sursa implementării: `polytabletop-games/src/rummy.rs`.

> Rezumat din `polytabletop list`: 2 jucători, 13 cărți, un pachet: tragi (stoc sau talon) și arunci; închizi când cele 13 cărți rămase formează secvențe și seturi (cel puțin o secvență; aranjare automată). Penalizare = punctele libere; meci până la 101. Mutări: stoc, talon, Ah, 10c, inchide-Ah, urmatoarea.

## Scop

Închizi cu cele 13 cărți rămase aranjate în secvențe și seturi, cu cel puțin o secvență; la meci ai cele mai puține puncte de penalizare.

## Pregătire

Un pachet de 52 de cărți, fără jokeri. Fiecare primește 13 cărți; una se întoarce pe talon, restul formează stocul (25 de cărți).

## Desfășurarea turei

Tragi o carte din stoc (`stoc`) sau iei cartea de deasupra talonului (`talon`), apoi arunci o carte (rang + culoare). Cartea luată din talon nu poate fi aruncată în aceeași tură.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `stoc`, `talon`.
* Aruncare: `Ah`, `10c` (rang + culoare c/d/h/s).
* Închidere: `inchide-Ah` (arunci cartea și arăți combinațiile); apare doar dacă există o aranjare.
* `urmatoarea` (după o mână încheiată).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `stoc`, `talon`. Introduci `stoc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (14): `3d`, `4c`, `3c`, `8d`, `3h`, `2h`, `6d`, `Jh`, `3s`, `4h` …. Introduci `Jh`; răspunsul calculatorului (în ordine): `stoc`; `Kc`.
3. Mutări legale (2): `stoc`, `talon`. Introduci `talon`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Secvențe: cel puțin 3 cărți consecutive de aceeași culoare; asul este mic sau mare, fără ocolire K-A-2.
* Seturi: 3–4 cărți de același rang, culori diferite.
* Jocul găsește singur aranjarea optimă.

## Punctaj

Cărțile rămase în afara combinațiilor: 2–10 cât scriu, J/Q/K/A = 10, maxim 80. Cine închide primește 0; adversarul își adaugă punctele. Dacă stocul se golește fără închidere, ambii adaugă punctele proprii.

## Sfârșitul jocului

Meciul se termină când un jucător ajunge la 101 puncte de penalizare; câștigă cine are mai puține (egal = remiză).

## Note despre AI

Informație ascunsă: afișajul arată mâna ta (cu aranjarea automată) și numărul de cărți ale adversarului; la sfârșitul mâinii se arată ambele. AI-ul (adâncime 1) folosește doar mâna lui, talonul public și scorurile; cartea trasă din stoc e înlocuită cu o valoare medie. Nu ține minte ce a luat adversarul din talon.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără jokeri, fără etalare progresivă și fără lipirea cărților la combinațiile adversarului.
* Meci la 101 de puncte de penalizare.
