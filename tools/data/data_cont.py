# Topuri pe continente (Africa, Asia, America de Nord, America de Sud, Oceania) + jocurile tradiționale
# regionale adăugate în catalog pentru ele (toate cu reguli publice; status `plan`, neimplementate).
# NEW = (Joc, Categorie, Origine). Clasamentele sunt estimări editoriale (vezi foaia „Note”).

NEW = [
    # Africa
    ("Oware", "Abstract", "Ghana / Africa de Vest"),
    ("Bao", "Abstract", "Tanzania / Africa de Est"),
    ("Seega", "Abstract", "Egipt"),
    ("Yote", "Abstract", "Senegal / Africa de Vest"),
    ("Hus", "Abstract", "Namibia"),
    ("Tâb", "Clasic", "Egipt / Orientul Mijlociu"),
    ("Senet", "Clasic", "Egiptul antic"),
    ("Whot!", "Cărți", "Nigeria"),
    ("Chkobba", "Cărți", "Tunisia"),
    ("Ronda", "Cărți", "Maroc / Spania"),
    # Asia
    ("Sholo Guti (Șaisprezece soldați)", "Abstract", "Bangladesh / India"),
    ("Aadu Puli Attam (Oi și tigri)", "Abstract", "India"),
    ("Mangala (mancala turcească)", "Abstract", "Turcia"),
    ("Dama (dame turcești)", "Abstract", "Turcia / Orientul Mijlociu"),
    ("Okey", "Clasic", "Turcia"),
    ("Riichi Mahjong", "Clasic", "Japonia"),
    ("Sic Bo", "Clasic", "China"),
    ("Ashta Chamma", "Clasic", "India"),
    ("Daifugo (Daihinmin)", "Cărți", "Japonia"),
    ("Tiến lên", "Cărți", "Vietnam"),
    ("Big Two (Pusoy dos)", "Cărți", "China / Filipine"),
    ("Tong-its", "Cărți", "Filipine"),
    ("Pasur", "Cărți", "Iran"),
    ("Hokm", "Cărți", "Iran"),
    ("Piști", "Cărți", "Turcia"),
    ("Court Piece (Rang, Mendikot)", "Cărți", "India"),
    ("Call Break", "Cărți", "Nepal / India"),
    # America de Nord
    ("Pinochle", "Cărți", "SUA"),
    ("Gin Rummy", "Cărți", "SUA"),
    ("Crazy Eights", "Cărți", "SUA"),
    ("Pitch (Setback)", "Cărți", "SUA"),
    ("Bid Whist", "Cărți", "SUA"),
    ("All Fours", "Cărți", "Trinidad și Tobago / Caraibe"),
    ("Conquián", "Cărți", "Mexic"),
    ("Chicken Foot (domino)", "Clasic", "SUA / Caraibe"),
    ("Bunco", "Clasic", "SUA"),
    ("Farkle (10 000)", "Clasic", "SUA"),
    ("Liar's Dice (Cacho, Dudo, Perudo)", "Clasic", "America Latină"),
    ("Crokinole", "Familie", "Canada"),
    # America de Sud
    ("Chinchón", "Cărți", "Spania / America Latină"),
    ("Burako (Buraco)", "Cărți", "Argentina / Uruguay / Brazilia"),
    ("Escoba (Escoba de 15)", "Cărți", "Argentina / Uruguay"),
    ("Cuarenta", "Cărți", "Ecuador"),
    ("Generala", "Clasic", "Argentina / Uruguay"),
    ("Parqués", "Familie", "Colombia"),
    # Oceania
    ("Five Hundred (500)", "Cărți", "SUA / Australia"),
    ("Pontoon", "Cărți", "Australia / Marea Britanie"),
    ("Two-up", "Clasic", "Australia"),
    ("Crown and Anchor", "Clasic", "Marea Britanie / Oceania"),
    ("Mu Torere", "Abstract", "Noua Zeelandă (maori)"),
]

