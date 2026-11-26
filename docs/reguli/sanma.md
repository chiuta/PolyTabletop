# Sanma (mahjong în trei, adaptat la doi) (`sanma`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sanma --seed 1`. Sursa implementării: `polytabletop-games/src/sanma.rs`.

> Rezumat din `polytabletop list`: Mahjong riichi cu schimbările sanma: 108 piese (fără 2m–8m), fără chi, piesele Nord pot fi declarate kita (dora bonus, tragere de înlocuire), indicatorul de dora face cerc 1m↔9m. Riichi, ron, tsumo, furiten și punctaj ca la riichi. Mutări: trage, 5m, riichi 5p, tsumo, ankan, kakan, kita, trage-moarta, ron, pon, kan, pas, urmatoarea.

## Reguli implementate

Mahjong riichi cu schimbările sanma: 108 piese (fără 2m–8m), fără chi, piesele Nord pot fi declarate kita (dora bonus, tragere de înlocuire), indicatorul de dora face cerc 1m↔9m. Riichi, ron, tsumo, furiten și punctaj ca la riichi. Mutări: trage, 5m, riichi 5p, tsumo, ankan, kakan, kita, trage-moarta, ron, pon, kan, pas, urmatoarea.

## Variante alese și limite

Bazat pe motorul `riichi` (2 jucători, nu 3): zid de 108 piese fără 2m–8m, fără chi, kita (nord) ca dora bonus cu tragere de înlocuire; indicator dora 1m↔9m; fără chankan pe kita. Tragerile din zid sunt pași forțați (AI-ul nu vede zidul). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
