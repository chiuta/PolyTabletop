# Mad Gab (`madgab`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play madgab --seed 1`. Sursa implementării: `polytabletop-games/src/madgab.rs`.

> Rezumat din `polytabletop list`: Puzzle fonetic cu cuvinte, în română, pentru doi: un șir de silabe fără sens sună, citit repede cu voce tare, ca o expresie obișnuită. Alegi expresia corectă dintre 4 (ghici 1…ghici 4), eventual după ajutor (se vede primul cuvânt, puncte mai puține). Corect = 2 puncte (1 cu ajutor); greșit = celălalt are o singură încercare pentru 1 punct. 8 runde; verific la final. Mutări: ghici N, ajutor, verific.

## Reguli implementate

Puzzle fonetic cu cuvinte, în română, pentru doi: un șir de silabe fără sens sună, citit repede cu voce tare, ca o expresie obișnuită. Alegi expresia corectă dintre 4 (ghici 1…ghici 4), eventual după ajutor (se vede primul cuvânt, puncte mai puține). Corect = 2 puncte (1 cu ajutor); greșit = celălalt are o singură încercare pentru 1 punct. 8 runde; verific la final. Mutări: ghici N, ajutor, verific.

## Variante alese și limite

30 de ghicitori scrise pentru acest joc, în română; AI-ul compară sunetele șirului cu variantele și nu citește indexul corect. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
