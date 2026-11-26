# Mima (Charades) (`mima`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mima --seed 1`. Sursa implementării: `polytabletop-games/src/mima.rs`.

> Rezumat din `polytabletop list`: Adaptare fără mimă fizică: cine mimează alege un cuvânt (mimez delfin) și dă indicii pe rând (gest categorie / silabe / prima-litera / ultima-litera / lungime); celălalt ghicește (ghicesc delfin) sau cere mai-multe. Cost = indicii + greșeli (eșec = 8). Două runde; costul mai mic câștigă.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare fără mimă fizică: indicii standardizate.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare fără mimă fizică: indicii standardizate.
