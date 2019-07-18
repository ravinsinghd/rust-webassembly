import * as wasm from 'random-number';

$(document).ready(function() {
  $('.refresh-btn').click(function() {
    resetValues();
  });
});

function resetValues() {
  let dataValue = wasm.greet();
  $('.num1').text(dataValue);
  dataValue = wasm.greet();
  $('.num2').text(dataValue);
  dataValue = wasm.greet();
  $('.num3').text(dataValue);
}

resetValues();