# Fiecare linie: Joc | unde e cel mai popular (termeni din tools/site/data/origins.json, separați prin " / ")
# Numele trebuie să aibă aceeași cheie ca o intrare din catalog (fără paranteze, litere mici).
AFRICA = """
Șah | Pe tot continentul
Dame | Pe tot continentul
Domino | Africa de Vest / Africa Centrală
Oware | Ghana / Africa de Vest
Ludo | Nigeria / Africa de Vest
Mancala | Pe tot continentul
Scrabble | Nigeria / Kenya
Monopoly | Africa de Sud / Nigeria
Bao | Tanzania / Africa de Est
Table (Backgammon) | Egipt / Africa de Nord
Remi (Rummy) | Africa de Nord
Ayo (Ayoayo) | Nigeria
Whot! | Nigeria / Ghana
Șerpi și scări | Pe tot continentul
Poker | Africa de Sud / Africa de Est
Trix | Egipt / Sudan
Chkobba | Tunisia / Algeria
Ronda | Maroc
Belote | Africa de Nord / Africa de Vest
Uno | Pe tot continentul
Carrom | Africa de Est / Africa de Sud
Blackjack | Africa de Sud
Dara | Nigeria / Africa de Vest
Morabaraba | Africa de Sud
Fanorona | Madagascar
Seega | Egipt
Yote | Senegal / Africa de Vest
Tâb | Egipt / Africa de Nord
Hus | Namibia
Dama (dame turcești) | Africa de Est / Africa de Nord
Bingo | Africa de Sud
Whist | Africa de Sud / Africa de Est
Bridge | Egipt / Africa de Sud
Spades | Africa de Sud
Connect Four | Pe tot continentul
Battleship | Pe tot continentul
Cluedo (Clue) | Africa de Sud
Trivial Pursuit | Africa de Sud
Pictionary | Africa de Sud
Jenga | Africa de Sud
Rummikub | Africa de Sud
Yahtzee | Africa de Sud
Taboo | Africa de Sud
Cards Against Humanity | Africa de Sud
Tarneeb | Egipt / Africa de Nord
Baloot | Africa de Nord
Teen Patti | Africa de Est / Africa de Sud
Hearts | Africa de Sud
Canasta | Africa de Sud
Catan | Africa de Sud
Carcassonne | Africa de Sud
Ticket to Ride | Africa de Sud
Game of Life | Africa de Sud
Risk | Africa de Sud
Activity | Africa de Sud
Mastermind | Africa de Sud
Memory | Pe tot continentul
Guess Who? (Cine e?) | Africa de Sud
Candy Land | Africa de Sud
Dungeons & Dragons | Africa de Sud
Pokémon TCG | Africa de Sud
Magic: The Gathering | Africa de Sud
Yu-Gi-Oh! TCG | Africa de Sud
Warhammer 40,000 | Africa de Sud
Pathfinder | Africa de Sud
Call of Cthulhu | Africa de Sud
Mahjong | Africa de Sud
Go (weiqi, baduk) | Africa de Sud
Xiangqi (șah chinezesc) | Africa de Sud
Werewolf / Mafia | Africa de Sud / Nigeria
Codenames | Africa de Sud
Pandemic | Africa de Sud
Dixit | Africa de Sud
Azul | Africa de Sud
Splendor | Africa de Sud
Wingspan | Africa de Sud
7 Wonders | Africa de Sud
Exploding Kittens | Africa de Sud
Love Letter | Africa de Sud
Sushi Go! | Africa de Sud
Kingdomino | Africa de Sud
Bang! | Africa de Sud
Munchkin | Africa de Sud
Dobble (Spot It!) | Africa de Sud
Scotland Yard | Africa de Sud
Labirintul nebun (Das verrückte Labyrinth) | Africa de Sud
Hex | Pe tot continentul
Gomoku | Pe tot continentul
Reversi / Othello | Pe tot continentul
Moara (Nine Men's Morris) | Pe tot continentul
Nim | Pe tot continentul
Senet | Egipt
Stratego | Africa de Sud
Small World | Africa de Sud
Skull | Africa de Sud
Lost Cities | Africa de Sud
Hanabi | Africa de Sud
Just One | Africa de Sud
Terraforming Mars | Africa de Sud
Gloomhaven | Africa de Sud
""".strip().split("\n")

