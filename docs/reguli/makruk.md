# Makruk (șah thailandez) (`makruk`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play makruk --seed 1`. Sursa implementării: `polytabletop-games/src/makruk.rs`.

> Rezumat din `polytabletop list`: 8×8: Khun (rege), Met (un pas diagonal), Khon (un pas diagonal sau înainte), cal, turn; pionii mută și capturează cu un pas și se promovează în Met pe rândul 6. Fără rocadă. Mat = victorie; pat = remiză; fără pioni, numărare limitată (8–64 de mutări) = remiză. Mutări: e3e4.

## Scop

Dai mat regelui (Khun) adversar.

## Pregătire

Tablă 8×8. Jucătorul 1 = alb (jos, rândurile 0–2), Jucătorul 2 = negru (sus). Piese: Khun (rege), Met (regină: un pas diagonal), Khon (nebun: un pas diagonal sau înainte), Ma (cal), Rua (turn), pion.

## Desfășurarea turei

Muți o piesă. Fără rocadă, fără pas dublu, fără en passant.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* De la, la: `e3e4`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (23): `a1a2`, `b1d2`, `c1d2`, `c1b2`, `c1c2`, `d1e2`, `d1c2`, `e1e2`, `e1f2`, `e1d2` …. Introduci `e3e4`; răspunsul calculatorului (în ordine): `f6f5`.
2. Mutări legale (24): `e4f5`, `a1a2`, `b1d2`, `c1d2`, `c1b2`, `c1c2`, `d1e2`, `d1c2`, `e1e2`, `e1f2` …. Introduci `f1e2`; răspunsul calculatorului (în ordine): `a6a5`.
3. Mutări legale (21): `e4f5`, `a1a2`, `b1d2`, `c1d2`, `c1b2`, `c1c2`, `d1c2`, `e1f1`, `e1f2`, `e1d2` …. Introduci `e4e5`; răspunsul calculatorului (în ordine): `d6e5`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Pionul mută și capturează cu un pas și se promovează automat în Met pe al șaselea rând (din partea lui).
* Numărare simplificată: când nu mai există pioni, se numără mutările (limită după materialul părții mai puternice: 2 ture 8, 1 turn 16, 2 nebuni 22, 2 cai 32, 1 nebun 44, altfel 64; ambele părți cu piese = 64).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Mat = victorie; pat = remiză; numărarea terminată = remiză; 200 de semi-mutări fără captură sau mutare de pion = remiză.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 12).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 12).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* În jocul real numărarea o pornește partea mai slabă (poate s-o oprească sau reia), începe de la numărul pieselor de pe tablă, iar „piece honour” se aplică doar când partea slabă a rămas cu regele gol; aici numărul rulează automat pentru toate pozițiile fără pioni.
