# Fitil (variantă originală) (`fitil`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fitil --seed 1`. Sursa implementării: `polytabletop-games/src/fitil.rs`.

> Rezumat din `polytabletop list`: O Bombă în pachet: joci orice cărți, apoi tragi (trag). Fără Dezamorsare, bomba te scoate; cu ea o repui unde vrei (dezamorsez 0..N). Cărți: atac (adversarul trage de 2 ori), sari, amesteca, privire (primele 3), cere (adversarul îți dă o carte: dau X), pereche A|B|C (furi o carte la întâmplare). Informație ascunsă: vezi doar mâna ta, privirea ta și locul bombei puse de tine.

## Scop

Nu trage Bomba din pachet; ultimul rămas câștigă.

## Pregătire

Pachetul ascunde o singură Bombă. Cărți: Dezamorsare, Atac, Sari, Amestecă, Privire, Cere și trei feluri de Mâți. Fiecare are o mână inițială.

## Desfășurarea turei

Poți juca orice cărți, apoi trebuie să tragi o carte (încheie tura), dacă nu ai jucat Atac sau Sari. Dacă tragi Bomba fără Dezamorsare, pierzi; cu Dezamorsare o pui înapoi în pachet unde vrei (adâncimea 0 = deasupra).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `juc atac`, `juc sari`, `juc amesteca`, `juc privire`, `juc cere`.
* `pereche A`, `pereche B`, `pereche C` (două cărți identice: furi o carte la întâmplare).
* `trag` (încheie tura), `dezamorsez 0..N` (adâncimea), `dau X` (răspuns la `cere`).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (5): `trag`, `juc sari`, `juc amesteca`, `juc privire`, `juc cere`. Introduci `trag`; răspunsul calculatorului (în ordine): `pereche B`; `juc cere`.
2. Mutări legale (5): `dau atac`, `dau sari`, `dau amesteca`, `dau privire`, `dau cere`. Introduci `dau amesteca`; răspunsul calculatorului (în ordine): `trag`.
3. Mutări legale (5): `trag`, `juc atac`, `juc sari`, `juc privire`, `juc cere`. Introduci `juc cere`; răspunsul calculatorului (în ordine): `dau C`.

## Reguli speciale și variante alese

* Atac încheie tura fără tragere; adversarul trage de două ori. Un Atac peste un Atac adaugă 2 trageri celor rămase.
* Privire: vezi primele 3 cărți. Cere: adversarul îți dă o carte la alegerea lui.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine trage Bomba fără Dezamorsare pierde.

## Note despre AI

Informație ascunsă: vezi doar mâna ta, Privirea ta și poziția Bombei puse de tine. AI-ul nu citește mâna adversarului sau pachetul: estimează riscul din ce știe.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Fără cartea „Nu”.
