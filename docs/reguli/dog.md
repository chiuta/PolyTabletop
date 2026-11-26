# Dog (curse cu cărți) (`dog`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dog --seed 1`. Sursa implementării: `polytabletop-games/src/dog.rs`.

> Rezumat din `polytabletop list`: Cursă pe inel cu cărți pentru doi: inel de 64 de câmpuri, 4 piese, 110 cărți. A sau K scot o piesă; cărțile mută piese după valoare (A = 1 sau 11, 4 = înainte sau înapoi, 7 = șapte pași de câte 1, J = schimb cu o piesă a adversarului, joker = orice carte). Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată. Câștigă cine aduce primul toate cele 4 piese în casă. Mutări: K:intra, 5:p2, 4:p1-, 7:p1, J:p1<>o2, pas. Mâini ascunse.

## Reguli implementate

Cursă pe inel cu cărți pentru doi: inel de 64 de câmpuri, 4 piese, 110 cărți. A sau K scot o piesă; cărțile mută piese după valoare (A = 1 sau 11, 4 = înainte sau înapoi, 7 = șapte pași de câte 1, J = schimb cu o piesă a adversarului, joker = orice carte). Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată. Câștigă cine aduce primul toate cele 4 piese în casă. Mutări: K:intra, 5:p2, 4:p1-, 7:p1, J:p1<>o2, pas. Mâini ascunse.

## Variante alese și limite

Adaptare pentru doi jucători a jocului de familie; 7-ul se joacă pas cu pas; fără echipe și fără schimb de cărți între parteneri.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
