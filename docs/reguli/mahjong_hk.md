# Mahjong Hong Kong (minimum 3 fan) (`mahjong_hk`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mahjong_hk --seed 1`. Sursa implementării: `polytabletop-games/src/mahjong_hk.rs`.

> Rezumat din `polytabletop list`: Mahjong în stil Hong Kong pentru 2 jucători, 136 de piese fără flori; o mână câștigă doar cu cel puțin 3 fan (tragere proprie 1, închisă 1, toate secvențele 1, toate pungurile 3, o culoare + onoruri 3, culoare pură 6, doar onoruri 10, pung de dragon/vânt propriu/vântul rundei câte 1). Mutări: trage, d5m, mahjong, kong5m, trage-moarta, pung, kong, chi2m3m, pas.

## Reguli implementate

Mahjong în stil Hong Kong pentru 2 jucători, 136 de piese fără flori; o mână câștigă doar cu cel puțin 3 fan (tragere proprie 1, închisă 1, toate secvențele 1, toate pungurile 3, o culoare + onoruri 3, culoare pură 6, doar onoruri 10, pung de dragon/vânt propriu/vântul rundei câte 1). Mutări: trage, d5m, mahjong, kong5m, trage-moarta, pung, kong, chi2m3m, pas.

## Variante alese și limite

Variantă a motorului `mahjong` (2 jucători, 136 de piese, fără flori) cu minimum 3 fan; fără plăți în puncte și fără mâini-limită. Mână fără 3 fan nu se poate declara, deci multe mâini sfârșesc remiză. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
