const data =
  window.konnektoren &&
  window.konnektoren.challenge &&
  window.konnektoren.challenge.data
    ? window.konnektoren.challenge.data
    : new Map([
        [
          "main_clauses",
          [
            new Map([
              ["id", "main-1"],
              ["text", "Ich gehe nach Hause,"],
            ]),
            new Map([
              ["id", "main-2"],
              ["text", "Es regnet,"],
            ]),
            new Map([
              ["id", "main-3"],
              ["text", "Er bleibt zu Hause,"],
            ]),
          ],
        ],
        [
          "sub_clauses",
          [
            new Map([
              ["id", "sub-1"],
              ["text", "weil es schon spät ist."],
            ]),
            new Map([
              ["id", "sub-2"],
              ["text", "weil er krank ist."],
            ]),
            new Map([
              ["id", "sub-3"],
              ["text", "obwohl die Sonne scheint."],
            ]),
          ],
        ],
        [
          "correct_pairs",
          new Map([
            ["main-1", "sub-1"],
            ["main-2", "sub-3"],
            ["main-3", "sub-2"],
          ]),
        ],
      ]);

let userAnswers = [];
let correctAnswers = 0;

// Function to translate text
function translate(key) {
  return window.konnektoren && window.konnektoren.tr
    ? window.konnektoren.tr(key)
    : key;
}

function setupDragAndDrop() {
  let draggedItem = null;

  document.addEventListener("dragstart", function (e) {
    if (
      e.target.classList.contains("custom-haupt-neben-satze__clause--draggable")
    ) {
      draggedItem = e.target;
      e.dataTransfer.setData("text/plain", e.target.id);
      setTimeout(
        () =>
          e.target.classList.add("custom-haupt-neben-satze__clause--dragging"),
        0,
      );
    }
  });

  document.addEventListener("dragend", function (e) {
    if (
      e.target.classList.contains("custom-haupt-neben-satze__clause--draggable")
    ) {
      e.target.classList.remove("custom-haupt-neben-satze__clause--dragging");
      draggedItem = null;
    }
  });

  document.addEventListener("dragover", function (e) {
    if (
      e.target.classList.contains("custom-haupt-neben-satze__clause--droppable")
    ) {
      e.preventDefault();
    }
  });

  document.addEventListener("dragenter", function (e) {
    if (
      e.target.classList.contains("custom-haupt-neben-satze__clause--droppable")
    ) {
      e.preventDefault();
      e.target.classList.add("custom-haupt-neben-satze__clause--hovering");
    }
  });

  document.addEventListener("dragleave", function (e) {
    if (
      e.target.classList.contains("custom-haupt-neben-satze__clause--droppable")
    ) {
      e.target.classList.remove("custom-haupt-neben-satze__clause--hovering");
    }
  });

  document.addEventListener("drop", function (e) {
    if (
      e.target.classList.contains("custom-haupt-neben-satze__clause--droppable")
    ) {
      e.preventDefault();
      e.target.classList.remove("custom-haupt-neben-satze__clause--hovering");

      const droppedItemId = e.dataTransfer.getData("text/plain");
      const draggedElement = document.getElementById(droppedItemId);

      // Remove existing draggable from droppable if any
      const existingDraggable = e.target.querySelector(
        ".custom-haupt-neben-satze__clause--draggable",
      );
      if (existingDraggable) {
        document.getElementById("sub-clauses").appendChild(existingDraggable);
      }

      // Append the dragged element to the droppable
      e.target.appendChild(draggedElement);
    }
  });
}

// Function to generate clauses dynamically
function generateClauses() {
  const mainClausesContainer = document.getElementById("main-clauses");
  const subClausesContainer = document.getElementById("sub-clauses");

  const mainClausesArray = data.get("main_clauses");
  const subClausesArray = data.get("sub_clauses");

  // Generate main clauses
  mainClausesArray.forEach((mainClause) => {
    const id = mainClause.get("id");
    const text = mainClause.get("text");

    const li = document.createElement("li");
    li.classList.add(
      "custom-haupt-neben-satze__clause",
      "custom-haupt-neben-satze__clause--droppable",
    );
    li.id = id;
    li.textContent = text;

    mainClausesContainer.appendChild(li);
  });

  // Generate subordinate clauses
  subClausesArray.forEach((subClause) => {
    const id = subClause.get("id");
    const text = subClause.get("text");

    const li = document.createElement("li");
    li.classList.add(
      "custom-haupt-neben-satze__clause",
      "custom-haupt-neben-satze__clause--draggable",
    );
    li.setAttribute("draggable", "true");
    li.id = id;
    li.textContent = text;

    subClausesContainer.appendChild(li);
  });
}

