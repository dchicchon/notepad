import{r as n,m as B,d as t,j as e,F as l,a as i,B as r,b as c,g as d,i as N,s as u,f as p,c as O,R as P}from"./jsx-runtime.eca01024.js";function R(){const[b,m]=n.exports.useState(12),[w,F]=n.exports.useState(""),[A,S]=n.exports.useState([]),[y,k]=n.exports.useState("white"),[x,C]=n.exports.useState("#282c34");n.exports.useEffect(()=>{B.setTitle("Preferences");async function h(){console.log("Get Preferences");let a=await d(l),o=await d(i),g=await d(r),f=await d(c),v=await N("get_font_families");console.log("FontSize:",a),console.log("FontColor:",o),console.log("BackgroundColor:",g),console.log("FontFamily:",f),console.log("FontFamilies:",v),a&&m(a),o&&k(o),g&&C(g),f&&F(f),v&&S(v)}h()},[]);const s=async h=>{const{name:a,value:o}=h.target;switch(a){case l:m(o),await u(l,o),p("update-setting",l);break;case i:k(o),await u(i,o),p("update-setting",i);break;case r:C(o),await u(r,o),p("update-setting",r);break;case c:F(o),await u(c,o),p("update-setting",c);break}};return t("div",{id:"preferences",children:[t("div",{className:"input-group",children:[e("label",{children:"Font Size:"}),t("select",{name:l,value:b,onChange:s,children:[e("option",{value:8,children:"8"}),e("option",{value:12,children:"12"}),e("option",{value:14,children:"14"}),e("option",{value:16,children:"16"}),e("option",{value:20,children:"20"}),e("option",{value:24,children:"24"}),e("option",{value:28,children:"28"}),e("option",{value:32,children:"32"})]})]}),t("div",{className:"input-group",children:[e("label",{children:"Font Color:"}),t("select",{name:i,value:y,onChange:s,children:[e("option",{value:"black",children:"Black"}),e("option",{value:"white",children:"White"}),e("option",{value:"darkgreen",children:"Dark Green"}),e("option",{value:"pink",children:"Pink"})]})]}),t("div",{className:"input-group",children:[e("label",{children:"Background Color:"}),t("select",{name:r,value:x,onChange:s,children:[e("option",{value:"black",children:"Black"}),e("option",{value:"#282c34",children:"Deep Purple"}),e("option",{value:"white",children:"White"}),e("option",{value:"darkgreen",children:"Dark Green"}),e("option",{value:"pink",children:"Pink"})]})]}),t("div",{className:"input-group",children:[e("label",{children:"Font Family (check https://fonts.google.com/ for fonts to import)"}),e("select",{name:c,value:w,onChange:s})]})]})}const _=document.getElementById("root"),z=O(_);z.render(e(P.StrictMode,{children:e(R,{})}));
