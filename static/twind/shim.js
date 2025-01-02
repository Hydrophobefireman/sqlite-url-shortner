import "./style-vendorizer.js";

import {createObserver} from "./observe.js";
import {setup as setup$1} from "./twind.js";

export {
  apply,
  autoprefix,
  create,
  cssomSheet,
  directive,
  expandGroups,
  hash,
  mode,
  noprefix,
  silent,
  strict,
  theme,
  tw,
  voidSheet,
  warn,
} from "./twind.js";
if (typeof document !== "undefined" && typeof addEventListener == "function") {
  onload = () => {
    const script = document.querySelector('script[type="twind-config"]');
    setup(script ? JSON.parse(script.innerHTML) : {});
  };
  if (document.readyState === "loading") {
    addEventListener("DOMContentLoaded", onload);
  } else {
    timeoutRef = setTimeout(onload);
  }
}
var onload;
var timeoutRef;
var observer = createObserver();
var disconnect = () => {
  if (onload) {
    removeEventListener("DOMContentLoaded", onload);
    clearTimeout(timeoutRef);
  }
  observer.disconnect();
};
var setup = ({target = document.documentElement, ...config} = {}) => {
  if (Object.keys(config).length) {
    setup$1(config);
  }
  disconnect();
  observer.observe(target);
  target.hidden = false;
};
export {disconnect, setup};
export default null;
