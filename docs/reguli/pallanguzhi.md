# Pallanguzhi (Tamil Nadu) (`pallanguzhi`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pallanguzhi --seed 1`. Sursa implementării: `polytabletop-games/src/pallanguzhi.rs`.

> Rezumat din `polytabletop list`: 2 × 7 cupe, 5 semințe în fiecare. Ridici o cupă proprie și semeni trigonometric; apoi: cupa următoare ocupată = o ridici și continui; goală = capturezi cupa de după ea și tura se termină. Captură suplimentară: orice cupă care ajunge la exact 4 semințe e capturată. Fără semințe la mutare = jocul se termină, fiecare ia ce are pe partea lui; câștigă cine are peste 35. O singură rundă. Mutări: 1–7.

## Scop

Ai peste 35 de semințe la final.

## Pregătire

2 × 7 cupe, 5 semințe în fiecare (70 în total), câte un buzunar de capturi pe jucător.

## Desfășurarea turei

Alegi o cupă proprie nenulă, ridici toate semințele și le semeni câte una în sens trigonometric. După ultima sămânță, cupa următoare: dacă are semințe, le ridici și continui; dacă e goală, capturezi cupa de după ea (dacă are) și tura se termină.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul cupei proprii: `1`–`7`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (7): `1`, `2`, `3`, `4`, `5`, `6`, `7`. Introduci `1`; răspunsul calculatorului (în ordine): `7`.
2. Mutări legale (5): `1`, `4`, `5`, `6`, `7`. Introduci `5`; răspunsul calculatorului (în ordine): `7`.
3. Mutări legale (3): `5`, `6`, `7`. Introduci `7`; răspunsul calculatorului (în ordine): `2`.

## Reguli speciale și variante alese

* „Pasu”: orice sămânță pusă care face o cupă să aibă exact 4 semințe o capturează imediat (semănatul continuă).

## Punctaj

Semințele capturate plus cele rămase pe partea proprie la final.

## Sfârșitul jocului

Când jucătorul la mutare nu mai are semințe pe partea lui: fiecare păstrează ce are; câștigă cine are peste 35. După 600 de mutări se numără pe loc.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 6.

Stil AI în cod: alpha-beta cu adâncime fixă 6.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* O singură rundă (fără refacerea cupelor din runda a doua).
* 5 semințe și pasu la 4 sunt atestate doar în unele descrieri (6 semințe e cel mai des).
