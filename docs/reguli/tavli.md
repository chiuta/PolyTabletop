# Tavli (Portes, Plakoto, Fevga) (`tavli`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tavli --seed 1`. Sursa implementării: `polytabletop-games/src/tavli.rs`.

> Rezumat din `polytabletop list`: Tabla greacă: J1 alege varianta (portes, plakoto sau fevga). Portes = backgammon cu lovire, Plakoto = imobilizezi piesa adversă singură, Fevga = ambii în același sens, fără lovire și cu regula zidului. Zaruri automate; 15 piese; scoți piesele când toate sunt acasă. Mutări: 24/18, bar/20, 6/off.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Fevga: fără regula primei piese și cu zid simplificat; Plakoto și Portes după regulile standard.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Fevga: fără regula primei piese și cu zid simplificat; Plakoto și Portes după regulile standard.
