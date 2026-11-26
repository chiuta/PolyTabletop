# Parqués (Colombia) (`parques`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play parques --seed 1`. Sursa implementării: `polytabletop-games/src/parques.rs`.

> Rezumat din `polytabletop list`: Pistă de 68 de pătrate, 4 piese pe jucător, două zaruri. Din cárcel ieși doar cu pereche (toate piesele deodată); pereche = încă o aruncare, trei perechi = o piesă direct în corona. Captura (obligatorie) trimite piesa adversă în cárcel, pe pătratele sigure nu se capturează; corona se atinge exact. Primul cu 4 piese în corona câștigă. Mutări: 12:5 (piesa de la progresul 12 avansează 5), sale, pasa.

## Reguli implementate

Pistă de 68 de pătrate, 4 piese pe jucător, două zaruri. Din cárcel ieși doar cu pereche (toate piesele deodată); pereche = încă o aruncare, trei perechi = o piesă direct în corona. Captura (obligatorie) trimite piesa adversă în cárcel, pe pătratele sigure nu se capturează; corona se atinge exact. Primul cu 4 piese în corona câștigă. Mutări: 12:5 (piesa de la progresul 12 avansează 5), sale, pasa.

## Variante alese și limite

Geometria pistei (68 de pătrate, progres 0–77) este proprie, fiindcă sursele publice diferă; regulile de bază (cárcel, perechi, captură, corona) sunt cele din regulamentul colombian. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație completă (fără cărți ascunse).

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