ASIA = """
Șah | Pe tot continentul
Mahjong | China / Asia de Est
Go (weiqi, baduk) | China / Japonia / Coreea
Xiangqi (șah chinezesc) | China / Vietnam
Ludo | India / Asia de Sud
Carrom | India / Pakistan
Șerpi și scări | India
Shogi | Japonia
Table (Backgammon) | Turcia / Iran / Orientul Mijlociu
Domino | Asia de Sud-Est / Orientul Mijlociu
Rummy (13 cărți) | India / Pakistan
Teen Patti | India / Asia de Sud
Poker | Asia de Est / Asia de Sud-Est
Dou dizhu | China
Pokémon TCG | Japonia / Asia de Est
Yu-Gi-Oh! TCG | Japonia
Monopoly | Asia de Est / Asia de Sud-Est
Uno | Pe tot continentul
Scrabble | India / Singapore / Malaezia
Dame | Asia Centrală / Turcia
Baloot | Arabia Saudită
Tarneeb | Orientul Mijlociu
Trix | Orientul Mijlociu
Hanafuda | Japonia
Karuta | Japonia
Sugoroku | Japonia
Hwatu (Go-Stop) | Coreea
Janggi | Coreea
Yut (Yunnori) | Coreea
Makruk | Thailanda
Sungka | Filipine
Congkak | Malaezia / Indonezia
Pallanguzhi | India
Ganjifa | India
Chaupar | India
Ludo / Parcheesi | India
Pai gow | China
Gomoku | Japonia / China / Coreea
Reversi / Othello | Japonia
Hasami Shogi | Japonia
Bagh-Chal | Nepal
Surakarta | Indonezia
Werewolf (langren sha) | China
Splendor | Asia de Est
Codenames | Asia de Est
Machi Koro | Japonia
Love Letter | Japonia
Azul | Asia de Est
Sword World RPG | Japonia
InSANe | Japonia
Call of Cthulhu | Japonia
Dungeons & Dragons | Asia de Est
Magic: The Gathering | Japonia / Asia de Est
Catan | Asia de Est
Ticket to Ride | Asia de Est
Sholo Guti (Șaisprezece soldați) | Bangladesh / India
Aadu Puli Attam (Oi și tigri) | India
Mangala (mancala turcească) | Turcia
Dama (dame turcești) | Turcia / Orientul Mijlociu
Okey | Turcia
Riichi Mahjong | Japonia
Sic Bo | China
Ashta Chamma | India
Daifugo (Daihinmin) | Japonia
Tiến lên | Vietnam
Big Two (Pusoy dos) | China / Filipine
Tong-its | Filipine
Pasur | Iran
Hokm | Iran
Piști | Turcia
Court Piece (Rang, Mendikot) | India
Call Break | Nepal / India
Blackjack | Asia de Est
Bingo | Filipine / Asia de Sud-Est
Pandemic | Asia de Est
Cluedo (Clue) | Asia de Est
Pictionary | Asia de Est
Trivial Pursuit | Asia de Est
Jenga | Asia de Est
Connect Four | Pe tot continentul
Battleship | Pe tot continentul
Mancala | Asia de Sud-Est
Fanorona | Asia de Sud
Durak | Asia Centrală
Bang! | Asia de Est
Exploding Kittens | Asia de Est
Dixit | Asia de Est
Kingdomino | Asia de Est
Wingspan | Asia de Est
7 Wonders | Asia de Est
Sushi Go! | Asia de Est
Dobble (Spot It!) | Asia de Est
Warhammer 40,000 | Asia de Est
Star Wars: X-Wing | Asia de Est
Disney Lorcana | Asia de Est
Flesh and Blood | Asia de Est
Rummikub | Israel
Mastermind | Israel
Stratego | Asia de Est
Skull | Asia de Est
""".strip().split("\n")

AMNORD = """
Monopoly | SUA / Canada
Scrabble | SUA / Canada
Poker | SUA
Uno | SUA / Canada / Mexic
Dungeons & Dragons | SUA / Canada
Magic: The Gathering | SUA
Pokémon TCG | SUA / Canada
Șah | Pe tot continentul
Domino | Mexic / Caraibe
Bingo | SUA / Canada
Catan | SUA / Canada
Yahtzee | SUA
Game of Life | SUA
Risk | SUA
Ticket to Ride | SUA
Cluedo (Clue) | SUA
Trivial Pursuit | Canada / SUA
Battleship | SUA
Connect Four | SUA
Dame | SUA / Canada
Cribbage | SUA / Canada
Euchre | SUA / Canada
Spades | SUA
Hearts | SUA
Bridge | SUA / Canada
Blackjack | SUA
Gin Rummy | SUA
Remi (Rummy) | SUA
Pinochle | SUA
Cards Against Humanity | SUA
Pictionary | SUA
Taboo | SUA
Jenga | SUA
Pandemic | SUA
Codenames | SUA
Exploding Kittens | SUA
Lotería | Mexic
Conquián | Mexic
Table (Backgammon) | SUA
Crazy Eights | SUA
Pitch (Setback) | SUA
Bid Whist | SUA
Bunco | SUA
Farkle (10 000) | SUA
Liar's Dice (Cacho, Dudo, Perudo) | SUA / Mexic
Crokinole | Canada
All Fours | Trinidad și Tobago / Caraibe
Chicken Foot (domino) | SUA / Caraibe
Oware | Caraibe
Ludo | Caraibe
Șerpi și scări | Caraibe
Pathfinder | SUA
Vampire: The Masquerade | SUA
Call of Cthulhu | SUA
Cyberpunk RED | SUA
Shadowrun | SUA
Blades in the Dark | SUA
Starfinder | SUA
Warhammer 40,000 | SUA / Canada
Warhammer: Age of Sigmar | SUA / Canada
Star Wars: X-Wing | SUA
Yu-Gi-Oh! TCG | SUA
Disney Lorcana | SUA
Flesh and Blood | SUA
Gloomhaven | SUA
Root | SUA
Scythe | SUA
Terraforming Mars | SUA
Wingspan | SUA
Everdell | SUA
Cascadia | SUA / Canada
Spirit Island | SUA
Dominion | SUA
Azul | SUA
Carcassonne | SUA
Splendor | SUA
Dune: Imperium | SUA
Twilight Imperium | SUA
Blood on the Clocktower | SUA
Secret Hitler | SUA
The Resistance: Avalon | SUA
Munchkin | SUA
Bang! | SUA
Candy Land | SUA
Guess Who? (Cine e?) | SUA
Twister | SUA
Mastermind | SUA
Scotland Yard | SUA
Rummikub | SUA
Dobble (Spot It!) | SUA
Sushi Go! | SUA
Love Letter | SUA
Mahjong | SUA
Go (weiqi, baduk) | SUA
Hex | SUA
Oh Hell | SUA
Whist | SUA
Canasta | SUA
Werewolf / Mafia | SUA
Dixit | SUA
""".strip().split("\n")

