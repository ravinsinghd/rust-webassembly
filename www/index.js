import * as wasm from 'random-number';

$(document).ready(function () {
  $('.refresh-btn').click(function () {
    resetValues();
  });
  $('.convert-ascii').click(function () {
    convertAscii();
  });
});

function resetValues() {
  let dataValue = wasm.greet(3);
  $('.num1').text(dataValue[0]);
  $('.num2').text(dataValue[1]);
  $('.num3').text(dataValue[2]);
  convertAscii();
}

function convertAscii() {
  const asciiVal = $('#asciiInput').val();
  let convertedData = wasm.convert_ascii_to_unicode(asciiVal);
  $('#unicodeOutput').val(convertedData);
}

resetValues();
