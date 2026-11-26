# Lasca (`lasca`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play lasca --seed 1`. Sursa implementării: `polytabletop-games/src/lasca.rs`.

> Rezumat din `polytabletop list`: Dama lui Emanuel Lasker pe 7×7: piesa capturată trece la baza coloanei tale, se ia doar piesa de deasupra; ofițerii mută și capturează în toate direcțiile; captura e obligatorie. Mutări: a1-b2, a1xc3xe5.

## Scop

Adversarul rămâne fără mutări (nicio coloană a lui nu mai are piesa de deasupra liberă).

## Pregătire

Tablă 7×7 cu 25 de pătrate întunecate; câte 11 soldați pe parte; J1 jos mută primul.

## Desfășurarea turei

Soldatul mută o căsuță pe diagonală înainte; ofițerul (soldat ajuns pe ultimul rând) în toate cele patru direcții. Captura: sari peste o coloană adversă; doar piesa de deasupra trece la baza coloanei tale.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `a1-b2`
* `a1xc3xe5`

## Simplificări și note

Remiză după 60 de semi-mutări fără captură și fără mutare de soldat. AI: alfa-beta.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
