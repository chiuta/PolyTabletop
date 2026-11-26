# Doppelkopf (2 jucători) (`doppelkopf`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play doppelkopf --seed 1`. Sursa implementării: `polytabletop-games/src/doppelkopf.rs`.

> Rezumat din `polytabletop list`: Adaptare pentru doi a jocului normal: 48 de cărți (9 10 J Q K A ×4 culori ×2), câte 24. Atuuri de la mare la mic: 10♥ (Dulle), Q♣ Q♠ Q♥ Q♦, J♣ J♠ J♥ J♦, A♦ 10♦ K♦ 9♦; culorile ♣ ♠ ♥ au ordinea A 10 K 9. Respecți culoarea sau atuul; din două cărți egale câștigă prima (a doua 10♥ o bate pe prima). Peste 120 din 240 de ochi câștigă mâna; primul la 2 mâini. Mutări: Qc, 10h, Ad, imparte (c=♣ s=♠ h=♥ d=♦).

## Reguli implementate

Adaptare pentru doi a jocului normal: 48 de cărți (9 10 J Q K A ×4 culori ×2), câte 24. Atuuri de la mare la mic: 10♥ (Dulle), Q♣ Q♠ Q♥ Q♦, J♣ J♠ J♥ J♦, A♦ 10♦ K♦ 9♦; culorile ♣ ♠ ♥ au ordinea A 10 K 9. Respecți culoarea sau atuul; din două cărți egale câștigă prima (a doua 10♥ o bate pe prima). Peste 120 din 240 de ochi câștigă mâna; primul la 2 mâini. Mutări: Qc, 10h, Ad, imparte (c=♣ s=♠ h=♥ d=♦).

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
