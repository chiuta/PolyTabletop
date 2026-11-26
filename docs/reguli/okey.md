# Okey (2 jucători) (`okey`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play okey --seed 1`. Sursa implementării: `polytabletop-games/src/okey.rs`.

> Rezumat din `polytabletop list`: 106 plăci (4 culori × 1–13 × 2 + 2 okey false), indicator și joker. 15/14 plăci; tragi din teanc (trage) sau iei ultima aruncată (ia), apoi arunci (ata-r5). Câștigi închizând 14 plăci în serii (≥ 3 consecutive, aceeași culoare) și grupe (3–4 cu același număr și culori diferite): bitir-r5. Informație ascunsă: vezi mâna Jucătorului 1.

## Scop

Închizi primul mâna de 14 plăci în serii și grupe.

## Pregătire

106 de plăci: culorile `r` roșu, `y` galben, `b` albastru, `k` negru × numerele 1–13 în două exemplare, plus 2 okey false (`f`). Se întoarce un indicator; okey (jokerul) este placa de aceeași culoare cu numărul următor (13 → 1), ambele ei exemplare. Okey-ul fals ține locul okey-ului, dar ca placă obișnuită. Jucătorul 1 primește 15 plăci, Jucătorul 2 primește 14.

## Desfășurarea turei

Jucătorul 1 începe aruncând o placă. La rândul tău tragi din teanc (`trage`) sau iei ultima placă aruncată de adversar (`ia`), apoi arunci o placă (`ata-r5`).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trage`, `ia`.
* `ata-r5`: arunci placa roșie 5; `ata-f`: okey-ul fals.
* `bitir-r5`: arunci r5 și închizi mâna (doar dacă cele 14 plăci rămase sunt complete).

## Reguli speciale și variante alese

* Serie: cel puțin 3 numere consecutive de aceeași culoare (1 poate urma după 13, dar nu se trece mai departe).
* Grupă: 3–4 plăci cu același număr și culori diferite.
* Jokerul ține loc de orice placă.

## Punctaj

O singură mână, fără punctaj.

## Sfârșitul jocului

Câștigă cine închide; dacă teancul se golește, remiză.

## Note despre AI

Informație ascunsă: AI-ul evaluează doar mâna lui, plăcile aruncate (publice) și numărul plăcilor adversarului; placa abia trasă nu e folosită la alegere. Alpha-beta cu adâncime 1.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; mâna afișată este a jucătorului la mutare, cu un banner de predare a ecranului.

## Diferențe față de regulile tradiționale

* Adaptare pentru 2 jucători în loc de 4; fără punctaj pe rânduri, fără închiderea cu perechi și fără dublarea pentru închidere cu okey.
