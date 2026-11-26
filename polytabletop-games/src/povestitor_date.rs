//! Cărțile „Povestitorului”: descrieri-simbol originale.
//! Format: (descrierea-simbol, etichete, indicii separate prin „;”).
//! Etichetele sunt cuvinte; potrivirea indiciu–carte se face pe rădăcini de 5 litere.

pub const CARDS: &[(&str, &str, &str)] = &[
    (
        "Un far aprins pe o stâncă, într-o mare de hârtii",
        "far lumină hârtie veghe singurătate departe cineva",
        "Cineva veghează departe;Corabia de scrisori",
    ),
    (
        "O scară de sfoară care urcă într-un nor",
        "scară sfoară nor urcuș cer treaptă treaptă",
        "Treaptă după treaptă;Drum spre vată",
    ),
    (
        "Un pește de sticlă înoată printr-o bibliotecă",
        "pește sticlă carte înot tăcere adâncuri cititor",
        "Cititor de adâncuri;Acvariul ideilor",
    ),
    (
        "Doi pantofi care dansează singuri sub lună",
        "pantofi dans lună noapte singuri stăpân pași",
        "Pași fără stăpân;Balul de la miezul nopții",
    ),
    (
        "O casă cu acoperiș de frunze și horn de ceainic",
        "casă frunze ceainic fum toamnă bunica",
        "Bunica a pus apa;Adăpost de toamnă",
    ),
    (
        "O cheie uriașă vârâtă în cer",
        "cheie cer lacăt uriaș deschidere încuiat norii",
        "Cine a încuiat norii;Descuie dimineața",
    ),
    (
        "Un pian plutind pe un râu de lapte",
        "pian râu lapte plutire muzică cântec curent",
        "Cântec pe curent;Notele curg",
    ),
    (
        "Un urs care citește povești unui bec",
        "urs povești bec lectură seară lumină",
        "Lectură pentru lumină;Somn de iarnă cu poveste",
    ),
    (
        "O barcă de hârtie cu pânze de stele",
        "barcă hârtie stele pânze copilărie călătorie buzunar",
        "Călătorie de buzunar;Navigator mic",
    ),
    (
        "Un copac cu ceasuri în loc de fructe",
        "copac ceasuri fructe timp ramuri coace",
        "Se coace ora;Recoltă de secunde",
    ),
    (
        "O pasăre cu aripi din file de calendar",
        "pasăre aripi calendar zbor zile zboară",
        "Zboară zilele;Anul cu pene",
    ),
    (
        "Un om care cară luna într-o roabă",
        "luna roabă noapte muncă povară nopții",
        "Povara nopții;Transport de lumină",
    ),
    (
        "O ușă izolată în mijlocul unui câmp",
        "ușă câmp izolată portal drum intri ieși",
        "Intri sau ieși?;Poarta fără casă",
    ),
    (
        "Un elefant pe un fir de ață",
        "elefant ață echilibru fir greutate uriaș",
        "Echilibrist uriaș;Grijă la cusătură",
    ),
    (
        "Un orologiu topit pe marginea unei mese",
        "orologiu topit masă timp topire înmuiat",
        "Ora s-a înmuiat;Timpul nu mai ține",
    ),
    (
        "Un balon cu o casă agățată dedesubt",
        "balon casă zbor gospodărie aer mutarea înalt",
        "Mutarea în înalt;Acasă pe vânt",
    ),
    (
        "O pădure de umbrele deschise sub ploaie",
        "pădure umbrele ploaie picături adăpost colorată",
        "Ploaie colorată;Copaci artificiali",
    ),
    (
        "Un tren care trece printr-o carte deschisă",
        "tren carte pagină drum poveste rândurilor șinele",
        "Pe șinele rândurilor;Capitolul următor, peron trei",
    ),
    (
        "O pisică ce își toarce noaptea în culori",
        "pisică toarce noaptea culori somn mângâiere",
        "Mângâiere de culoare;Mustăți de curcubeu",
    ),
    (
        "O inimă ținută în colivie",
        "inimă colivie captiv dor libertate eliberează",
        "Cine o eliberează;Bate în gratii",
    ),
    (
        "Un cal alb care galopează pe suprafața unui lac",
        "cal lac galop alb apă oglindă fugă",
        "Fugă pe oglindă;Copitele nu se udă",
    ),
    (
        "O mână care picură aur într-o ceașcă",
        "mână aur ceașcă picură ceai zahărul prețios",
        "Zahărul prețios;Ceasul de ceai regal",
    ),
    (
        "Un cerb cu coarne pline de lumânări",
        "cerb coarne lumânări flăcări pădure mergătoare biserica",
        "Biserica mergătoare;Candelabru viu",
    ),
    (
        "O luntre pe valuri de cărți vechi",
        "luntre valuri cărți vechi mare naufragiu literar",
        "Naufragiu literar;Marea cititorilor",
    ),
    (
        "Un cățel care latră la un nor",
        "cățel latră nor cer curaj",
        "Mic și curajos;Trece fără să-i pese",
    ),
    (
        "Doi copii care țin o punte de curcubeu",
        "copii punte curcubeu legătură culori maluri între",
        "Legătură între două maluri;Joacă cu cerul",
    ),
    (
        "O poartă de ceață la capătul grădinii",
        "poartă ceață grădină mister capăt dincolo vede",
        "Dincolo nu se vede;Intrarea în ceva alb",
    ),
    (
        "Un cocoș pe un acoperiș de timp",
        "cocoș acoperiș zori timp cântec trezește soarele",
        "Cine trezește soarele;Strigăt de dimineață",
    ),
    (
        "O harpă înfășurată de iederă",
        "harpă iederă strune muzică uitare acoperită",
        "Muzică acoperită;Pădurea cântă singură",
    ),
    (
        "Un pod făcut din degete întinse",
        "pod degete întinse atingere trecere ajutorul sosește",
        "Ajutorul sosește;Un lanț de mâini",
    ),
    (
        "O cană din care se ridică un oraș",
        "cană oraș abur ceai lume metropolă",
        "Metropolă de abur;Mica lume fierbinte",
    ),
    (
        "Un elev cu ghiozdan plin de stele",
        "elev ghiozdan stele școală cer cosmică lecția",
        "Lecția cosmică;Temă pentru Calea Lactee",
    ),
    (
        "O broască pe un nufăr care citește harta",
        "broască nufăr hartă lac drum orientare baltă",
        "Orientare în baltă;Călătorul verde",
    ),
    (
        "Un clopot înecat într-un lac de lună",
        "clopot lac lună sunet adânc sună",
        "Sună sub apă;Liniște cu ecou",
    ),
    (
        "O păpușă care coase un nor rupt",
        "păpușă coase nor ac ploaie reparații",
        "Reparații la cer;Croitoreasa celestă",
    ),
    (
        "Un labirint pe palma unei mâini",
        "labirint palmă mână drum linii desenată soarta",
        "Soarta e desenată;Linii de viață",
    ),
    (
        "O lumânare topită în forma unei cetăți",
        "lumânare cetate topită ceară flacără",
        "Zid de ceară;Palat de lumină",
    ),
    (
        "Un vânt care poartă o scrisoare roșie",
        "vânt scrisoare roșie mesaj zbor departe veste",
        "Veste de departe;Poștaș nevăzut",
    ),
    (
        "O vulpe cu coada ca o flacără",
        "vulpe coada flacără pădure vicleană arzătoare șireată",
        "Șireată și arzătoare;Foc cu patru picioare",
    ),
    (
        "Un om-ceas cu cadranul în loc de față",
        "ceas față cadran timp om chip chip",
        "Chip fără chip;Cum arată ora",
    ),
    (
        "O cetate de nisip lângă mare",
        "cetate nisip mare val trecere valul până",
        "Până vine valul;Castel efemer",
    ),
    (
        "O barză care aduce un coș cu cuvinte",
        "barză coș cuvinte cuib veste specială livrare",
        "Livrare specială;Veste ce vine pe cer",
    ),
    (
        "Un arbore al cărui trunchi e o scară în spirală",
        "arbore trunchi scară spirală urcare cerc",
        "Urcă în cerc;Casa copacului uriaș",
    ),
    (
        "O oglindă care reflectă alt anotimp",
        "oglindă anotimp reflectă iarnă schimbare vara uită",
        "Vara se uită în iarnă;Fereastră spre altceva",
    ),
    (
        "Un pescar care prinde stele cu undița",
        "pescar stele undița noapte răbdare nocturnă",
        "Răbdare nocturnă;Momeală de lumină",
    ),
    (
        "O morișcă de vânt pe un acoperiș de nori",
        "morișcă vânt nori acoperiș aer râșnește cerul",
        "Râșnește cerul;Se învârte sus",
    ),
    (
        "Un cor de păsări pe fire de telefon ca pe portativ",
        "păsări fire portativ cor melodie singure notele",
        "Notele s-au așezat singure;Concert pe stâlpi",
    ),
    (
        "O inimă de lemn cu rădăcini",
        "inimă lemn rădăcini tulpină dor prins",
        "A prins rădăcini;Sentiment de stejar",
    ),
    (
        "Un geamantan care se deschide într-o pădure",
        "geamantan pădure plecare deschis aventură bagaj",
        "Bagaj de aventuri;Ce ai uitat acasă",
    ),
    (
        "Un soare cu ochelari de soare",
        "soare ochelari strălucire vară cald pauză",
        "Își ia o pauză;Prea multă lumină",
    ),
    (
        "O broască țestoasă cu o casă în spate și o grădină pe casă",
        "țestoasă casă grădină spate încet mergător ținutul",
        "Ținutul mergător;Rădăcini în mers",
    ),
    (
        "O stradă care se termină în cer",
        "stradă cer capăt drum asfalt continuă",
        "Drumul continuă sus;Unde duc trotuarele",
    ),
    (
        "Un cuib în care stau trei lămpi aprinse",
        "cuib lămpi aprinse pui căldură lumină",
        "Pui de lumină;Căldura de acasă",
    ),
    (
        "O șoaptă desenată ca un șarpe de aer",
        "șoaptă șarpe aer secret vorbă strecoară printre",
        "Se strecoară printre urechi;Vorbă pe furiș",
    ),
    (
        "O sanie trasă de un dragon mic",
        "sanie dragon iarnă zăpadă foc gheață",
        "Foc pe gheață;Cai de basm",
    ),
    (
        "Un cerc de copii în jurul unui foc de artificii",
        "copii cerc foc artificii bucurie sărbătoare seară",
        "Seară de sărbătoare;Ochii strălucesc",
    ),
    (
        "Un telefon vechi pe care sună o floare",
        "telefon floare sună vechi apel primăvară sunet",
        "Sunet de primăvară;Cineva te caută din grădină",
    ),
    (
        "O cale ferată ce se pierde în pustiu",
        "cale ferată pustiu orizont tren departe mereu",
        "Mai departe, mereu;Până dispare linia",
    ),
    (
        "Un balet de frunze în jurul unui stâlp",
        "balet frunze stâlp vânt toamnă dans",
        "Dans de toamnă;Vârtej fără muzică",
    ),
    (
        "O fată care își pieptănă părul cu o rază",
        "fată pieptene rază păr lumină soare",
        "Pieptene de soare;Toaletă de dimineață",
    ),
    (
        "O scrisoare care crește ca o floare",
        "scrisoare floare crește plic vorbă înflorește mesajul",
        "Mesajul înflorește;Flori de hârtie",
    ),
    (
        "Un tunel de mătase prin care trece un tren de jucărie",
        "tunel mătase tren jucărie copil cutezanță joacă",
        "Joacă prin cutezanță;Lume în miniatură",
    ),
    (
        "Un om adormit pe o carte uriașă",
        "adormit carte uriașă somn vis citește visând",
        "Citește visând;Perna de pagini",
    ),
    (
        "O pereche de aripi agățată în cuier",
        "aripi cuier haină zbor uitat",
        "Hainele de zbor;Ce lași la ușă",
    ),
    (
        "Un steag alb pe vârful unui munte de vată",
        "steag alb munte vată pace predat",
        "Cine s-a predat?;Vârf liniștit",
    ),
    (
        "Un râu care curge din robinetul unei case",
        "râu robinet casă apă curge bucătărie sursa",
        "Sursa de la bucătărie;Fântâna din perete",
    ),
    (
        "O hartă desenată pe o frunză uscată",
        "hartă frunză uscată drum toamnă comoara copaci",
        "Comoara de sub copaci;Cartografie fragilă",
    ),
    (
        "O pasăre albastră în cușca unui ceas",
        "pasăre albastră ceas cușcă oră cucu",
        "Cucu!;Cântă la fix",
    ),
    (
        "Un cavaler de hârtie cu sabie din creion",
        "cavaler hârtie sabie creion curaj războinic caiet",
        "Războinic de caiet;Luptă pe pagină",
    ),
    (
        "Un nor care udă o grădină de ghivece",
        "nor grădină ghivece ploaie udă nevoie cade",
        "Cade când e nevoie;Stropire cerească",
    ),
    (
        "O fântână în care se văd stele ziua",
        "fântână stele ziua adânc apă cerul",
        "Cerul din adânc;Oglinda din pământ",
    ),
    (
        "Un violonist pe un acoperiș plin de zăpadă",
        "violonist acoperiș zăpadă cântec iarnă muzică gheață",
        "Muzică de gheață;Echilibru și arcuș",
    ),
    (
        "O poartă mare de lemn cu un lacăt cât o casă",
        "poartă lemn lacăt mare interzis nimeni intră",
        "Nimeni nu intră;Seif uriaș",
    ),
    (
        "O fluture uriașă pe turnul unui castel",
        "fluture turn castel aripi uriaș aterizat acolo",
        "Cine a aterizat acolo;Palat cu antene",
    ),
    (
        "Un arici purtând pe spini toate merele toamnei",
        "arici spini mere toamnă bogăție comerciant ghimpos",
        "Comerciant ghimpos;Cărăuș de toamnă",
    ),
    (
        "O banană-barcă pe un râu de ciocolată",
        "banană barcă râu ciocolată dulce navigare gustoasă",
        "Navigare gustoasă;Poftă de călătorie",
    ),
    (
        "O carte cu ferestre în loc de pagini",
        "carte ferestre pagini privire lume citești",
        "Citești privind;Vedere la poveste",
    ),
    (
        "Un gândac în costum, cu pălărie",
        "gândac costum pălărie domn eleganță distins mărunt",
        "Distins și mărunt;Musafir de seamă",
    ),
    (
        "O lună plină servită pe o farfurie",
        "lună plină farfurie cină noapte cerului",
        "Cina cerului;Mâncare de vis",
    ),
    (
        "Un pian cu clapele din scară",
        "pian clape scară urcuș muzică ascendentă gamă",
        "Gamă ascendentă;Treptele cântă",
    ),
    (
        "O ceașcă de ceai pe un vulcan",
        "ceașcă ceai vulcan fierbinte lavă pauza risc",
        "Pauza de risc;Aburul care trădează",
    ),
    (
        "Un balon de săpun care conține o grădină",
        "balon săpun grădină fragil lume",
        "O lume foarte fragilă;Primăvara care poate pocni",
    ),
    (
        "Un copil care trage un nor cu sfoara",
        "copil nor sfoară joacă cer plimbare norul",
        "Plimbare cu norul;Zmeu fără vânt",
    ),
    (
        "Un om care aprinde lampa cu o scânteie de pe stea",
        "om lampă scânteie stea aprinde lumină furată",
        "Lumină furată;Foc de sus",
    ),
    (
        "O barcă cu două vâsle uriașe pe un lac de gheață",
        "barcă vâsle lac gheață iarnă piatră",
        "Cum vâslești pe piatră;Popas înghețat",
    ),
    (
        "Un bufon care jonglează cu planete",
        "bufon jonglează planete circ cosmos spectacol galactic",
        "Spectacol galactic;Echilibru cosmic",
    ),
    (
        "O cărare de pietre care se termină cu o ușă în pământ",
        "cărare pietre ușă pământ secret intrarea adânc",
        "Intrarea în adânc;Cine locuiește sub dealuri",
    ),
    (
        "Un nod gordian pe un fir de păr",
        "nod fir păr încurcat dificil desface greu",
        "Ce greu se desface;Încurcătură fină",
    ),
    (
        "O vioară care plânge cu strune de ploaie",
        "vioară plânge ploaie strune tristețe melodie",
        "Melodie udă;Doru-n picături",
    ),
    (
        "O gheară de dragon care ține un trandafir",
        "dragon gheară trandafir blândețe forță gingășie",
        "Forță și gingășie;Frumoasa și fiara",
    ),
    (
        "Un cuc într-un ceas de nisip",
        "cuc ceas nisip timp strigăt vremea trece",
        "Trece vremea;Păsărica din clepsidră",
    ),
    (
        "O frunză-corabie cu un șoarece căpitan",
        "frunză corabie șoarece căpitan apă navigator",
        "Mic navigator;Echipaj de grădină",
    ),
    (
        "O piramidă de cutii pe un cal",
        "piramidă cutii cal echilibru povară atenție cădere",
        "Atenție la cădere;Povara povețelor",
    ),
    (
        "O lanternă care ține piept întunericului",
        "lanternă întuneric piept lumină curaj fascicul singur",
        "Un singur fascicul;Veghe de buzunar",
    ),
    (
        "O furnică cu o frunză cât un cort",
        "furnică frunză cort muncă mică harnic",
        "Mic și harnic;Umbrela de sub talpă",
    ),
    (
        "O gară pe o insulă fără tren",
        "gară insulă tren așteptare mare",
        "Cine mai așteaptă?;Peron de nisip",
    ),
    (
        "Un tablou care iese din ramă",
        "tablou ramă iese pictură evadare pânză",
        "Evadare din pânză;Personajul fuge",
    ),
    (
        "O stea căzută în buzunarul unui copil",
        "stea căzută buzunar copil pretioasă comoara noapte",
        "Comoara de noapte;O dorință la mână",
    ),
    (
        "Un elefant care bea dintr-o ceașcă mică",
        "elefant ceașcă mică politețe ceai elegant enorm",
        "Elegant și enorm;Sorb cu delicatețe",
    ),
    (
        "Un cântec care iese dintr-un coș de viță",
        "cântec coș viță toamnă recoltă strugurii",
        "Strugurii cântă;Ghirlandă sonoră",
    ),
    (
        "O arcă plină cu păsări de hârtie",
        "arcă păsări hârtie potop salvare bucată",
        "Salvare de bucată;Zbor de caiet",
    ),
    (
        "Un deget care atinge luna",
        "deget luna atingere cer dor aproape atât",
        "Atât de aproape;Un pic mai sus",
    ),
    (
        "O pană scrie singură un jurnal",
        "pană scrie singură jurnal povești cerneala vorbește",
        "Cerneala vorbește;Amintire fără autor",
    ),
    (
        "Un cerc de pietre cu un foc în mijloc",
        "cerc pietre foc mijloc sfat seară",
        "Sfat de seară;Tăcere în jurul flăcării",
    ),
    (
        "O gospodină care aruncă ziua în aer ca pe o plăcintă",
        "gospodină ziua plăcintă aer coace frământă timpul",
        "Frământă timpul;Totul se învârte",
    ),
    (
        "O ancoră agățată de un balon",
        "ancoră balon greutate ușor contradicție câștigă",
        "Cine câștigă, jos sau sus?;Povară ușoară",
    ),
    (
        "Un lac în formă de ochi",
        "lac ochi formă privire adânc cerul uită",
        "Cerul se uită;Iris albastru",
    ),
    (
        "Un portocal cu fructe de lumină",
        "portocal fructe lumină copac seară culegem căldură",
        "Culegem căldură;Soare în crengi",
    ),
    (
        "O sirenă care coafează un val",
        "sirenă coafează val mare păr salon marin",
        "Salon marin;Mătasea apei",
    ),
    (
        "Un cortegiu de melci cu lumânări",
        "cortegiu melci lumânări încet noapte procesiune domoală",
        "Procesiune domoală;Lumină în cochilie",
    ),
    (
        "Un pod peste prăpastie făcut din cuvinte",
        "pod prăpastie cuvinte legătură vorbe",
        "Vorbele ne țin;Treci prin ce spui",
    ),
    (
        "O țărancă ce țese un covor din răsărit",
        "țărancă țese covor răsărit zori dimineții culorile",
        "Culorile dimineții;Război de lumină",
    ),
    (
        "O pereche de ochelari care privesc înapoi",
        "ochelari privesc înapoi vedere trecut fost",
        "Se vede ce a fost;Lentile de amintiri",
    ),
    (
        "Un călător cu o umbră mai mare decât el",
        "călător umbră mare lumină drum departe vede",
        "Se vede de departe;Coloană de întuneric",
    ),
    (
        "O lalea care înghite un fluture",
        "lalea înghite fluture floare ironie",
        "Cine pe cine?;Capcană de petale",
    ),
    (
        "O foaie albă pe care ninge",
        "foaie albă ninge hârtie zăpadă lucrurilor începutul",
        "Începutul tuturor lucrurilor;Poveste neîncepută",
    ),
    (
        "Un cocoș de vânt care arată spre mare",
        "cocoș vânt arată mare direcție încotro bate",
        "Încotro bate;Busolă de acoperiș",
    ),
    (
        "Un fotoliu rămas singur într-un câmp cu maci",
        "fotoliu câmp maci singur odihnă roșu",
        "Odihnă pe roșu;Cine a stat aici",
    ),
    (
        "Un cuptor din care ies stele",
        "cuptor stele ies pâine seară cerului",
        "Pâinea cerului;Foc și cosmos",
    ),
    (
        "Un cântec închis într-o scoică",
        "cântec scoică mare ureche ecou șoptește",
        "Marea șoptește;Amintire din valuri",
    ),
    (
        "O frânghie întinsă între două turnuri",
        "frânghie turnuri echilibru înălțime curaj greșit",
        "Un pas greșit;Mers pe fir",
    ),
    (
        "O oală care fierbe un curcubeu",
        "oală fierbe curcubeu culori gătit cerului rețeta",
        "Rețeta cerului;Mirosul culorii",
    ),
    (
        "Un steag pe care e desenat un zâmbet",
        "steag zâmbet desenat veselie piraterie bucuriei semnul",
        "Sub semnul bucuriei;Flamură fără război",
    ),
    (
        "Doi arbori care își țin crengile ca într-o îmbrățișare",
        "arbori crengi îmbrățișare iubire pădure rădăcini diferite",
        "Rădăcini diferite;Împreună în vânt",
    ),
    (
        "O pasăre de foc deasupra unei case de paie",
        "pasăre foc casă paie pericol stinge",
        "Cine stinge?;Splendoare periculoasă",
    ),
    (
        "O bibliotecă pe care o ține în spate un melc",
        "bibliotecă melc spate cărți lentoare cititor",
        "Cititor lent;Povara savantului",
    ),
    (
        "Un ciob de oglindă în care locuiește un soare mic",
        "ciob oglindă soare mic lumină strălucire strop",
        "Strop de strălucire;Fărâmă fierbinte",
    ),
    (
        "O noapte cusută cu fir de aur",
        "noapte cusută aur fir stele broderie cerească",
        "Broderie cerească;Luminițe pe catifea",
    ),
    (
        "O barcă ce se întoarce acasă cu pânze de vânt",
        "barcă întoarce acasă pânze vânt ajunge",
        "Ajunge la mal;Vânt bun la întoarcere",
    ),
    (
        "Un clopot pe un deal înconjurat de oi",
        "clopot deal oi sunet pășune stânii",
        "Sunetul stânii;Cheamă turma",
    ),
    (
        "Un zmeu în formă de dragon peste un oraș",
        "zmeu dragon oraș vânt copilărie jocul",
        "Jocul sus;Monstru de hârtie",
    ),
    (
        "O fată pe o balansoar de lună",
        "fată balansoar lună legănare noapte leagăn",
        "Leagăn pe cer;Visul copilăriei",
    ),
    (
        "Un cavou de sticlă plin de fluturi",
        "sticlă fluturi colecție fragil prizonier frumusețe fixată",
        "Frumusețe fixată;Muzeu al zborului",
    ),
    (
        "Un arici de ace de pin sub un brad",
        "arici ace pin brad iarnă mireasmă ghimpi",
        "Ghimpi cu mireasmă;Casă de crăciun",
    ),
    (
        "Un tunet care e doar un cap de dragon îndepărtat",
        "tunet cap dragon îndepărtat furtună urlet nori",
        "Urlet din nori;Cine mormăie?",
    ),
    (
        "O potecă pe care s-au pierdut pantofii",
        "potecă pantofi pierdut drum călătorie plecare urme",
        "Urme de plecare;Un pas rămas",
    ),
    (
        "Un calendar cu o singură zi înflorită",
        "calendar zi înflorită zi specială sărbătoare ziua",
        "Ziua aceea;Un cerc roșu pe floare",
    ),
    (
        "O pasăre de lut cu un fluier în coadă",
        "pasăre lut fluier coadă olar cântec limbă",
        "Cântec cu limbă de lut;Joacă de târg",
    ),
    (
        "O fereastră deschisă spre alt tavan",
        "fereastră deschisă tavan alt cer cealaltă camerei",
        "Cealaltă parte a camerei;Răsturnare",
    ),
    (
        "Un cerb de zăpadă care își pierde coarnele la dezgheț",
        "cerb zăpadă coarne dezgheț topire primăvara taie",
        "Primăvara îl taie;Frumusețe trecătoare",
    ),
    (
        "O lupă uriașă care mărește o picătură",
        "lupă uriașă picătură mărește univers lacrimă lumea",
        "Lumea dintr-o lacrimă;Detaliu de ploaie",
    ),
    (
        "O rochie de petale pe un manechin de ceață",
        "rochie petale manechin ceață eleganță dimineții moda",
        "Moda dimineții;Croială de primăvară",
    ),
    (
        "Un vânător care așteaptă un fluture",
        "vânător așteaptă fluture răbdare plasă ciudată ținta",
        "Ținta ciudată;Vânătoare blândă",
    ),
    (
        "Un râs care sare dintr-un coș cu mere",
        "râs coș mere sare bucurie rostogolește voioșia",
        "Se rostogolește voioșia;Vara a desfăcut capacul",
    ),
    (
        "Un măr care poartă pe el o hartă a lumii",
        "măr hartă lume gust continente pământul palmă",
        "Pământul în palmă;Fructul geografic",
    ),
    (
        "Un om care își poartă casa pe bicicletă",
        "om casă bicicletă nomad drum continuă mutarea",
        "Mutarea continuă;Pe două roți și o ușă",
    ),
    (
        "O barcă plină cu pisici într-un canal",
        "barcă pisici canal mieunat apă echipaj mustăți",
        "Echipaj cu mustăți;Pe apă, dar cu laba",
    ),
    (
        "Un ceas care bate doar în somn",
        "ceas bate somn noapte tic numără visele",
        "Cine numără visele;Ora de după ora",
    ),
    (
        "O ploaie de pene peste un oraș",
        "ploaie pene oraș ușor cade cerului perna",
        "Perna cerului;Zăpadă moale și caldă",
    ),
    (
        "Un gard pictat cu flori care se întind peste drum",
        "gard flori drum pictat sat culoarea limită",
        "Culoarea trece peste limită;Margini de vară",
    ),
    (
        "O stea de mare care ține o stea de pe cer",
        "stea mare ține cer reflexie pereche stele",
        "Pereche de stele;Jos și sus",
    ),
    (
        "O masă lungă cu scaune pentru toți strămoșii",
        "masă lungă scaune strămoși familie neamului cina",
        "Cina neamului;Locuri pentru cei plecați",
    ),
    (
        "Un nor în formă de elefant care umple o găleată",
        "nor elefant găleată apă ploaie dealul",
        "Udă dealul;Elefantul vărsat",
    ),
    (
        "O flaută cântând unui șarpe de smarald",
        "flaută șarpe smarald cântec vrajă magie bazar",
        "Magie de bazar;Dansul cu frica",
    ),
    (
        "Un deal cu o casă care are umbra în altă parte",
        "deal casă umbra alta ciudat regulă",
        "Ceva nu e în regulă;Seara mută lucrurile",
    ),
    (
        "O pasăre a nopții cu ochii de ceas",
        "pasăre noaptea ochi ceas bufniță orele vede",
        "Cea care vede orele;Privire de veghe",
    ),
    (
        "Un ciorchine de struguri în formă de clopot",
        "ciorchine struguri clopot toamnă dulce sunet",
        "Dulce ca un sunet;Recolta sună",
    ),
    (
        "O lacrimă care ține în ea un peisaj",
        "lacrimă peisaj sticlă tristețe lume rotundă plină",
        "Rotundă și plină;Ce a văzut ochiul",
    ),
    (
        "Doi frați de zăpadă care se țin de mână",
        "frați zăpadă mână iarnă topire soarele până",
        "Până vine soarele;Jucăria de iarnă",
    ),
    (
        "Un clovn singuratic pe o scenă goală",
        "clovn singuratic scenă goală tristețe imaginare aplauze",
        "Aplauze imaginare;Zâmbet pictat",
    ),
    (
        "O fântână în care cad scrisori",
        "fântână cad scrisori dorințe apă aruncate",
        "Dorințe aruncate;Cutia poștală a adâncului",
    ),
    (
        "Un morar care cerne făină de lumină",
        "morar cerne făină lumină moară albul zilei",
        "Albul zilei;Pânea soarelui",
    ),
    (
        "O mireasă cu voal de păianjen",
        "mireasă voal păianjen nuntă vechi dantelă vremuri",
        "Dantelă și vremuri;Pânză lungă de pază",
    ),
    (
        "Un cărăuș cu un car plin de tăceri",
        "cărăuș car tăceri liniște drum fragilă marfă",
        "Marfă fragilă;Zgomotul lipsă",
    ),
    (
        "O monedă care plutește pe un lac fără fund",
        "monedă plutește lac fără fund noroc dorință fost",
        "Ce dorință a fost;Aur pe apă",
    ),
    (
        "O iarnă ținută într-un borcan",
        "iarnă borcan frig ținută zăpadă anotimpului conserva",
        "Conserva anotimpului;Depozit de ger",
    ),
    (
        "O barcă mică pe o mare de lapte",
        "barcă mică mare lapte alb minunea albă",
        "Minunea albă;Pe valuri de cereale",
    ),
    (
        "O ploaie de petale peste o statuie",
        "ploaie petale statuie primăvară piatră înflorește",
        "Piatra înflorește;Cine mai e rece",
    ),
    (
        "Un nor ce poartă un clopot de aur",
        "nor clopot aur sunet cer anunță ceru",
        "Ceru-i anunță;Vestire de sus",
    ),
    (
        "Un cerc de cretă desenat în jurul unei umbre",
        "cerc cretă umbră contur desen asfalt prins",
        "Prins pe asfalt;Un loc pentru cineva",
    ),
    (
        "O bucătărie în care fierbe o furtună",
        "bucătărie fierbe furtună vas fulger tunetelor supa",
        "Supa tunetelor;Foc sub capac",
    ),
    (
        "Un clopoțel în pălăria unui om care nu aude",
        "clopoțel pălărie om aude sunet zgomot alții",
        "Zgomot pentru alții;Cum să te găsească",
    ),
    (
        "Doi bărbați care joacă șah pe o barcă",
        "bărbați șah barcă joc mare mutări valuri",
        "Mutări pe valuri;Regele pe apă",
    ),
    (
        "O pajiște unde fiecare floare e o clipă",
        "pajiște floare clipă timp culoare secunde cules",
        "Cules de secunde;Ierbar de momente",
    ),
    (
        "O trotinetă pe un drum de aur",
        "trotinetă drum aur viteză copil alergare ușoară",
        "Alergare ușoară;Avânt strălucitor",
    ),
    (
        "Un zeu mic în palma unui munte",
        "zeu mic palmă munte mărime ține",
        "Cine pe cine ține;Dimensiuni inversate",
    ),
    (
        "Un cântec în care dorm toți bursucii",
        "cântec dorm bursuci somn pădure lalele iarnă",
        "Lalele de iarnă;Cântec de leagăn",
    ),
    (
        "O coroană făcută din frunze și sticle",
        "coroană frunze sticle rege pădure maiestate reciclată",
        "Maiestate reciclată;Domnie de gunoi",
    ),
    (
        "Un cavaler ce își pierde calul într-o carte",
        "cavaler cal pierde carte aventură înghițit pagina",
        "Pagina a înghițit calul;Povestea l-a luat",
    ),
    (
        "O bufniță care citește o hartă stelară",
        "bufniță hartă stelară noapte citește cartograf nocturn",
        "Cartograf nocturn;Ochi pentru cer",
    ),
    (
        "Un butoi cu vin din care se aude muzica",
        "butoi vin muzică petrecere toamnă strugurii cântă",
        "Cântă strugurii;Zumzet de pivniță",
    ),
    (
        "O broască țestoasă care aleargă după o lumânare",
        "țestoasă aleargă lumânare lentă flacără imposibilă cursă",
        "Cursă imposibilă;Încet spre foc",
    ),
    (
        "O inimă de fier cu o floare",
        "inimă fier floare dur gingaș blândețe metal",
        "Câtă blândețe în metal;Dur și moale",
    ),
    (
        "O barcă de cartofi pe un râu de supă",
        "barcă cartofi râu supă foame marinari flămânzi",
        "Marinari flămânzi;Navigare în farfurie",
    ),
    (
        "Un tractor înflorit în mijlocul zăpezii",
        "tractor înflorit zăpadă iarnă tuns primăvara rătăcită",
        "Primăvara rătăcită;Mașina care visează",
    ),
    (
        "O fată care picură cerneală pe pagini de nor",
        "fată cerneală pagini nor scrie ploi",
        "Scrisoare pentru ploi;Povestește cerul",
    ),
    (
        "O dantelă de brumă pe fereastră",
        "dantelă brumă fereastră ger dimineață înghețată broderie",
        "Broderie înghețată;Lucrează iarna",
    ),
    (
        "O pereche de cizme în bălți cu stele",
        "cizme bălți stele ploaie joacă picioarele",
        "Sar în cer cu picioarele;Reflexii în noroi",
    ),
    (
        "O ceață care poartă haină de blană",
        "ceață haină blană rece iarnă pufoasă",
        "Gri și pufoasă;Se înfășoară în zori",
    ),
    (
        "Un moș cu barba plină de lumânări",
        "moș barbă lumânări bătrân lumină serbare",
        "Serbare de lumini;Înțelept aprins",
    ),
    (
        "Un tun care trage cu confeti",
        "tun confeti sărbătoare joc zgomot bătălie veselă",
        "Bătălie veselă;Pace cu mult zgomot",
    ),
    (
        "Un râu care curge pe o scară de piatră",
        "râu scară piatră curge cascadă treaptă",
        "Treaptă de apă;Cântă în pas",
    ),
    (
        "Un cuib în vârful unui turn de catedrală",
        "cuib turn catedrală pasăre înalt locuitori",
        "Cei mai sus locuitori;Cer și piatră",
    ),
    (
        "O vioară de furnici care cântă o petrecere",
        "vioară furnici petrecere cântă mici hotărâtor mărunt",
        "Mărunt dar hotărâtor;Orchestră de grădină",
    ),
    (
        "O poveste care aleargă desculță prin sat",
        "poveste aleargă desculță sat vorbă zvonul zboară",
        "Zvonul zboară;Se aude din poartă în poartă",
    ),
    (
        "Un ceas fără limbi pe un perete de ceață",
        "ceas fără limbi perete ceață timp fapt",
        "Cât e ceasul de fapt;Dincolo de minute",
    ),
    (
        "O rădăcină ce bea din cer",
        "rădăcină bea cer ploaie răsturnat copacul",
        "Copacul pe dos;Setea de sus",
    ),
    (
        "O pasăre în mijlocul unei cărți deschise",
        "pasăre carte deschisă zbor pagini zboară",
        "Cartea zboară;Eliberare literară",
    ),
    (
        "Un lan de grâu în care se joacă un copil mic",
        "lan grâu copil joacă aur mareea aurie",
        "Mareea aurie;Ascuns printre spice",
    ),
    (
        "O arhivă de nori etichetați",
        "arhivă nori etichetați ordine cer birocrația cerului",
        "Birocrația cerului;Dosare pufoase",
    ),
    (
        "O bicicletă ce se sprijină de un curcubeu",
        "bicicletă curcubeu sprijină ploaie culori colorat popas",
        "Popas colorat;Plimbare sfârșită",
    ),
    (
        "Un om cu o umbrelă într-un deșert",
        "om umbrelă deșert soare ironie sperată ploaie",
        "Ploaie sperată;Pregătit pentru orice",
    ),
    (
        "O pereche de mănuși care se țin de mână",
        "mănuși mână pereche iarnă iubire călduri lână",
        "Călduri de lână;Doi care nu se despart",
    ),
    (
        "Un ceainic care zbiară ca un tren",
        "ceainic zbiară tren abur fluier plecarea cinci",
        "Plecarea de la ora cinci;Aburii vor să plece",
    ),
    (
        "O ferigă pe un acoperiș de aur",
        "ferigă acoperiș aur verde umbră natura palat",
        "Natura pe palat;Sălbăticia se cațără",
    ),
    (
        "O ciutură care scoate o oră din fântână",
        "ciutură oră fântână timp apă trage",
        "Timpul se poate trage?;Apă de ceas",
    ),
    (
        "Un ghem de lână care se desface într-un drum",
        "ghem lână desface drum fir ariadnei firul",
        "Firul Ariadnei;Cămine și plecări",
    ),
    (
        "Un drum de săpun printre ghete",
        "drum săpun ghete curat aluneca departe merge",
        "Curățenia merge mai departe;Lunecuș de spumă",
    ),
    (
        "O lună care se oglindește într-un pahar",
        "lună pahar oglindește noapte sorb toast",
        "Noapte de toast;Un strop de cer",
    ),
    (
        "O vitrină de brutărie cu pâini în formă de case",
        "vitrină brutărie pâini case miros acoperișul mănâncă",
        "Se mănâncă acoperișul;Cartierul cald",
    ),
    (
        "Un vânător de vise cu o plasă de păianjen",
        "vânător vise plasă păianjen noapte pescar somn",
        "Pescar de somn;Capcană pentru coșmaruri",
    ),
    (
        "O cetate de gheață pe o farfurie",
        "cetate gheață farfurie rece topit servit palat",
        "Palat servit rece;Răcoare de masă",
    ),
    (
        "Doi ochi de lampă într-o pădure neagră",
        "ochi lampă pădure neagră priveghere uită",
        "Cine se uită;Păduri cu privire",
    ),
    (
        "Un porumbel purtând o frunză de măslin",
        "porumbel frunză măslin pace vestire potop",
        "După potop;Vestitorul alb",
    ),
    (
        "O oală cu aur la capătul unui drum de ploaie",
        "oală aur drum ploaie comoară curcubeu",
        "Capăt de curcubeu;Noroc ud",
    ),
    (
        "O balenă care ține pe spate un oraș cu lumini",
        "balenă spate oraș lumini mare insulă",
        "Insulă vie;Metropolă care înoată",
    ),
    (
        "O păpădie care ține o poveste în fiecare puf",
        "păpădie poveste puf vânt semințe suflă plouă",
        "Suflă și plouă cu povești;Cuvinte plutitoare",
    ),
    (
        "Un nor-bătrân sprijinit într-un baston de fulger",
        "nor bătrân baston fulger vechime vârstei mânia",
        "Mânia vârstei;Rege bătrân al furtunii",
    ),
    (
        "O ușă mică în trunchiul unui stejar",
        "ușă mică trunchi stejar locuitor înăuntru",
        "Cine stă înăuntru;Intrare pentru pitici",
    ),
    (
        "O țară pe o hartă ce nu se mai termină",
        "țară hartă nesfârșit orizont linie granița fuge",
        "Granița fuge;Pământ nou la fiecare pas",
    ),
    (
        "Un dans de umbre pe peretele unei sobe",
        "dans umbre perete sobă flăcări teatrul focului",
        "Teatrul focului;Seri în jurul căldurii",
    ),
    (
        "O stradă cu casele făcute din cărți",
        "stradă casele cărți oraș lectură scriitorilor",
        "Cartier al scriitorilor;Paginile locuiesc",
    ),
    (
        "O ploaie care cântă la fiecare acoperiș altfel",
        "ploaie cântă acoperiș notă sunete orchestră olane",
        "Orchestră de olane;Fiecare casă, alt ton",
    ),
    (
        "Un țap negru cu clopot de argint",
        "țap negru clopot argint sat păstorul noaptea",
        "Păstorul noaptea;Zgomot nobil",
    ),
    (
        "O fântână pe un acoperiș de gară",
        "fântână acoperiș gară apă sete călători",
        "Sete pentru călători;Izvor suspendat",
    ),
    (
        "O ceainărie pe o insulă care plutește",
        "ceainărie insulă plutește ceai liniște",
        "Ora zen;Ceai pe valuri",
    ),
    (
        "O rochie de bal făcută din ziare",
        "rochie bal ziare vești eleganță știri modă",
        "Știri la modă;Dans cu titluri",
    ),
    (
        "Un tablou al unui cer pictat în nopți diferite",
        "tablou cer nopți pictat culori zece",
        "Zece nopți într-una;Pensulă de lună",
    ),
    (
        "O gară de hârtie în care pleacă trenuri de aburi",
        "gară hârtie trenuri aburi plecare jucării cutia",
        "Cutia cu jucării;Peron de poveste",
    ),
    (
        "Un șoarece care poartă coroana unei pisici",
        "șoarece coroană pisică mic mare împrumutat tron",
        "Tron împrumutat;Mic și îndrăzneț",
    ),
    (
        "Un stol de ceasuri care migrează",
        "stol ceasuri migrează timp toamnă pleacă orele",
        "Orele pleacă spre sud;Calendar zburător",
    ),
];
