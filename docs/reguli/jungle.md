# Jungle (Dou Shou Qi) (`jungle`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play jungle --seed 1`. Sursa implementării: `polytabletop-games/src/jungle.rs`.

> Rezumat din `polytabletop list`: Tablă 7×9, 8 animale pe jucător (șobolan 1 … elefant 8). Pas ortogonal; capturezi rang egal sau mai mic (șobolanul poate lua elefantul); doar șobolanul intră în apă; leul și tigrul sar lacul; în capcana adversă rangul e 0. Câștigi intrând în bârlogul adversarului sau capturând tot. Mutări: a1-a2, c3xc4.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Reguli standard Dou Shou Qi; fără remiză prin repetare.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Reguli standard Dou Shou Qi; fără remiză prin repetare.
