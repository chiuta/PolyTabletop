# Telefonul fără fir (`chinesewhispers`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chinesewhispers --seed 1`. Sursa implementării: `polytabletop-games/src/chinesewhispers.rs`.

> Rezumat din `polytabletop list`: Versiune în duel pentru doi: trimițătorul vede un mesaj de 4 cifre și îl lasă (las) sau strică o cifră cu ±1 (strica 2+). Receptorul vede mesajul ajuns și suma de control (suma cifrelor originale mod 10) și arată cifra stricată (restaur 1–4) sau zice restaur 0. Restaurarea corectă aduce punct receptorului, altfel trimițătorului. 6 runde, rolurile alternează. Originalul e ascuns.

## Reguli implementate

Versiune în duel pentru doi: trimițătorul vede un mesaj de 4 cifre și îl lasă (las) sau strică o cifră cu ±1 (strica 2+). Receptorul vede mesajul ajuns și suma de control (suma cifrelor originale mod 10) și arată cifra stricată (restaur 1–4) sau zice restaur 0. Restaurarea corectă aduce punct receptorului, altfel trimițătorului. 6 runde, rolurile alternează. Originalul e ascuns.

## Variante alese și limite

Joc de petrecere în mai mulți reduce la un duel asimetric trimițător/receptor cu sumă de control publică.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
