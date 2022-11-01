import{d as p,I as h,R as u,s as m,u as f,_ as g,r as v,o as N,c as w,e as t,t as l,f as i,j as S,v as y,p as I,k}from"./index.3c790e21.js";import{u as _}from"./notes.5fa31621.js";import{m as a,H as c}from"./index.867c1d86.js";a.setOptions({renderer:new a.Renderer,highlight:function(e,o){const r=c.getLanguage(o)?o:"plaintext";return c.highlight(e,{language:r}).value},langPrefix:"hljs language-",pedantic:!1,gfm:!0,breaks:!1,sanitize:!1,smartypants:!1,xhtml:!1});const $=p({name:"EditEmailView",components:{Icon:h,AppCard:u},data:()=>({showOptions:!1}),computed:{noteId(){return this.$route.params.noteId},...m(_,["getNoteById"]),fetchedNote(){return this.getNoteById(String(this.noteId))},markedContent(){return a.parse(String(this.fetchedNote.content))}},methods:{...f(_,["deleteNote"]),deleteNoteRequest(){this.deleteNote(String(this.fetchedNote.id)),this.$router.go(-1)}}});const d=e=>(I("data-v-311a810e"),e=e(),k(),e),C={id:"edit__note__page"},E={class:"note__entry"},O={class:"note__entry__header"},B={class:"note__entry__header__date"},D=d(()=>t("span",null," Star ",-1)),R=d(()=>t("span",null," Delete ",-1)),V=d(()=>t("span",null," Edit ",-1)),H={class:"note__title"},L={class:"note__entry__content"},q=["innerHTML"];function A(e,o,r,M,b,j){const s=v("Icon");return N(),w("div",C,[t("div",E,[t("div",O,[t("p",B,l(new Date(e.fetchedNote.dateAdded.toString()).toLocaleDateString(void 0,{weekday:"short",year:"numeric",month:"short",day:"numeric"})),1),i(s,{icon:"mdi:dots-vertical",onClick:o[0]||(o[0]=n=>e.showOptions=!0),onMouseover:o[1]||(o[1]=n=>e.showOptions=!0)}),S(t("div",{id:"edit__note__controls",onClick:o[3]||(o[3]=n=>e.showOptions=!1)},[t("div",null,[i(s,{icon:"mdi:star-outline",class:"star"}),D]),t("div",{onClick:o[2]||(o[2]=(...n)=>e.deleteNoteRequest&&e.deleteNoteRequest(...n))},[i(s,{icon:"mdi:trash-can-outline",class:"delete"}),R]),t("div",null,[i(s,{icon:"mdi:clipboard-edit-outline",class:"edit"}),V])],512),[[y,e.showOptions]])]),t("h3",H,l(e.fetchedNote.title),1),t("div",L,[t("p",{innerHTML:e.markedContent},null,8,q)])])])}const P=g($,[["render",A],["__scopeId","data-v-311a810e"]]);export{P as default};
