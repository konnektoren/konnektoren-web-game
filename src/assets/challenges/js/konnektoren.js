// This file provides dummy implementations of Konnektoren functions for development.
// On runtime, the Rust code will inject a WASM code that provides the real functions.

window.konnektoren = window.konnektoren || {};

// Challenge Data
window.konnektoren.challenge = window.konnektoren.challenge || {};

//  Challenge data
window.konnektoren.challenge.data = window.konnektoren.challenge.data || {};

//  Result data
window.konnektoren.result = window.konnektoren.result || {};

// Translation function
window.konnektoren.tr =
  window.konnektoren.tr ||
  function (text) {
    console.log("Translation requested:", text);
    return text; // Just return the text for now
  };

// Send event
window.konnektoren.sendEvent =
  window.konnektoren.sendEvent ||
  function (event) {
    console.log("Event received:", event);
  };

// Execute command
window.konnektoren.executeCommand =
  window.konnektoren.executeCommand ||
  function (command) {
    console.log("Command received:", command);
  };
