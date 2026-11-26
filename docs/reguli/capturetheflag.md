# Capture the Flag (`capturetheflag`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play capturetheflag --seed 1`. Sursa implementării: `polytabletop-games/src/capturetheflag.rs`.

> Rezumat din `polytabletop list`: Tablă 9×7, jumătate de teritoriu fiecare (coloana e e neutră). Câte 3 alergători; un pas ortogonal pe mutare (b2-b3). Un alergător care intră în teritoriul inamic lângă un apărător, sau un intrus lângă piesa ta în teritoriul tău, e atins și stă 3 mutări în închisoare. Ajungi la steagul inamic și îl aduci în teritoriul tău ca să câștigi; purtătorul atins pierde steagul. 160 de mutări: remiză.

## Reguli implementate

Tablă 9×7, jumătate de teritoriu fiecare (coloana e e neutră). Câte 3 alergători; un pas ortogonal pe mutare (b2-b3). Un alergător care intră în teritoriul inamic lângă un apărător, sau un intrus lângă piesa ta în teritoriul tău, e atins și stă 3 mutări în închisoare. Ajungi la steagul inamic și îl aduci în teritoriul tău ca să câștigi; purtătorul atins pierde steagul. 160 de mutări: remiză.

## Variante alese și limite

Închisoare cu timp fix (3 mutări), fără eliberare de către colegi.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
