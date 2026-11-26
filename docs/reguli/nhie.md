# Eu niciodată (`nhie`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play nhie --seed 1`. Sursa implementării: `polytabletop-games/src/nhie.rs`.

> Rezumat din `polytabletop list`: Joc de petrecere pentru doi, cu 5 degete fiecare și un profil ascuns de lucruri făcute. Spui o afirmație pe care n-ai făcut-o; celălalt răspunde sincer și coboară un deget dacă a făcut-o. Mutări: eu-niciodata-5, raspund.

## Scop

Păstrezi mai multe degete sau adversarul rămâne primul fără degete.

## Pregătire

5 degete fiecare; 16 afirmații cu probabilitate publică; fiecare jucător are un profil ascuns (ce a făcut) tras la început.

## Desfășurarea turei

Alegi o afirmație din cele pe care nu le-ai făcut (`eu-niciodata-N`), apoi `raspund` (mutare separată): celălalt răspunde după profilul lui și coboară un deget dacă a făcut-o.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `eu-niciodata-1`
* `raspund`

## Simplificări și note

Răspunsul e sincer (nu se poate minți). AI: alege afirmația cu cea mai mare probabilitate publică; nu citește profilul adversarului. Hot-seat: arată lista doar celui la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
