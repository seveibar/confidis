(this.webpackJsonptryonline=this.webpackJsonptryonline||[]).push([[0],{164:function(e,t,n){e.exports=n(174)},169:function(e,t,n){},174:function(e,t,n){"use strict";n.r(t);var a,r=n(0),o=n.n(r),i=n(150),c=n.n(i),l=(n(169),n(156)),s=n(184),d=n(5),u=n(185),m=function(e){var t=e.children;return o.a.createElement(s.a,{display:"flex",justifyContent:"center"},o.a.createElement(s.a,{maxWidth:1200,width:"100%"},t))},E=n(154),f=n(26),b=n(155);n.e(3).then(n.bind(null,193)).then((function(e){a=e}));var g="\n\nSET q1 a FROM s1\nSET q1 a FROM s2\nSET q1 a FROM s3\nSET q1 w FROM s4\n\nSET q2 b FROM s1\nSET q2 c FROM s2\nSET q2 b FROM s3\nSET q2 w FROM s4\n\nSET q3 d FROM s1\nSET q4 w FROM s4\n\nGET ANSWER TO q1\nGET ANSWER TO q2\nGET ANSWER TO q3\nGET ANSWER TO q4\n\n".trim(),p=Object(l.a)(s.a)({padding:16}),h=Object(l.a)(b.a)({fontFamily:"monospace",boxSizing:"border-box",width:"100%",fontSize:24,minHeight:600,padding:16,border:"1px solid ".concat(d.a.grey[500]),resize:"both",overflow:"auto"}),O=Object(l.a)(s.a)({display:"inline-block",fontWeight:900,fontSize:24,color:d.a.grey[800],borderBottom:"2px solid ".concat(d.a.grey[400]),paddingBottom:6,paddingLeft:4,paddingRight:16,marginBottom:16}),y=function(){var e=Object(r.useState)(g),t=Object(f.a)(e,2),n=t[0],i=t[1],c=Object(r.useState)("Computing in WebAssembly..."),l=Object(f.a)(c,2),d=l[0],m=l[1],b=Object(r.useState)(!1),y=Object(f.a)(b,2),v=y[0],w=y[1];return Object(r.useEffect)((function(){if(!v){var e=setInterval((function(){a&&(w(!0),clearInterval(e))}),100);return function(){clearInterval(e)}}}),[v,w]),Object(r.useEffect)((function(){if(a){var e,t=a.GraphJS.new(),r=[],o=Object(E.a)(n.split("\n"));try{for(o.s();!(e=o.n()).done;){var i=e.value;try{var c=t.execute_command(i.trim());"GetAnswer"===c.cmd?r.push("".concat(c.answer," (").concat((100*c.confidence).toFixed(3),"%)")):r.push("")}catch(l){"Blank command"===l.toString()?r.push(""):r.push("Err: "+l.toString())}}}catch(s){o.e(s)}finally{o.f()}m(r.join("\n"))}}),[n,v]),o.a.createElement(p,null,o.a.createElement(u.a,{container:!0},o.a.createElement(u.a,{item:!0,xs:12},o.a.createElement(s.a,{display:"flex"},o.a.createElement(O,null,"Try It Out"),o.a.createElement(s.a,{flexGrow:1})),o.a.createElement(s.a,{style:{position:"relative"}},o.a.createElement(h,{contentEditable:!0,value:n,onChange:function(e){return i(e.target.value)}}),o.a.createElement(h,{value:d,style:{display:"inline-block",pointerEvents:"none",position:"absolute",top:0,right:0,border:"none",backgroundColor:"transparent",width:400,opacity:.5}})))))},v=Object(l.a)(s.a)({marginTop:40}),w=Object(l.a)(s.a)({paddingBottom:40}),S=Object(l.a)(s.a)({padding:20}),T=Object(l.a)(s.a)({paddingTop:80,color:d.a.grey[600],"& a":{color:d.a.blue[500]}}),j=Object(l.a)(s.a)({fontWeight:900,fontSize:96}),R=Object(l.a)(s.a)({fontWeight:800,fontSize:48,marginTop:20}),q=Object(l.a)(s.a)({fontSize:18,lineHeight:1.5,padding:8}),x=Object(l.a)(s.a)({});var F=function(){return o.a.createElement(v,null,o.a.createElement(m,null,o.a.createElement(w,null,o.a.createElement(u.a,{container:!0},o.a.createElement(u.a,{xs:12,md:8,item:!0},o.a.createElement(S,null,o.a.createElement(j,null,"Confidis"),o.a.createElement(R,null,"A probabilistic key store for finding truth from arguing sources"))),o.a.createElement(u.a,{xs:12,md:4,item:!0},o.a.createElement(T,null,o.a.createElement(q,null,"Confidis focuses on ease-of-use, correctness, performance, and resilience against adverserial scenarios. The library is written in Rust and can be used as an npm module.",o.a.createElement("br",null),o.a.createElement("br",null),"Confidis was originally designed by"," ",o.a.createElement("a",{href:"https://twitter.com/seveibar"},"@seveibar")," to analyze disagreeing data sources for data aggregation. It was further developed and sponsored by"," ",o.a.createElement("a",{href:"https://wao.ai"},"wao.ai"),".",o.a.createElement("br",null),o.a.createElement("br",null),o.a.createElement("a",{href:"https://github.com/waoai/confidis"},"Read more on the README.md")))))),o.a.createElement(x,null,o.a.createElement(y,null))))};Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));c.a.render(o.a.createElement(F,null),document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then((function(e){e.unregister()})).catch((function(e){console.error(e.message)}))}},[[164,1,2]]]);
//# sourceMappingURL=main.a57a14c9.chunk.js.map