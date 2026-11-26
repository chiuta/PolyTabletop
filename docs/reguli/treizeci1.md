# Treizeci și unu (31) (`treizeci1`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play treizeci1 --seed 1`. Sursa implementării: `polytabletop-games/src/treizeci1.rs`.

> Rezumat din `polytabletop list`: 3 cărți în mână, A=11, figuri=10; valoarea = suma celei mai bune culori (trei la fel = 30). Tragi (trage) sau iei din aruncate (ia), apoi arunci (arunca 5h), sau bați (bat): adversarul mai are o tură, apoi cea mai mică mână pierde o viață (bătătorul singur cel mai mic: două). 31 = adversarul pierde o viață. 3 vieți. Informație ascunsă: vezi doar mâna ta. Mutări: trage, ia, bat, arunca Kd, sfarsit.

## Scop

Pierzi mai puține vieți decât adversarul; cine rămâne fără vieți pierde.

## Pregătire

Pachet de 52, 3 cărți fiecare, una se întoarce în teancul de aruncate. A = 11, figurile = 10, celelalte cât scriu. Mâna valorează suma cărților de aceeași culoare (cea mai mare); trei cărți de aceeași valoare = 30. 3 vieți fiecare.

## Desfășurarea turei

Tragi (`trage`, din stoc, pe nevăzute) sau iei (`ia`, cartea de sus din aruncate), apoi arunci (`arunca 5h`; nu poți arunca cartea tocmai luată din aruncate) sau bați (`bat`): oprești runda; adversarul mai are o tură, apoi mâinile se arată.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trage`, `ia`.
* `arunca 5h` (rang + culoare).
* `bat`, `sfarsit` (formală).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `trage`, `ia`, `bat`. Introduci `trage`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (4): `arunca Qs`, `arunca 3d`, `arunca 6c`, `arunca Jc`. Introduci `arunca 6c`; răspunsul calculatorului (în ordine): `trage`; `arunca 4h`.
3. Mutări legale (3): `trage`, `ia`, `bat`. Introduci `bat`; răspunsul calculatorului (în ordine): `trage`; `arunca 6d`; `bat`.

## Reguli speciale și variante alese

* Cine face 31 (după aruncare sau la împărțire) încheie imediat runda și adversarul pierde o viață.
* La arătare, mâna mai mică pierde o viață; cine a bătut și are strict cea mai mică mână pierde două; dacă cel care a bătut egalează, pierde adversarul. Fără bătaie și stoc epuizat: egalitatea nu costă nimic.

## Punctaj

Vieți (3 la start).

## Sfârșitul jocului

Când un jucător rămâne fără vieți.

## Note despre AI

Informație ascunsă: vezi doar mâna ta. AI-ul vede doar mâna lui și aruncatele; cartea trasă din stoc nu intră în evaluare (valoare medie), iar rezultatul arătării nu e vizibil în evaluare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Doi jucători, vieți în loc de monede, trei la fel = 30, stoc epuizat = arătare.
