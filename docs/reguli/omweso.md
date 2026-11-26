# Omweso (`omweso`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play omweso --seed 1`. Sursa implementării: `polytabletop-games/src/omweso.rs`.

> Rezumat din `polytabletop list`: Mancala din Uganda: 4×8 gropi, două rânduri fiecare, câte 2 semințe (64). Alegi o groapă cu cel puțin 2 semințe și semeni în sens trigonometric prin cele 16 gropi ale tale; dacă ultima sămânță cade într-o groapă nevidă din rândul interior, cu ambele gropi adverse din coloană nevide, le capturezi; altfel continui să semeni (releu). Pierzi când nu mai ai nicio groapă cu 2 semințe. Mutări: i1…i8 (rândul interior), o1…o8 (exterior).

## Reguli implementate

Mancala din Uganda: 4×8 gropi, două rânduri fiecare, câte 2 semințe (64). Alegi o groapă cu cel puțin 2 semințe și semeni în sens trigonometric prin cele 16 gropi ale tale; dacă ultima sămânță cade într-o groapă nevidă din rândul interior, cu ambele gropi adverse din coloană nevide, le capturezi; altfel continui să semeni (releu). Pierzi când nu mai ai nicio groapă cu 2 semințe. Mutări: i1…i8 (rândul interior), o1…o8 (exterior).

## Variante alese și limite

Aranjarea inițială fixă (2 semințe pe groapă); fără semănat în sens orar și fără bonusuri de turneu. Releu infinit oprit după 400 de ture, 400 de mutări = câștigă cine are mai multe semințe. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
