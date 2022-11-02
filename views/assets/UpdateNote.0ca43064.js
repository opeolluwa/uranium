import{d as f,a as B,B as E,s as _,u as h,_ as y,r as n,o as s,c as r,f as d,l as v,e as a,j as b,C as I,w as N,p as w,k as V}from"./index.1e245a18.js";import{B as g}from"./BaseTextarea.0273d02e.js";import{E as M}from"./style.3349eec2.js";import{u as l}from"./notes.1e5caa52.js";const S=f({name:"CreateNewNoteEntry",data:()=>({note:{title:"",content:""},useMdEditor:!1,mdToolBarExclude:["github"]}),components:{BaseButton:B,BaseTextInput:E,BaseTextarea:g,MdEditor:M},computed:{..._(l,["getNoteById"]),noteId(){return this.$route.params.noteId},currentNote(){return this.getNoteById(String(this.noteId))}},methods:{...h(l,["createNewEntry"]),async updateEntry(){await this.editNote(String(this.noteId),{...this.note})&&(Object.assign(this.note,{title:"",content:""}),this.$router.replace({name:"all-notes"}))}}});const i=e=>(w("data-v-ea2dc4a7"),e=e(),V(),e),k={key:1,id:"md__editor"},x=i(()=>a("label",{for:"Content"},"Content",-1)),C={id:"editor__type"},T=["toolbarsExclude"],U=i(()=>a("label",{for:"editor-style"},"Use Markdown Editor",-1));function $(e,t,R,j,q,A){const u=n("BaseTextInput"),p=n("BaseTextarea"),c=n("MdEditor"),m=n("BaseButton");return s(),r("form",{onSubmit:t[4]||(t[4]=N((...o)=>e.updateEntry&&e.updateEntry(...o),["prevent"]))},[d(u,{label:"title",type:"text",placeholder:"Ex: how to create repo",modelValue:e.note.title,"onUpdate:modelValue":t[0]||(t[0]=o=>e.note.title=o),class:"field"},null,8,["modelValue"]),e.useMdEditor?(s(),r("div",k,[x,d(c,{modelValue:e.note.content,"onUpdate:modelValue":t[2]||(t[2]=o=>e.note.content=o),preview:!1,language:"en-US",placeholder:"provide note details here",style:{"margin-bottom":"20px"}},null,8,["modelValue"])])):(s(),v(p,{key:0,placeholder:"provide note details here",label:"content",modelValue:e.note.content,"onUpdate:modelValue":t[1]||(t[1]=o=>e.note.content=o)},null,8,["modelValue"])),a("div",C,[b(a("input",{type:"checkbox",name:"editor-style",id:"","onUpdate:modelValue":t[3]||(t[3]=o=>e.useMdEditor=o),toolbarsExclude:e.mdToolBarExclude,showCodeRowNumber:!0},null,8,T),[[I,e.useMdEditor]]),U]),d(m,{text:"Save changes",class:"field"})],32)}const G=y(S,[["render",$],["__scopeId","data-v-ea2dc4a7"]]);export{G as default};