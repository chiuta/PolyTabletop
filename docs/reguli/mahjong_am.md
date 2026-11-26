# American Mah Jongg (`mahjong_am`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mahjong_am --seed 1`. Sursa implementării: `polytabletop-games/src/mahjong_am.rs`.

> Rezumat din `polytabletop list`: Mah jongg simplificat pentru doi: 152 de piese cu jokeri, pung-uri expuse, 4 seturi + pereche sau șapte perechi. Mutări: arunc 5k, pung, pas, mahjong.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Simplificat: fără cartea NMJL, fără Charleston, fără kong/quint; doi jucători.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Simplificat: fără cartea NMJL, fără Charleston, fără kong/quint; doi jucători.
