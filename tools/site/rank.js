// ---- Clasamente (injectat în index.html de gen_games.py)
const REG=["world","europe","africa","asia","namerica","samerica","oceania","other","romania"],CATS=["abstract","classic","family","strategy","party","coop","cards","rpg","minis","tcg","kids","dice"],LIDS=REG.concat(CATS.map(c=>"c-"+c)),GRP=[REG,LIDS.slice(REG.length)],last=[REG[0],LIDS[REG.length]],isC=l=>l[0]==="c"&&l[1]==="-",RKC=["classic","abstract","family","strategy","party","coop","cards","rpg","minis","tcg","kids"];
const STI={play:"●",var:"◆",kit:"▲",brand:"✕",plan:"◇"};
const GM={};GAMES.forEach(g=>GM[g.i]=g);
let rkTab="world",rkCat="all",rkQ="";
function badge(s,t){const b=document.createElement("span");b.className="bd bd-"+s;const i=document.createElement("span");i.setAttribute("aria-hidden","true");i.textContent=STI[s];b.append(i,document.createTextNode(t["st_"+s]));return b}
function rkExtra(lid,x,li){
  if(lid==="romania"){const o=OBS[x];return o?o[li]:x}
  return x.split(" / ").map(a=>{const o=ORI[a];return o?o[li]:a}).join(" / ");
}
function rkRows(lid){
  const t=T[lang],li=LANGS.indexOf(lang),q=norm(rkQ.trim());
  return RANK[lid].filter(r=>{
    if(!isC(lid)&&rkCat!=="all"&&r[2]!==rkCat)return false;
    if(q){const g=r[5]?GM[r[5]]:null;const hay=norm([r[1],rkExtra(lid,r[3],li),g?LANGS.map(l=>g[l]||g.en).join(" "):""].join(" "));if(!hay.includes(q))return false}
    return true;
  });
}
function renderPanel(lid){
  const t=T[lang],li=LANGS.indexOf(lang),box=$("#rkp-"+lid);box.textContent="";
  const all=RANK[lid],rows=rkRows(lid);
  const np=all.filter(r=>r[4]==="play").length,nv=all.filter(r=>r[4]==="var").length;
  const sum=document.createElement("p");sum.className="rk-sum";sum.textContent=fmt(t.rk_sum,{n:np+nv,p:np,v:nv,t:all.length});box.appendChild(sum);
  const wrap=document.createElement("div");wrap.className="tbl-wrap";wrap.setAttribute("role","region");wrap.tabIndex=0;
  const cn=isC(lid),ln=cn?t["cat_"+lid.slice(2)]:t["rk_t_"+lid],cap=fmt(cn?t.rk_ccap:t.rk_cap,{list:ln});
  wrap.setAttribute("aria-label",cap+" — "+t.rk_scroll);
  const tb=document.createElement("table"),cp=document.createElement("caption");cp.textContent=cap;tb.appendChild(cp);
  const th=document.createElement("thead"),hr=document.createElement("tr");
  (cn?[t.th_place,t.th_game]:[t.th_place,t.th_game,t.th_cat]).concat([lid==="romania"?t.th_obs:lid==="world"||lid==="europe"?t.th_origin:t.th_where,t.th_status,t.th_page]).forEach(h=>{const c=document.createElement("th");c.scope="col";c.textContent=h;hr.appendChild(c)});
  th.appendChild(hr);tb.appendChild(th);
  const tbd=document.createElement("tbody");
  rows.forEach(r=>{
    const tr=document.createElement("tr");
    const c1=document.createElement("th");c1.scope="row";c1.className="r";c1.textContent=r[0];
    const c2=document.createElement("td"),nm=document.createElement("strong");nm.textContent=r[1];c2.appendChild(nm);
    if(r[5]&&GM[r[5]]){const ti=gname(GM[r[5]]);if(norm(ti)!==norm(r[1])){const o=document.createElement("span");o.className="ours";o.textContent=fmt(t.rk_ours,{t:ti});c2.appendChild(o)}}
    const c3=document.createElement("td");if(!cn)c3.textContent=t["cat_"+r[2]];
    const c4=document.createElement("td");c4.textContent=rkExtra(cn?"world":lid,r[3],li);
    const c5=document.createElement("td");c5.appendChild(badge(r[4],t));
    const c6=document.createElement("td");
    if(r[5]&&GM[r[5]]){const a=document.createElement("a");a.href="jocuri/"+r[5]+".html?lang="+lang;a.textContent=t.rk_page;a.setAttribute("aria-label",t.rk_page+": "+gname(GM[r[5]]));c6.appendChild(a)}
    tr.append(c1,c2);if(!cn)tr.append(c3);tr.append(c4,c5,c6);tbd.appendChild(tr);
  });
  if(!rows.length){const tr=document.createElement("tr"),td=document.createElement("td");td.colSpan=cn?5:6;td.textContent=t.rk_none;tr.appendChild(td);tbd.appendChild(tr)}
  tb.appendChild(tbd);wrap.appendChild(tb);box.appendChild(wrap);
}
function rkCount(){$("#rk-count").textContent=fmt(T[lang].rk_shown,{n:rkRows(rkTab).length,t:RANK[rkTab].length})}
function renderRank(){
  const t=T[lang],sel=$("#rk-cat");sel.textContent="";
  [["all",t.cat_all]].concat(RKC.map(k=>[k,t["cat_"+k]])).forEach(([k,l])=>{const o=document.createElement("option");o.value=k;o.textContent=l;sel.appendChild(o)});
  sel.value=rkCat;
  LIDS.forEach(renderPanel);rkCount();
  const lg=$("#rk-legend");lg.textContent="";
  ["play","var","kit","plan","brand"].forEach(s=>{const li=document.createElement("li"),sp=document.createElement("span");sp.className="t";sp.textContent=t["stl_"+s];li.append(badge(s,t),sp);lg.appendChild(li)});
  const nl=$("#rk-notes");nl.textContent="";
  [1,2,3,4,5,6,7,8,9,10,11].forEach(n=>{const li=document.createElement("li");li.textContent=t["rk_n"+n];nl.appendChild(li)});
}
function selectTab(id,focus){
  rkTab=id;const g=GRP[isC(id)?1:0];last[isC(id)?1:0]=id;
  $$("#rk-tabs [role=tab],#rk-ctabs [role=tab]").forEach(b=>{const on=b.dataset.lid===id,mine=g.includes(b.dataset.lid);b.setAttribute("aria-selected",String(on));b.tabIndex=(mine?on:last[0]===b.dataset.lid||last[1]===b.dataset.lid)?0:-1;if(on&&focus)b.focus()});
  LIDS.forEach(l=>{$("#rkp-"+l).hidden=l!==id});$("#rk-catbox").hidden=isC(id);rkCount();
}
$("#rk-cat").addEventListener("change",e=>{rkCat=e.target.value;LIDS.forEach(renderPanel);rkCount()});
$("#rk-q").addEventListener("input",e=>{rkQ=e.target.value;LIDS.forEach(renderPanel);rkCount()});
$$("#rk-tabs [role=tab],#rk-ctabs [role=tab]").forEach(b=>{
  b.addEventListener("click",()=>selectTab(b.dataset.lid,false));
  b.addEventListener("keydown",e=>{
    const G=GRP[isC(b.dataset.lid)?1:0],i=G.indexOf(b.dataset.lid);let j=-1;
    if(e.key==="ArrowRight")j=(i+1)%G.length;else if(e.key==="ArrowLeft")j=(i+G.length-1)%G.length;else if(e.key==="Home")j=0;else if(e.key==="End")j=G.length-1;
    if(j>=0){e.preventDefault();selectTab(G[j],true)}
  });
});
function hashTab(){const m=new RegExp("^#rankings-("+LIDS.join("|")+")$").exec(location.hash);if(m){selectTab(m[1],false)}}
addEventListener("hashchange",hashTab);hashTab();