AMSUD = """
Truco | Argentina / Uruguay / Brazilia
Șah | Pe tot continentul
Domino | Venezuela / Colombia
Dame | Brazilia / Argentina
Uno | Pe tot continentul
Monopoly | Brazilia / Argentina
Ludo | Brazilia
Parqués | Colombia
Poker | Brazilia / Argentina
Burako (Buraco) | Argentina / Uruguay / Brazilia
Chinchón | Argentina / Uruguay
Escoba (Escoba de 15) | Argentina / Uruguay
Generala | Argentina / Uruguay
Cuarenta | Ecuador
Liar's Dice (Cacho, Dudo, Perudo) | Bolivia / Peru / Chile
Canasta | Uruguay / Argentina
Bingo | Brazilia / Argentina
Battleship | Brazilia / Argentina
Connect Four | Brazilia
Risk | Brazilia
Cluedo (Clue) | Brazilia
Game of Life | Brazilia
Guess Who? (Cine e?) | Brazilia
Șerpi și scări | Pe tot continentul
Table (Backgammon) | Brazilia / Argentina
Scrabble | Brazilia / Argentina
Blackjack | Argentina / Brazilia
Remi (Rummy) | Argentina / Chile
Tormenta | Brazilia
Dungeons & Dragons | Brazilia
Magic: The Gathering | Brazilia
Pokémon TCG | Brazilia
Yu-Gi-Oh! TCG | Brazilia
Hanafuda | Brazilia
Shogi | Brazilia
Mahjong | Brazilia
Go (weiqi, baduk) | Brazilia
Pictionary | Brazilia / Argentina
Taboo | Brazilia
Jenga | Brazilia
Mastermind | Brazilia
Trivial Pursuit | Brazilia / Argentina
Cards Against Humanity | Brazilia
Werewolf / Mafia | Brazilia
Catan | Brazilia / Argentina
Carcassonne | Brazilia / Argentina
Ticket to Ride | Brazilia
Codenames | Brazilia
Dixit | Brazilia / Argentina
Exploding Kittens | Brazilia
Bang! | Brazilia
Munchkin | Brazilia
Dobble (Spot It!) | Brazilia / Argentina
Splendor | Brazilia
Azul | Brazilia
Wingspan | Brazilia
7 Wonders | Brazilia
Kingdomino | Brazilia
Love Letter | Brazilia
Sushi Go! | Brazilia
Spades | Brazilia
Hearts | Brazilia
Whist | Chile / Argentina
Bridge | Brazilia / Argentina
Briscola | Argentina / Uruguay
Scopa | Argentina / Uruguay
Rummikub | Brazilia / Argentina
Yahtzee | Brazilia / Argentina
Activity | Brazilia
Candy Land | Brazilia
Memory | Pe tot continentul
Scotland Yard | Brazilia
Labirintul nebun (Das verrückte Labyrinth) | Brazilia
Call of Cthulhu | Brazilia
Pathfinder | Brazilia
Vampire: The Masquerade | Brazilia
Warhammer 40,000 | Brazilia / Argentina / Chile
Warhammer: Age of Sigmar | Brazilia / Chile
Star Wars: X-Wing | Brazilia
Blood Bowl | Brazilia
Disney Lorcana | Brazilia
Flesh and Blood | Brazilia
Gloomhaven | Brazilia
Terraforming Mars | Brazilia
Root | Brazilia
Scythe | Brazilia
Nim | Pe tot continentul
Gomoku | Pe tot continentul
Reversi / Othello | Pe tot continentul
Hex | Pe tot continentul
Moara (Nine Men's Morris) | Pe tot continentul
Lotería | Colombia / Peru
Xiangqi (șah chinezesc) | Brazilia
Stratego | Brazilia
Hanabi | Brazilia
Small World | Brazilia
Zombicide | Brazilia
Skull | Brazilia
Lost Cities | Brazilia
Bohnanza | Brazilia
""".strip().split("\n")

