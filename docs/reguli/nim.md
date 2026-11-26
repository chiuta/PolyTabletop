# Nim cu grămezi (`nim`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play nim --seed 1`. Sursa implementării: `polytabletop-games/src/nim.rs`.

> Rezumat din `polytabletop list`: Patru grămezi (3, 4, 5, 6 bețe); iei oricâte bețe dintr-o singură grămadă; cine ia ultimul băț câștigă. Mutări: b3 (litera grămezii + număr).

## Scop

Iei ultimul băț (joc normal: cine ia ultimul băț câștigă).

## Pregătire

Patru grămezi (a, b, c, d) cu 3, 4, 5 și 6 bețe. J1 începe.

## Desfășurarea turei

Iei oricâte bețe vrei (cel puțin unul) dintr-o singură grămadă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Litera grămezii urmată de număr: `b3` = trei bețe din grămada b.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (18): `a1`, `a2`, `a3`, `b1`, `b2`, `b3`, `b4`, `c1`, `c2`, `c3` …. Introduci `a1`; răspunsul calculatorului (în ordine): `d3`.
2. Mutări legale (14): `a1`, `a2`, `b1`, `b2`, `b3`, `b4`, `c1`, `c2`, `c3`, `c4` …. Introduci `c2`; răspunsul calculatorului (în ordine): `b2`.
3. Mutări legale (10): `a1`, `a2`, `b1`, `b2`, `c1`, `c2`, `c3`, `d1`, `d2`, `d3`. Introduci `d3`; răspunsul calculatorului (în ordine): `b1`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Când ultimul băț este luat; cine îl ia câștigă.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (200 ms, adâncime maximă 4).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (200 ms, adâncime maximă 4).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
