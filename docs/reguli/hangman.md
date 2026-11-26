# Spânzurătoarea (Hangman) (`hangman`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hangman --seed 1`. Sursa implementării: `polytabletop-games/src/hangman.rs`.

> Rezumat din `polytabletop list`: Cine pune alege în secret un cuvânt din listă (cuvant casa); celălalt ghicește litere (litera a) sau cuvântul (ghicesc casa). 7 greșeli = pierdută. Reușită = 1 + (7 − greșeli) puncte. Două runde cu roluri inversate; cine are mai multe puncte câștigă.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare cu listă fixă de cuvinte; doi jucători alternează rolurile.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare cu listă fixă de cuvinte; doi jucători alternează rolurile.
