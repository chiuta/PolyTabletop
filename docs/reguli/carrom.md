# Carrom (discret, fără fizică) (`carrom`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play carrom --seed 1`. Sursa implementării: `polytabletop-games/src/carrom.rs`.

> Rezumat din `polytabletop list`: Tablă 9 × 9, 12 monede pe jucător, damă, patru buzunare în colțuri. Lovitura = coloană + direcție (L/S/R) + forță 1–3, traseu deterministic cu ricoșeu; monedă proprie băgată = +1 și mai tragi; moneda adversarului = punct pentru el; dama se acoperă la lovitura următoare (+3) sau revine; strikerul în buzunar = greșeală. Mutări: eS3, aL1.

## Scop

Ai mai multe puncte la final: bagi monede proprii în buzunare și acoperi dama.

## Pregătire

Tablă 9 × 9 cu patru buzunare în colțuri: 12 monede albe (Jucătorul 1, `x`), 12 negre (Jucătorul 2, `o`) și o damă roșie (`Q`). Fără fizică: lovitura este un traseu determinist pe grilă.

## Desfășurarea turei

O lovitură = coloana de start (a–i) pe rândul tău de bază (rândul 2 pentru J1, rândul 8 pentru J2; celula trebuie să fie liberă), direcția (`L` stânga-înainte, `S` drept înainte, `R` dreapta-înainte) și forța 1–3. Strikerul are un buget de 4·forța + 2 pași, merge câte o celulă și ricoșează la pereți; dacă următoarea celulă are o monedă, se oprește lângă ea și o împinge pe aceeași direcție cu ⌈bugetul rămas / 2⌉ celule. O monedă care ajunge într-un colț cade în buzunar.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Coloană + direcție + forță: `eS3`, `aL1`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (63): `bL1`, `bL2`, `bL3`, `bS1`, `bS2`, `bS3`, `bR1`, `bR2`, `bR3`, `cL1` …. Introduci `bL1`; răspunsul calculatorului (în ordine): `gL3`; `cR1`; `cR2`; `dR1`.
2. Mutări legale (63): `bL1`, `bL2`, `bL3`, `bS1`, `bS2`, `bS3`, `bR1`, `bR2`, `bR3`, `cL1` …. Introduci `eS2`; răspunsul calculatorului (în ordine): `gS1`.
3. Mutări legale (54): `bL1`, `bL2`, `bL3`, `bS1`, `bS2`, `bS3`, `bR1`, `bR2`, `bR3`, `cL1` …. Introduci `hR3`; răspunsul calculatorului (în ordine): `bS3`; `gR2`.

## Reguli speciale și variante alese

* Moneda proprie băgată: +1 și mai tragi o dată. Moneda adversarului: punctul merge la el și tura trece.
* Dama se „acoperă” dacă lovitura următoare bagă o monedă proprie (+3 pentru tine); altfel revine în centru.
* Strikerul băgat în buzunar = greșeală: o monedă proprie înapoi pe tablă (−1) și tura trece.
* Nicio monedă nu împinge alta în lanț; un singur contact pe lovitură.

## Punctaj

+1 monedă proprie băgată, +3 dama acoperită, punct pentru adversar când bagi moneda lui, −1 la greșeală.

## Sfârșitul jocului

Când un jucător și-a băgat toate monedele (inclusiv când adversarul le-a băgat pentru el) sau după 300 de lovituri; câștigă cine are mai multe puncte.

## Note despre AI

Informație completă, deterministă. Alpha-beta cu adâncime fixă 3.

Stil AI în cod: alpha-beta cu adâncime fixă 3.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Fără frecare, rotații sau unghiuri arbitrare; un singur striker; variantă discretă documentată.
