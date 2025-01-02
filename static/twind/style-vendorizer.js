var i = new Map([
  ["align-self", "-ms-grid-row-align"],
  ["color-adjust", "-webkit-print-color-adjust"],
  ["column-gap", "grid-column-gap"],
  ["forced-color-adjust", "-ms-high-contrast-adjust"],
  ["gap", "grid-gap"],
  ["grid-template-columns", "-ms-grid-columns"],
  ["grid-template-rows", "-ms-grid-rows"],
  ["justify-self", "-ms-grid-column-align"],
  ["margin-inline-end", "-webkit-margin-end"],
  ["margin-inline-start", "-webkit-margin-start"],
  ["mask-border", "-webkit-mask-box-image"],
  ["mask-border-outset", "-webkit-mask-box-image-outset"],
  ["mask-border-slice", "-webkit-mask-box-image-slice"],
  ["mask-border-source", "-webkit-mask-box-image-source"],
  ["mask-border-repeat", "-webkit-mask-box-image-repeat"],
  ["mask-border-width", "-webkit-mask-box-image-width"],
  ["overflow-wrap", "word-wrap"],
  ["padding-inline-end", "-webkit-padding-end"],
  ["padding-inline-start", "-webkit-padding-start"],
  ["print-color-adjust", "color-adjust"],
  ["row-gap", "grid-row-gap"],
  ["scroll-margin-bottom", "scroll-snap-margin-bottom"],
  ["scroll-margin-left", "scroll-snap-margin-left"],
  ["scroll-margin-right", "scroll-snap-margin-right"],
  ["scroll-margin-top", "scroll-snap-margin-top"],
  ["scroll-margin", "scroll-snap-margin"],
  ["text-combine-upright", "-ms-text-combine-horizontal"],
]);
function r(r2) {
  return i.get(r2);
}
function a(i2) {
  var r2 =
    /^(?:(text-(?:decoration$|e|or|si)|back(?:ground-cl|d|f)|box-d|mask(?:$|-[ispro]|-cl)|pr|hyphena|flex-d)|(tab-|column(?!-s)|text-align-l)|(ap)|u|hy)/i.exec(
      i2
    );
  return r2 ? (r2[1] ? 1 : r2[2] ? 2 : r2[3] ? 3 : 5) : 0;
}
function t(i2, r2) {
  var a2 =
    /^(?:(pos)|(cli)|(background-i)|(flex(?:$|-b)|(?:max-|min-)?(?:block-s|inl|he|widt))|dis)/i.exec(
      i2
    );
  return a2
    ? a2[1]
      ? /^sti/i.test(r2)
        ? 1
        : 0
      : a2[2]
      ? /^pat/i.test(r2)
        ? 1
        : 0
      : a2[3]
      ? /^image-/i.test(r2)
        ? 1
        : 0
      : a2[4]
      ? r2[3] === "-"
        ? 2
        : 0
      : /^(?:inline-)?grid$/i.test(r2)
      ? 4
      : 0
    : 0;
}
export {
  r as cssPropertyAlias,
  a as cssPropertyPrefixFlags,
  t as cssValuePrefixFlags,
};
export default null;