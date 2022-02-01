"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[69],{3905:function(e,r,t){t.d(r,{Zo:function(){return s},kt:function(){return d}});var n=t(7294);function o(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function a(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);r&&(n=n.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,n)}return t}function c(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?a(Object(t),!0).forEach((function(r){o(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):a(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function i(e,r){if(null==e)return{};var t,n,o=function(e,r){if(null==e)return{};var t,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)t=a[n],r.indexOf(t)>=0||(o[t]=e[t]);return o}(e,r);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)t=a[n],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(o[t]=e[t])}return o}var u=n.createContext({}),l=function(e){var r=n.useContext(u),t=r;return e&&(t="function"==typeof e?e(r):c(c({},r),e)),t},s=function(e){var r=l(e.components);return n.createElement(u.Provider,{value:r},e.children)},p={inlineCode:"code",wrapper:function(e){var r=e.children;return n.createElement(n.Fragment,{},r)}},f=n.forwardRef((function(e,r){var t=e.components,o=e.mdxType,a=e.originalType,u=e.parentName,s=i(e,["components","mdxType","originalType","parentName"]),f=l(t),d=o,b=f["".concat(u,".").concat(d)]||f[d]||p[d]||a;return t?n.createElement(b,c(c({ref:r},s),{},{components:t})):n.createElement(b,c({ref:r},s))}));function d(e,r){var t=arguments,o=r&&r.mdxType;if("string"==typeof e||o){var a=t.length,c=new Array(a);c[0]=f;var i={};for(var u in r)hasOwnProperty.call(r,u)&&(i[u]=r[u]);i.originalType=e,i.mdxType="string"==typeof e?e:o,c[1]=i;for(var l=2;l<a;l++)c[l]=t[l];return n.createElement.apply(null,c)}return n.createElement.apply(null,t)}f.displayName="MDXCreateElement"},6738:function(e,r,t){t.r(r),t.d(r,{frontMatter:function(){return i},contentTitle:function(){return u},metadata:function(){return l},toc:function(){return s},default:function(){return f}});var n=t(7462),o=t(3366),a=(t(7294),t(3905)),c=["components"],i={sidebar_position:3},u="Global Chat",l={unversionedId:"Structure/Servers/global-chat",id:"Structure/Servers/global-chat",title:"Global Chat",description:"The core logic is working!",source:"@site/docs/Structure/Servers/global-chat.md",sourceDirName:"Structure/Servers",slug:"/Structure/Servers/global-chat",permalink:"/TheBarrels/docs/Structure/Servers/global-chat",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"OAuth server",permalink:"/TheBarrels/docs/Structure/Servers/auth-server"},next:{title:"Web",permalink:"/TheBarrels/docs/Structure/Web/web-general"}},s=[],p={toc:s};function f(e){var r=e.components,t=(0,o.Z)(e,c);return(0,a.kt)("wrapper",(0,n.Z)({},p,t,{components:r,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"global-chat"},"Global Chat"),(0,a.kt)("p",null,"The core logic is working!"),(0,a.kt)("p",null,"This server will depend on ",(0,a.kt)("a",{parentName:"p",href:"auth-server"},"OAuth server")),(0,a.kt)("p",null,"After the user connects, the first message they sent should be a valid token\notherwise they will be imediatly disconnected (Not authenticated)."),(0,a.kt)("p",null,"After they server validate the user token, they are connected and can start sending messages."))}f.isMDXComponent=!0}}]);