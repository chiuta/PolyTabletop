# Ghicește ce văd (I Spy) (`ispy`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ispy --seed 1`. Sursa implementării: `polytabletop-games/src/ispy.rs`.

> Rezumat din `polytabletop list`: Cine vede alege un obiect din cameră (vad masa) și spune litera lui; celălalt ghicește (este masa) sau cere indicii (indiciu culoare / loc / marime). Maxim 6 încercări (eșec = 7). Două runde cu roluri inversate; costul total mai mic câștigă.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare: indicii standardizate (culoare, loc, mărime).

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare: indicii standardizate (culoare, loc, mărime).
