// Lightbox accesibil pentru capturile din galerii (injectat de gen_pages.py în paginile de joc care au capturi).
// LOGIC (testat în tools/site/test_lightbox.js)
function lbStep(i,n,k){
  if(n<1)return 0;
  switch(k){
    case "ArrowRight":case "PageDown":case "next":return (i+1)%n;
    case "ArrowLeft":case "PageUp":case "prev":return (i+n-1)%n;
    case "Home":return 0;
    case "End":return n-1;
  }
  return i;
}
function lbSwipe(dx,dy){
  if(Math.abs(dx)<50||Math.abs(dx)<Math.abs(dy)*1.2)return "";
  return dx<0?"next":"prev";
}
// /LOGIC
(function(){
const links=$$("a.lb-t");
if(!links.length)return;
const n=links.length,root=document.documentElement;
let dlg,img,cnt,cap,orig,btnF,cur=0,opener=null,tx=0,ty=0;
function lt(k,o){const v=T[lang][k]||T.en[k]||k;return o?fmt(v,o):v}
function build(){
  dlg=document.createElement("dialog");dlg.className="lb";
  dlg.innerHTML='<div class="lb-st"><div class="lb-top"><div class="lb-info"><div class="lb-cnt" role="status" aria-live="polite"></div><div class="lb-cap"></div></div><a class="lb-b lb-o" target="_blank" rel="noopener">&#8599;</a><button type="button" class="lb-b lb-f">&#9974;</button><button type="button" class="lb-b lb-x">&times;</button></div><div class="lb-main"><button type="button" class="lb-b lb-nav lb-p">&#8249;</button><img alt=""><button type="button" class="lb-b lb-nav lb-n">&#8250;</button></div></div>';
  document.body.appendChild(dlg);
  img=dlg.querySelector("img");cnt=dlg.querySelector(".lb-cnt");cap=dlg.querySelector(".lb-cap");orig=dlg.querySelector(".lb-o");btnF=dlg.querySelector(".lb-f");
  dlg.querySelector(".lb-x").addEventListener("click",()=>dlg.close());
  dlg.querySelector(".lb-p").addEventListener("click",()=>go("prev"));
  dlg.querySelector(".lb-n").addEventListener("click",()=>go("next"));
  if(!document.fullscreenEnabled)btnF.hidden=true;
  btnF.addEventListener("click",()=>{
    try{
      if(document.fullscreenElement)document.exitFullscreen();
      else dlg.querySelector(".lb-st").requestFullscreen().catch(()=>{});
    }catch(e){}
  });
  dlg.addEventListener("click",e=>{if(!e.target.closest("button,a,img,.lb-info"))dlg.close()});
  dlg.addEventListener("keydown",e=>{
    if(e.altKey||e.ctrlKey||e.metaKey)return;
    if(["ArrowLeft","ArrowRight","PageUp","PageDown","Home","End"].includes(e.key)){e.preventDefault();go(e.key)}
  });
  const m=dlg.querySelector(".lb-main");
  m.addEventListener("touchstart",e=>{const t=e.changedTouches[0];tx=t.clientX;ty=t.clientY},{passive:true});
  m.addEventListener("touchend",e=>{const t=e.changedTouches[0],s=lbSwipe(t.clientX-tx,t.clientY-ty);if(s)go(s)},{passive:true});
  dlg.addEventListener("close",()=>{
    root.classList.remove("lb-open");
    if(document.fullscreenElement)try{document.exitFullscreen()}catch(e){}
    if(opener&&document.contains(opener))opener.focus();
  });
}
function show(i){
  cur=i;const a=links[i],th=a.querySelector("img");
  img.src=a.href;img.alt=th?th.alt:"";
  cnt.textContent=(i+1)+" / "+n;
  const s=+a.dataset.s,g=lt(a.dataset.g==="gui"?"pg_gui_h":"pg_cli_h");
  cap.textContent=g+(s>=1&&s<=3?" · "+lt("lb_s"+s):"");
  orig.href=a.href;
  [(i+1)%n,(i+n-1)%n].forEach(j=>{const p=new Image();p.src=links[j].href});
}
function go(k){show(lbStep(cur,n,k))}
function open(i,a){
  if(!dlg)build();
  opener=a||null;
  dlg.setAttribute("aria-label",lt("lb_dialog"));
  dlg.querySelector(".lb-x").setAttribute("aria-label",lt("lb_close"));dlg.querySelector(".lb-x").title=lt("lb_close");
  dlg.querySelector(".lb-p").setAttribute("aria-label",lt("lb_prev"));dlg.querySelector(".lb-p").title=lt("lb_prev");
  dlg.querySelector(".lb-n").setAttribute("aria-label",lt("lb_next"));dlg.querySelector(".lb-n").title=lt("lb_next");
  btnF.setAttribute("aria-label",lt("lb_full"));btnF.title=lt("lb_full");
  orig.setAttribute("aria-label",lt("lb_orig"));orig.title=lt("lb_orig");
  root.classList.add("lb-open");
  show(i);
  if(typeof dlg.showModal==="function")dlg.showModal();else dlg.setAttribute("open","");
  dlg.querySelector(".lb-x").focus();
}
links.forEach((a,i)=>{
  a.setAttribute("aria-haspopup","dialog");
  a.addEventListener("click",e=>{
    if(e.ctrlKey||e.metaKey||e.shiftKey||e.altKey||e.button>0)return;
    e.preventDefault();open(i,a);
  });
});
/*TEST_HOOK*/
})();