OCEANIA = """
Monopoly | Australia / Noua Zeelandă
Șah | Pe tot continentul
Scrabble | Australia / Noua Zeelandă
Poker | Australia
Two-up | Australia
Euchre | Australia
Five Hundred (500) | Australia
Cribbage | Noua Zeelandă / Australia
Bridge | Australia / Noua Zeelandă
Bingo | Australia / Noua Zeelandă
Dame | Pe tot continentul
Uno | Pe tot continentul
Domino | Insulele Pacificului
Cluedo (Clue) | Australia / Noua Zeelandă
Trivial Pursuit | Australia / Noua Zeelandă
Yahtzee | Australia
Catan | Australia / Noua Zeelandă
Carcassonne | Australia / Noua Zeelandă
Ticket to Ride | Australia / Noua Zeelandă
Sushi Go! | Australia
Wingspan | Australia
Warhammer 40,000 | Australia / Noua Zeelandă
Warhammer: Age of Sigmar | Australia / Noua Zeelandă
Flesh and Blood | Australia
Pokémon TCG | Australia / Noua Zeelandă
Magic: The Gathering | Australia / Noua Zeelandă
Dungeons & Dragons | Australia / Noua Zeelandă
Pathfinder | Australia
Call of Cthulhu | Australia
Mu Torere | Noua Zeelandă
Konane | Hawaii
Hanafuda | Hawaii
Crown and Anchor | Australia / Noua Zeelandă
Pontoon | Australia / Noua Zeelandă
Blackjack | Australia
Table (Backgammon) | Australia
Remi (Rummy) | Australia / Noua Zeelandă
Whist | Australia / Noua Zeelandă
Hearts | Australia
Spades | Australia
Oh Hell | Australia / Noua Zeelandă
Canasta | Australia / Noua Zeelandă
Rummikub | Australia / Noua Zeelandă
Mahjong | Australia / Noua Zeelandă
Go (weiqi, baduk) | Australia
Connect Four | Pe tot continentul
Battleship | Australia / Noua Zeelandă
Șerpi și scări | Pe tot continentul
Ludo | Fiji / Insulele Pacificului
Pictionary | Australia
Taboo | Australia
Jenga | Australia
Cards Against Humanity | Australia
Activity | Australia
Codenames | Australia
Pandemic | Australia
Dixit | Australia
Azul | Australia
Splendor | Australia
7 Wonders | Australia
Kingdomino | Australia
Cascadia | Australia
Everdell | Australia
Terraforming Mars | Australia
Root | Australia
Scythe | Australia
Gloomhaven | Australia
Spirit Island | Australia
Exploding Kittens | Australia
Love Letter | Australia
Bang! | Australia
Munchkin | Australia
Dobble (Spot It!) | Australia
Werewolf / Mafia | Australia
Blood on the Clocktower | Australia
The Resistance: Avalon | Australia
Secret Hitler | Australia
Game of Life | Australia / Noua Zeelandă
Risk | Australia / Noua Zeelandă
Candy Land | Australia
Guess Who? (Cine e?) | Australia
Mastermind | Australia
Scotland Yard | Australia
Memory | Australia
Twister | Australia
Star Wars: X-Wing | Australia
Disney Lorcana | Australia
Yu-Gi-Oh! TCG | Australia
Blood Bowl | Australia
Vampire: The Masquerade | Australia
Hex | Pe tot continentul
Gomoku | Pe tot continentul
Reversi / Othello | Pe tot continentul
Nim | Pe tot continentul
Skull | Australia
Lost Cities | Australia
Hanabi | Australia
Just One | Australia
Small World | Australia
Stratego | Australia
""".strip().split("\n")

LISTS = [("Africa", "africa", AFRICA), ("Asia", "asia", ASIA), ("America de Nord", "namerica", AMNORD),
         ("America de Sud", "samerica", AMSUD), ("Oceania", "oceania", OCEANIA)]
