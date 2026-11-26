# Karuta (recitare vs AI, proverbe) (`karuta`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play karuta --seed 1`. Sursa implementării: `polytabletop-games/src/karuta.rs`.

> Rezumat din `polytabletop list`: Karuta cu 40 de proverbe românești: 16 cărți pe masă (8 pe parte, cu sfârșitul proverbului), 24 de citiri (8 false). Începutul se dezvăluie cuvânt cu cuvânt; pe rând: asteapta sau ia N (atingi cartea). Corect: cartea dispare (ținta de pe partea adversarului: îi trimiți una de-a ta). Greșit: adversarul îți trimite o carte. Câștigă cine rămâne primul fără cărți (sau are mai puține după toate citirile). Mutări: asteapta, ia 12.

## Scop

Rămâi primul fără cărți pe partea ta (sau ai mai puține după toate citirile).

## Pregătire

40 de proverbe românești. 16 cărți pe masă (8 pe partea fiecărui jucător), care arată doar sfârșitul proverbului. Cititul are 24 de proverbe: cele 16 de pe masă plus 8 „false” fără carte.

## Desfășurarea turei

Începutul proverbului se dezvăluie cuvânt cu cuvânt. Fiecare jucător, pe rând (cine e primul alternează), poate `asteapta` sau `ia N` (atingi cartea cu numărul N, de pe oricare parte). Dacă ambii așteaptă, se dezvăluie următorul cuvânt; după ultimul cuvânt, un proverb neluat se închide.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `asteapta`.
* `ia N` (N = numărul cărții), de exemplu `ia 12`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (17): `asteapta`, `ia 2`, `ia 4`, `ia 5`, `ia 7`, `ia 11`, `ia 13`, `ia 14`, `ia 19`, `ia 21` …. Introduci `asteapta`; răspunsul calculatorului (în ordine): `ia 7`; `asteapta`.
2. Mutări legale (16): `asteapta`, `ia 2`, `ia 4`, `ia 5`, `ia 11`, `ia 13`, `ia 14`, `ia 19`, `ia 21`, `ia 22` …. Introduci `ia 21`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (16): `asteapta`, `ia 2`, `ia 4`, `ia 5`, `ia 11`, `ia 13`, `ia 14`, `ia 19`, `ia 21`, `ia 22` …. Introduci `ia 40`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Atingere corectă: cartea se scoate; dacă era de pe partea adversarului, îi trimiți una de-a ta.
* Atingere greșită (alt proverb sau proverb fals): adversarul îți trimite o carte din ale lui.

## Punctaj

Numărul de cărți rămase pe partea fiecăruia.

## Sfârșitul jocului

Câștigă primul care rămâne fără cărți; dacă se termină cele 24 de citiri, câștigă cine are mai puține cărți (egal = remiză).

## Note despre AI

Informație ascunsă: ordinea citirii și proverbul curent întreg. AI-ul vede doar începutul dezvăluit, cărțile de pe masă și lista celor 40 de proverbe; ia cartea doar când începutul dezvăluit duce la un singur candidat sigur.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Varianta noastră folosește 40 de proverbe românești, nu cele 100 de poezii Hyakunin Isshu; recitare contra AI.
