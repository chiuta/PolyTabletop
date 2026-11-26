# Crima cu ochiade (Wink Murder) (`crima`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play crima --seed 1`. Sursa implementării: `polytabletop-games/src/crima.rs`.

> Rezumat din `polytabletop list`: Zece suspecți în cerc. J1 = ucigașul (sunt 4), face cu ochiul și un suspect moare (clipesc 7); J2 = detectivul primește un martor (4 suspecți, unul e ucigașul) și acuză (acuz 3) sau așteaptă (astept). Acuzație corectă = detectivul câștigă; 3 greșeli sau 2 suspecți rămași = ucigașul.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pe ture cu zece suspecți, ucigaș și detectiv.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare pe ture cu zece suspecți, ucigaș și detectiv.
