# Pit (`pit`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pit --seed 1`. Sursa implementării: `polytabletop-games/src/pit.rs`.

> Rezumat din `polytabletop list`: Schimburi ascunse de cărți între doi jucători; corner = toate cele 6 cărți ale unei mărfi. Mutări: ofer grau 2, dau ovaz, schimb.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pentru doi, pe ture, cu 3 mărfi și schimburi ascunse; fără taur și urs.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare pentru doi, pe ture, cu 3 mărfi și schimburi ascunse; fără taur și urs.
