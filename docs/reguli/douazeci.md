# Douăzeci de întrebări (`douazeci`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play douazeci --seed 1`. Sursa implementării: `polytabletop-games/src/douazeci.rs`.

> Rezumat din `polytabletop list`: Cine gândește alege un obiect din listă (obiect caine); celălalt pune întrebări da/nu numerotate (intreb 7) sau încearcă un răspuns (este caine); maxim 20 între întrebări și încercări. Cost = câte folosește (eșec = 21). Două runde cu roluri inversate; costul total mai mic câștigă.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare cu listă fixă de obiecte și întrebări numerotate.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare cu listă fixă de obiecte și întrebări numerotate.
