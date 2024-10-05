function checkAnswers() {
  // Get selected values
  let wartenPrep = document.getElementById("warten-prep").value;
  let wartenCase = document.getElementById("warten-case").value;

  let freuenPrep = document.getElementById("freuen-prep").value;
  let freuenCase = document.getElementById("freuen-case").value;

  // Correct answers
  let correctWarten = wartenPrep === "auf" && wartenCase === "Akkusativ";
  let correctFreuen = freuenPrep === "auf" && freuenCase === "Akkusativ";

  // Display results
  let result = document.getElementById("result");
  if (correctWarten && correctFreuen) {
    result.textContent = "🎉 Super! Alle Antworten sind richtig.";
    result.style.color = "green";
  } else {
    result.textContent =
      "❌ Leider sind nicht alle Antworten richtig. Bitte versuchen Sie es erneut.";
    result.style.color = "red";
  }
}

// Function to show theory for each verb
function showTheory(id) {
  let theoryText = document.getElementById(id);
  if (theoryText.style.display === "block") {
    theoryText.style.display = "none";
  } else {
    theoryText.style.display = "block";
  }
}

// Attach event listeners after the DOM content is loaded
function init() {
  // Attach event listener to the submit button
  document.getElementById("submit-btn").addEventListener("click", checkAnswers);

  // Attach event listeners to the theory buttons
  document
    .getElementById("warten-theory-btn")
    .addEventListener("click", function () {
      showTheory("warten-theory");
    });

  document
    .getElementById("freuen-theory-btn")
    .addEventListener("click", function () {
      showTheory("freuen-theory");
    });
}

init();
