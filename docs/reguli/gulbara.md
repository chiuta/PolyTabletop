# Gul bara (Golful Persic) (`gulbara`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gulbara --seed 1`. Sursa implementării: `polytabletop-games/src/gulbara.rs`.

> Rezumat din `polytabletop list`: Variantă de table fără lovituri: 15 piese pe jucător, colțuri opuse, același sens de mers; un punct ocupat de adversar e blocat. Dublurile dau 4 mutări (apoi 4 cu fiecare număr până la 6, după primele 3 aruncări). Scoți piesele când toate sunt acasă; primul care termină câștigă. Mutări: 24/18, 6/off.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Nu se dau mutările pierdute adversarului; restul după regulile descrise.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Nu se dau mutările pierdute adversarului; restul după regulile descrise.