function checkAnswer() {
  let isAllCorrect = true;
  correctAnswers = 0; // Reset correctAnswers at the start of checking
  userAnswers = [];

  const mainClausesArray = data.get("main_clauses");
  const correctPairs = data.get("correct_pairs");

  mainClausesArray.forEach((mainClause) => {
    const mainId = mainClause.get("id");
    const mainElement = document.getElementById(mainId);
    const subElement = mainElement.querySelector(
      ".custom-haupt-neben-satze__clause--draggable",
    );

    mainElement.classList.remove(
      "custom-haupt-neben-satze__clause--correct",
      "custom-haupt-neben-satze__clause--incorrect",
    );

    const correctSubId = correctPairs.get(mainId);
    const isCorrect = subElement && subElement.id === correctSubId;

    if (isCorrect) {
      correctAnswers++; // Increment correctAnswers here
      mainElement.classList.add("custom-haupt-neben-satze__clause--correct");
    } else {
      isAllCorrect = false;
      mainElement.classList.add("custom-haupt-neben-satze__clause--incorrect");
    }

    userAnswers.push({
      mainClause: mainClause.get("text"),
      userSubClause: subElement
        ? subElement.textContent
        : translate("No selection"),
      correctSubClause: data
        .get("sub_clauses")
        .find((sub) => sub.get("id") === correctSubId)
        .get("text"),
      isCorrect: isCorrect,
    });
  });

  const result = document.querySelector(".custom-haupt-neben-satze__result");
  if (isAllCorrect) {
    result.textContent = translate("Correct!");
    result.style.color = "green";
    finishChallenge();
  } else {
    result.textContent = translate(
      "There are errors. Please check your answers.",
    );
    result.style.color = "red";
    document.querySelector(
      ".custom-haupt-neben-satze__button--reset",
    ).style.display = "inline-block";
  }

  document.querySelector(".custom-haupt-neben-satze__button--check").disabled =
    true;
}

// Finish function
function finishChallenge() {
  const mainClausesArray = data.get("main_clauses");
  const total = mainClausesArray.length;

  const performance = correctAnswers / total;

  // Prepare result data
  const resultData = {
    answers: userAnswers,
    performance: performance,
  };

  // Send the result event if possible
  if (window.konnektoren && window.konnektoren.sendEvent) {
    const event = {
      type: "Finish",
      result: {
        id: window.konnektoren.challenge.id,
        performance: performance,
        data: resultData,
      },
    };
    window.konnektoren.sendEvent(event);
  } else {
    // For testing purposes
    alert(`Ihre Leistung: ${(performance * 100).toFixed(2)}%`);
  }
}

function translateStaticText() {
  const title = document.querySelector(".custom-haupt-neben-satze__title");
  const checkButton = document.querySelector(
    ".custom-haupt-neben-satze__button--check",
  );
  const resetButton = document.querySelector(
    ".custom-haupt-neben-satze__button--reset",
  );
  const finishButton = document.querySelector(
    ".custom-haupt-neben-satze__button--finish",
  );
  if (title) {
    title.textContent = translate("Haupt- und Nebensätze Übung");
  }
  if (checkButton) {
    checkButton.textContent = translate("Check");
  }
  if (resetButton) {
    resetButton.textContent = translate("Try Again");
  }
  if (finishButton) {
    finishButton.textContent = translate("Finish Challenge");
  }
}

