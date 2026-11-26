# Mariáš (`mariash`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mariash --seed 1`. Sursa implementării: `polytabletop-games/src/mariash.rs`.

> Rezumat din `polytabletop list`: Mariáš ceh pentru doi: forhont, talon de două cărți, atu, hlášky 20/40, ultima levată +10; 6 mâini. Mutări: las 7g (două cărți), atu inimi, apoi cartea (ex. Ag) sau Kg hlaska.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pentru doi jucători: fără betl, durch, sedma și flekuri; 6 mâini.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare pentru doi jucători: fără betl, durch, sedma și flekuri; 6 mâini.
