# Patru în linie (`connect4`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play connect4 --seed 1`. Sursa implementării: `polytabletop-games/src/connect4.rs`.

> Rezumat din `polytabletop list`: Lasă discuri în coloane; primul cu patru în linie (orizontal, vertical sau diagonal) câștigă. Mutări: 1–7 (numărul coloanei).

## Scop

Primul care aliniază patru discuri ale sale (orizontal, vertical sau diagonal) câștigă.

## Pregătire

Grilă de 7 coloane × 6 rânduri, goală. Jucătorul 1 începe.

## Desfășurarea turei

Lași un disc într-o coloană care nu e plină; discul cade pe cel mai de jos loc liber.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul coloanei, `1`–`7`, de exemplu `4`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (7): `4`, `3`, `5`, `2`, `6`, `1`, `7`. Introduci `4`; răspunsul calculatorului (în ordine): `4`.
2. Mutări legale (7): `4`, `3`, `5`, `2`, `6`, `1`, `7`. Introduci `3`; răspunsul calculatorului (în ordine): `2`.
3. Mutări legale (7): `4`, `3`, `5`, `2`, `6`, `1`, `7`. Introduci `5`; răspunsul calculatorului (în ordine): `6`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Patru discuri în linie (victorie) sau grila plină fără patru în linie (remiză).

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 16).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 16).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Numele comercial este o marcă; regulile sunt cele clasice.
