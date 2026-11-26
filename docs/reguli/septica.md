# Septica (`septica`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play septica --seed 1`. Sursa implementării: `polytabletop-games/src/septica.rs`.

> Rezumat din `polytabletop list`: Pachet de 32 de cărți, 4 în mână. Preiei masa cu aceeași valoare ca prima carte sau cu 7 / 8; poți tăia la rândul tău; cine dă pas pierde masa. 10 și A = 1 punct (8 în total), peste 4 câștigi. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 9h, 7s, pas.

## Scop

Aduni cât mai multe puncte: cărțile de 10 și Așii valorează 1 punct fiecare (8 în total). La final ai nevoie de peste 4.

## Pregătire

Pachet de 32 de cărți (7–A). Fiecare jucător primește 4 cărți; restul formează stocul. Jucătorul 1 conduce prima masă.

## Desfășurarea turei

Cine conduce pune o carte pe masă. Adversarul poate prelua masa cu o carte de aceeași valoare ca prima carte de pe masă sau cu un 7 ori un 8 (acestea preiau orice). Apoi primul poate tăia la rândul lui, și tot așa. Cine nu poate sau nu vrea să taie spune `pas`: ultimul care a pus o carte ia toată masa. Câștigătorul mesei trage primul până la 4 cărți și conduce următoarea masă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Carte: valoare + culoare: `9h`, `7s`, `10d` (culori: h, d, s, c; valori 7–10, J, Q, K, A).
* `pas`: apare doar când pe masă sunt cărți și vrei să lași masa adversarului.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (4): `7d`, `Jh`, `10d`, `Qs`. Introduci `7d`; răspunsul calculatorului (în ordine): `pas`.
2. Mutări legale (4): `Jh`, `10d`, `Qs`, `8s`. Introduci `Qs`; răspunsul calculatorului (în ordine): `pas`.
3. Mutări legale (4): `Jh`, `10d`, `8s`, `Qd`. Introduci `Qd`; răspunsul calculatorului (în ordine): `pas`.

## Reguli speciale și variante alese

* 7 și 8 preiau orice carte (convenție a implementării; unele surse dau doar 7-urile ca jokere).

## Punctaj

Puncte = cărțile de 10 și A din levatele câștigate (8 în total). Fără bonus pentru ultima levată.

## Sfârșitul jocului

Când stocul și mâinile sunt goale: peste 4 puncte câștigi; 4–4 este remiză.

## Note despre AI

Informație ascunsă: vezi doar mâna ta; afișajul arată doar numărul de cărți ale adversarului. Împărțirea și amestecul sunt făcute de joc cu generatorul propriu (reproductibil cu `--seed`). AI-ul evaluează doar ce vede (mâna lui, masa, scorul) și nu citește mâna adversarului. Pașii care ar dezvălui informație sunt separați în mutări distincte.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Cartea 8 ca joker este o convenție locală; majoritatea surselor atestă doar 7-ul (8 apare în variante cu trei jucători).
* Doi jucători (nu patru în echipe) și fără bonus pentru ultima levată.
