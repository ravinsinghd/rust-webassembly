(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg sync recursive":
/*!*******************!*\
  !*** ../pkg sync ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"../pkg sync recursive\";\n\n//# sourceURL=webpack:///../pkg_sync?");

/***/ }),

/***/ "../pkg/random_number.js":
/*!*******************************!*\
  !*** ../pkg/random_number.js ***!
  \*******************************/
/*! exports provided: greet, __wbg_new_c6fb156f56e56d5f, __wbg_call_f0cc87553c1e3b2a, __wbindgen_object_drop_ref, __wbindgen_jsval_eq, __wbg_self_4bcc93945365c19a, __wbg_require_10ff47d62350bb71, __wbg_crypto_67744919473fd26f, __wbindgen_is_undefined, __wbg_getRandomValues_cfbff5328b3ac59b, __wbg_getRandomValues_542f44b2df1d9c36, __wbg_randomFillSync_7c8b02669ad0870b */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_c6fb156f56e56d5f\", function() { return __wbg_new_c6fb156f56e56d5f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_f0cc87553c1e3b2a\", function() { return __wbg_call_f0cc87553c1e3b2a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_jsval_eq\", function() { return __wbindgen_jsval_eq; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_4bcc93945365c19a\", function() { return __wbg_self_4bcc93945365c19a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_10ff47d62350bb71\", function() { return __wbg_require_10ff47d62350bb71; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_67744919473fd26f\", function() { return __wbg_crypto_67744919473fd26f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_cfbff5328b3ac59b\", function() { return __wbg_getRandomValues_cfbff5328b3ac59b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_542f44b2df1d9c36\", function() { return __wbg_getRandomValues_542f44b2df1d9c36; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_7c8b02669ad0870b\", function() { return __wbg_randomFillSync_7c8b02669ad0870b; });\n/* harmony import */ var _random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./random_number_bg.wasm */ \"../pkg/random_number_bg.wasm\");\n\n\nlet cachegetInt32Memory = null;\nfunction getInt32Memory() {\n    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== _random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory = new Int32Array(_random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory;\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n/**\n* @returns {string}\n*/\nfunction greet() {\n    const retptr = 8;\n    const ret = _random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"greet\"](retptr);\n    const memi32 = getInt32Memory();\n    const v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();\n    _random_number_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);\n    return v0;\n}\n\nconst heap = new Array(32);\n\nheap.fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction getArrayU8FromWasm(ptr, len) {\n    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);\n}\n\nconst __wbg_new_c6fb156f56e56d5f = function(arg0, arg1) {\n    const ret = new Function(getStringFromWasm(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_call_f0cc87553c1e3b2a = function(arg0, arg1) {\n    const ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbindgen_jsval_eq = function(arg0, arg1) {\n    const ret = getObject(arg0) === getObject(arg1);\n    return ret;\n};\n\nconst __wbg_self_4bcc93945365c19a = function(arg0) {\n    const ret = getObject(arg0).self;\n    return addHeapObject(ret);\n};\n\nconst __wbg_require_10ff47d62350bb71 = function(arg0, arg1) {\n    const ret = __webpack_require__(\"../pkg sync recursive\")(getStringFromWasm(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_67744919473fd26f = function(arg0) {\n    const ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    const ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_getRandomValues_cfbff5328b3ac59b = function(arg0) {\n    const ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\nconst __wbg_getRandomValues_542f44b2df1d9c36 = function(arg0, arg1, arg2) {\n    getObject(arg0).getRandomValues(getArrayU8FromWasm(arg1, arg2));\n};\n\nconst __wbg_randomFillSync_7c8b02669ad0870b = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm(arg1, arg2));\n};\n\n\n\n//# sourceURL=webpack:///../pkg/random_number.js?");

/***/ }),

/***/ "../pkg/random_number_bg.wasm":
/*!************************************!*\
  !*** ../pkg/random_number_bg.wasm ***!
  \************************************/
/*! exports provided: memory, greet, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./random_number.js */ \"../pkg/random_number.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/random_number_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var random_number__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! random-number */ \"../pkg/random_number.js\");\n\r\n\r\n$(document).ready(function() {\r\n  $('.refresh-btn').click(function() {\r\n    resetValues();\r\n  });\r\n});\r\n\r\nfunction resetValues() {\r\n  let dataValue = random_number__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\r\n  $('.num1').text(dataValue);\r\n  dataValue = random_number__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\r\n  $('.num2').text(dataValue);\r\n  dataValue = random_number__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\r\n  $('.num3').text(dataValue);\r\n}\r\n\r\nresetValues();\r\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);