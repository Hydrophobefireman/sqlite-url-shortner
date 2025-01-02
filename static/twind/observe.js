import "./style-vendorizer.js";

import {tw} from "./twind.js";

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
  setup,
  silent,
  strict,
  theme,
  tw,
  voidSheet,
  warn,
} from "./twind.js";
var ensureMaxSize = (map, max) => {
  if (map.size > max) {
    map.delete(map.keys().next().value);
  }
};
var caches = new WeakMap();
var getCache = (tw3) => {
  let rulesToClassCache = caches.get(tw3);
  if (!rulesToClassCache) {
    rulesToClassCache = new Map();
    caches.set(tw3, rulesToClassCache);
  }
  return rulesToClassCache;
};
var uniq = (value, index, values) => values.indexOf(value) == index;
var createObserver = ({tw: tw$1 = tw} = {}) => {
  if (typeof MutationObserver == "function") {
    const rulesToClassCache = getCache(tw$1);
    const handleMutation = ({target, addedNodes}) => {
      var _a;
      const rules =
        (_a = target.getAttribute) == null ? void 0 : _a.call(target, "class");
      if (rules) {
        let className = rulesToClassCache.get(rules);
        if (!className) {
          className = tw$1(rules).split(/ +/g).filter(uniq).join(" ");
          rulesToClassCache.set(rules, className);
          rulesToClassCache.set(className, className);
          ensureMaxSize(rulesToClassCache, 3e4);
        }
        if (rules !== className) {
          target.setAttribute("class", className);
        }
      }
      for (let index = addedNodes.length; index--; ) {
        const node = addedNodes[index];
        handleMutations([
          {
            target: node,
            addedNodes: node.children || [],
          },
        ]);
      }
    };
    const handleMutations = (mutations) => {
      mutations.forEach(handleMutation);
      mutations = observer.takeRecords();
      if (mutations) mutations.forEach(handleMutation);
    };
    const observer = new MutationObserver(handleMutations);
    return {
      observe(target) {
        handleMutations([{target, addedNodes: [target]}]);
        observer.observe(target, {
          attributes: true,
          attributeFilter: ["class"],
          subtree: true,
          childList: true,
        });
        return this;
      },
      disconnect() {
        observer.disconnect();
        return this;
      },
    };
  }
  return {
    observe() {
      return this;
    },
    disconnect() {
      return this;
    },
  };
};
function observe(target, config = typeof this == "function" ? void 0 : this) {
  return createObserver(config).observe(target);
}
export {createObserver, observe};
export default null;
