# Mahjong taiwanez (`mahjong_tw`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mahjong_tw --seed 1`. Sursa implementării: `polytabletop-games/src/mahjong_tw.rs`.

> Rezumat din `polytabletop list`: Mahjong taiwanez pentru doi: 144 de piese cu 8 flori, mâini de 16 piese, câștigi cu 5 seturi și o pereche; florile se înlocuiesc din zidul mort și valorează tai. Mutări: trage, d5m, pung, kong, chi2m3m, mahjong, pas.

## Scop

Declari o mână câștigătoare (5 seturi + pereche) cu cât mai multe tai.

## Pregătire

144 de piese (cele 136 obișnuite + 8 flori); 16 piese pe jucător; zid mort de 16.

## Desfășurarea turei

`trage`, apoi arunci (`d5m`) sau declari `mahjong`/`kong5m`; adversarul poate lua aruncarea cu `mahjong`, `pung`, `kong`, `chi...` sau `pas`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `trage`
* `d5m`
* `mahjong`

## Simplificări și note

Variantă simplificată pentru doi, fără mâini speciale rare; punctaj în tai. AI-ul vede doar mâna lui, seturile expuse, florile și aruncările publice. Hot-seat: arată doar mâna jucătorului la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
