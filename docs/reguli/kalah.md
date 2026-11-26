# Kalah (`kalah`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play kalah --seed 1`. Sursa implementării: `polytabletop-games/src/kalah.rs`.

> Rezumat din `polytabletop list`: Kalah 6×4: semănat antiorar (sari peste magazia adversarului); ultima sămânță în magazia ta = mutare în plus; într-o groapă proprie goală cu opusa nevidă capturezi opusa. Când o parte rămâne goală, restul merge în magazia celeilalte. Mutări: 1–6 (gropile tale).

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Este același motor ca `mancala` (Kalah 6×4), expus și sub numele Kalah.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Este același motor ca `mancala` (Kalah 6×4), expus și sub numele Kalah.
