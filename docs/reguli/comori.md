# Vânătoarea de comori (`comori`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play comori --seed 1`. Sursa implementării: `polytabletop-games/src/comori.rs`.

> Rezumat din `polytabletop list`: Pe o grilă 8×8, cine ascunde alege celula (ascund d5); celălalt sapă (sap c3) și primește indiciul cald/rece după distanța Manhattan. 12 săpături (eșec = 13). Două runde cu roluri inversate; costul total mai mic câștigă.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pe grilă 8×8 cu indiciu cald/rece.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare pe grilă 8×8 cu indiciu cald/rece.