function displayResults(resultData) {
  let resultsContainer = document.querySelector(
    ".custom-haupt-neben-satze__results-container",
  );

  // If the results container is not found, try to find elements from the results page
  if (!resultsContainer) {
    resultsContainer = document.querySelector(
      ".custom-haupt-neben-satze--results",
    );
    if (!resultsContainer) {
      console.warn("Results container not found.");
      return;
    }
  }

  resultsContainer.innerHTML = "";

  const performance = resultData.get("performance");
  const answers = resultData.get("answers");

  if (!answers || !Array.isArray(answers)) {
    console.warn("Invalid answers data.");
    return;
  }

  try {
    const performanceElement = document.createElement("h2");
    performanceElement.className =
      "custom-haupt-neben-satze__performance-score";
    performanceElement.textContent = `${translate("Your Performance")}: ${(performance * 100).toFixed(2)}%`;
    resultsContainer.appendChild(performanceElement);

    const overviewElement = document.createElement("div");
    overviewElement.className = "custom-haupt-neben-satze__overview";
    const correctAnswers = answers.filter((answer) =>
      answer.get("isCorrect"),
    ).length;
    const totalQuestions = answers.length;

    overviewElement.innerHTML = `
      <h3>${translate("Overview")}</h3>
      <p>${translate("Correct Answers")}: ${correctAnswers}</p>
      <p>${translate("Incorrect Answers")}: ${totalQuestions - correctAnswers}</p>
    `;
    resultsContainer.appendChild(overviewElement);

    const detailedResultsElement = document.createElement("div");
    detailedResultsElement.className = "custom-haupt-neben-satze__results";
    detailedResultsElement.innerHTML = `<h3>${translate("Detailed Results")}</h3>`;
    const resultsList = document.createElement("ul");
    resultsList.className = "custom-haupt-neben-satze__results-list";

    answers.forEach((answer, index) => {
      const listItem = document.createElement("li");
      listItem.className = "custom-haupt-neben-satze__result-item";
      listItem.innerHTML = `
        <p><strong>${translate("Question")} ${index + 1}</strong></p>
        <p>${translate("Main Clause")}: ${answer.get("mainClause")}</p>
        <p>${translate("Your Subordinate Clause")}: ${answer.get("userSubClause")}</p>
        <p>${translate("Correct Subordinate Clause")}: ${answer.get("correctSubClause")}</p>
        <p>${answer.get("isCorrect") ? "✅" : "❌"}</p>
      `;
      resultsList.appendChild(listItem);
    });

    detailedResultsElement.appendChild(resultsList);
    resultsContainer.appendChild(detailedResultsElement);
  } catch (error) {
    console.error("Error displaying results:", error);
  }
}

// Add this function to handle finishing the challenge at any time
function finishChallengeEarly() {
  checkAnswer(); // Call checkAnswer to update correctAnswers and userAnswers

  const mainClausesArray = data.get("main_clauses");
  const performance = correctAnswers / mainClausesArray.length;
  const resultData = {
    answers: userAnswers,
    performance,
  };

  if (window.konnektoren && window.konnektoren.sendEvent) {
    const event = {
      type: "Finish",
      result: {
        id: window.konnektoren.challenge.id,
        performance: performance,
        data: resultData,
      },
    };
    window.konnektoren.sendEvent(event);
  } else {
    console.log("Challenge finished early. Performance:", performance);
    console.log("User Answers:", userAnswers);
    displayResults(resultData);
  }
}

function init() {
  if (!window.konnektoren.result) {
    translateStaticText();
    generateClauses();
    setupDragAndDrop();

    const checkBtn = document.querySelector(
      ".custom-haupt-neben-satze__button--check",
    );
    const resetBtn = document.querySelector(
      ".custom-haupt-neben-satze__button--reset",
    );
    const finishBtn = document.querySelector(
      ".custom-haupt-neben-satze__button--finish",
    );

    if (checkBtn) checkBtn.addEventListener("click", checkAnswer);
    if (resetBtn) {
      resetBtn.addEventListener("click", function () {
        location.reload(); // Simple reset by reloading the page
      });
    }
    if (finishBtn) finishBtn.addEventListener("click", finishChallengeEarly);
  } else {
    const result = window.konnektoren && window.konnektoren.result;
    if (result) {
      console.log(result);
      const resultData = new Map([
        ["answers", result.data.get("answers")],
        ["performance", result.performance],
      ]);
      displayResults(resultData);
    } else {
      console.warn("No result data available.");
    }
  }
}

// Initialize the challenge
init();
