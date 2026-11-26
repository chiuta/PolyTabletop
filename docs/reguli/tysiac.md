# Tysiąc (`tysiac`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tysiac --seed 1`. Sursa implementării: `polytabletop-games/src/tysiac.rs`.

> Rezumat din `polytabletop list`: Tysiąc (o mie) pentru doi: licitație 100–300, hlášky/mariaje și două musiks; primul la 1000 sau 6 mâini. Mutări: licitez 110, pas, musik 1, las 9h, apoi cartea (ex. Ah) sau Kh marjas.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pentru doi (jocul clasic e pentru 3): două musiks, 6 mâini sau 1000 de puncte.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Adaptare pentru doi (jocul clasic e pentru 3): două musiks, 6 mâini sau 1000 de puncte.
