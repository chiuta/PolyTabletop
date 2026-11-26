# Toc (jeu) (`toc`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play toc --seed 1`. Sursa implementării: `polytabletop-games/src/toc.rs`.

> Rezumat din `polytabletop list`: Cursă franceză cu cărți și popice pentru doi: inel de 80 de câmpuri, 4 piese, 54 de cărți. A sau K scot o piesă; cărțile mută după valoare; 4 merge numai înapoi; 7 = șapte pași de câte 1; valetul mută 11 sau schimbă; jokerul mută 15 sau scoate o piesă. Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată. Câștigă cine aduce primul toate cele 4 piese în casă. Mâini ascunse.

## Reguli implementate

Cursă franceză cu cărți și popice pentru doi: inel de 80 de câmpuri, 4 piese, 54 de cărți. A sau K scot o piesă; cărțile mută după valoare; 4 merge numai înapoi; 7 = șapte pași de câte 1; valetul mută 11 sau schimbă; jokerul mută 15 sau scoate o piesă. Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată. Câștigă cine aduce primul toate cele 4 piese în casă. Mâini ascunse.

## Variante alese și limite

Adaptare pentru doi jucători (în original pe echipe); 7-ul se joacă pas cu pas.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
