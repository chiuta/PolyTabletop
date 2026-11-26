# Go (9×9) (`go`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play go --seed 1`. Sursa implementării: `polytabletop-games/src/go.rs`.

> Rezumat din `polytabletop list`: Scor pe arie (pietre + teritoriu), komi 7,5 pentru alb, ko simplu, fără sinucidere; două pass-uri consecutive încheie jocul. Mutări: e5 (coloane a–h și j, fără i; rânduri 1–9), pass.

## Scop

Controlezi mai multă suprafață (pietre + teritoriu propriu) decât adversarul, după punctajul pe arie și komi.

## Pregătire

Tablă 9×9, goală; Negru (Jucătorul 1, X) începe. Komi 7,5 pentru Alb.

## Desfășurarea turei

Pui o piatră pe o intersecție liberă sau treci. Grupurile fără libertăți sunt capturate.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Intersecția: `e5` (coloanele a–h și j, fără i; rândurile 1–9).
* `pass`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (82): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `g1`, `h1`, `j1`, `a2` …. Introduci `e5`; răspunsul calculatorului (în ordine): `f7`.
2. Mutări legale (80): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `g1`, `h1`, `j1`, `a2` …. Introduci `c3`; răspunsul calculatorului (în ordine): `g1`.
3. Mutări legale (78): `a1`, `b1`, `c1`, `d1`, `e1`, `f1`, `h1`, `j1`, `a2`, `b2` …. Introduci `pass`; răspunsul calculatorului (în ordine): `pass`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Ko simplu (nu poți reface imediat poziția anterioară) și interdicția de sinucidere.

## Punctaj

Punctaj pe arie (chinezesc): pietre + teritoriu; Alb primește komi 7,5.

## Sfârșitul jocului

După două pass-uri consecutive sau la limita de 260 de mutări.

## Note despre AI

Informație completă. MCTS (Monte-Carlo cu UCT), nu alpha-beta; simulările sunt limitate la 140 de mutări.

Stil AI în cod: MCTS (Monte-Carlo cu UCT).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Tablă mică 9×9; fără handicap; ko simplu (nu superko) și arie, nu teritoriu japonez.
