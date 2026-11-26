# Piști (`pisti`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pisti --seed 1`. Sursa implementării: `polytabletop-games/src/pisti.rs`.

> Rezumat din `polytabletop list`: 52 de cărți: 4 pe masă (3 cu fața în jos), 4 în mână. Cartea ta ia toată grămada dacă are același rang ca cea de deasupra sau e Valet. Piști (grămadă de o carte luată cu același rang) 10 p., Valet peste Valet 20 p. Puncte: Ași și Valeți 1, 2♣ 2, 10♦ 3, cele mai multe cărți 3. Se joacă până la 101. Mutări: 7h, Js, imparte.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Piști (Turcia, „Pişti”) — joc de capturat grămada de pe masă, 2 jucători.

Reguli folosite (varianta clasică pentru doi):
* 52 de cărți. Pe masă se pun 4 cărți: primele 3 **cu fața în jos** (nu le știe nimeni) și a
patra cu fața în sus (nu poate fi valet — altfel se reamestecă). Fiecare primește 4 cărți;
când mâinile se golesc se dau încă 4 (6 runde de câte 8 cărți). Începe cine nu e dealer.
* Joci o carte pe grămada de pe masă. Dacă are **același rang** ca a cărții de deasupra, sau
dacă e un **Valet** (J), iei toată grămada (masa goală: doar pui cartea). Valetul ia tot,
chiar și pe masă cu un singur valet.
* „Piști”: iei grămada când are o singură carte și o iei cu aceeași valoare (nu cu valet
peste altă carte) = 10 puncte; valet peste un singur valet = piști dublu, 20 de puncte.
* La sfârșit cărțile rămase pe masă merg la ultimul care a capturat.
* Puncte pe împărțire: fiecare As 1, fiecare Valet 1, 2♣ 2, 10♦ 3, cele mai multe cărți 3
(la egalitate nimeni), plus piști-urile. Se joacă împărțiri până când cineva ajunge la 101
(mai mare câștigă; la egalitate se mai joacă una).
* Mutări: cartea jucată (`7h`, `Js`); `imparte` când mâinile s-au golit.
* Informație ascunsă: vezi doar mâna ta și cartea de deasupra. AI-ul nu citește nici cărțile
rămase cu fața în jos, nici mâna adversarului; estimează probabilitatea ca adversarul să
poată captura (din cărțile nevăzute) și, la Hard/Expert, face eșantionare Monte-Carlo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
