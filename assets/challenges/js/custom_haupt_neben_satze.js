class HauptNebenSatzeChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "haupt_neben_satze",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".custom-haupt-neben-satze__title"),
      mainClauses: document.getElementById("main-clauses"),
      subClauses: document.getElementById("sub-clauses"),
      checkButton: document.querySelector(
        ".custom-haupt-neben-satze__button--check",
      ),
      resetButton: document.querySelector(
        ".custom-haupt-neben-satze__button--reset",
      ),
      finishButton: document.querySelector(
        ".custom-haupt-neben-satze__button--finish",
      ),
      result: document.querySelector(".custom-haupt-neben-satze__result"),
      resultsContainer: document.querySelector(
        ".custom-haupt-neben-satze__results-container",
      ),
    };

    this.correctAnswers = 0;
  }

  translateStaticText() {
    if (this.elements.title) {
      this.elements.title.textContent = window.konnektoren.tr(
        "Haupt- und Nebensätze Übung",
      );
    }
    if (this.elements.checkButton) {
      this.elements.checkButton.textContent = window.konnektoren.tr("Check");
    }
    if (this.elements.resetButton) {
      this.elements.resetButton.textContent =
        window.konnektoren.tr("Try Again");
    }
    if (this.elements.finishButton) {
      this.elements.finishButton.textContent =
        window.konnektoren.tr("Finish Challenge");
    }
  }

  setupDragAndDrop() {
    let draggedItem = null;

    document.addEventListener("dragstart", (e) => {
      if (
        e.target.classList.contains(
          "custom-haupt-neben-satze__clause--draggable",
        )
      ) {
        draggedItem = e.target;
        e.dataTransfer.setData("text/plain", e.target.id);
        setTimeout(
          () =>
            e.target.classList.add(
              "custom-haupt-neben-satze__clause--dragging",
            ),
          0,
        );
      }
    });

    document.addEventListener("dragend", (e) => {
      if (
        e.target.classList.contains(
          "custom-haupt-neben-satze__clause--draggable",
        )
      ) {
        e.target.classList.remove("custom-haupt-neben-satze__clause--dragging");
        draggedItem = null;
      }
    });

    document.addEventListener("dragover", (e) => {
      if (
        e.target.classList.contains(
          "custom-haupt-neben-satze__clause--droppable",
        )
      ) {
        e.preventDefault();
      }
    });

    document.addEventListener("dragenter", (e) => {
      if (
        e.target.classList.contains(
          "custom-haupt-neben-satze__clause--droppable",
        )
      ) {
        e.preventDefault();
        e.target.classList.add("custom-haupt-neben-satze__clause--hovering");
      }
    });

    document.addEventListener("dragleave", (e) => {
      if (
        e.target.classList.contains(
          "custom-haupt-neben-satze__clause--droppable",
        )
      ) {
        e.target.classList.remove("custom-haupt-neben-satze__clause--hovering");
      }
    });

    document.addEventListener("drop", (e) => {
      if (
        e.target.classList.contains(
          "custom-haupt-neben-satze__clause--droppable",
        )
      ) {
        e.preventDefault();
        e.target.classList.remove("custom-haupt-neben-satze__clause--hovering");

        const droppedItemId = e.dataTransfer.getData("text/plain");
        const draggedElement = document.getElementById(droppedItemId);

        const existingDraggable = e.target.querySelector(
          ".custom-haupt-neben-satze__clause--draggable",
        );
        if (existingDraggable) {
          this.elements.subClauses.appendChild(existingDraggable);
        }

        e.target.appendChild(draggedElement);
      }
    });
  }

  generateClauses() {
    const mainClausesArray = this.data.get("main_clauses");
    const subClausesArray = this.data.get("sub_clauses");

    this.elements.mainClauses.innerHTML = "";
    this.elements.subClauses.innerHTML = "";

    mainClausesArray.forEach((mainClause) => {
      const li = document.createElement("li");
      li.classList.add(
        "custom-haupt-neben-satze__clause",
        "custom-haupt-neben-satze__clause--droppable",
      );
      li.id = mainClause.get("id");
      li.textContent = mainClause.get("text");
      this.elements.mainClauses.appendChild(li);
    });

    subClausesArray.forEach((subClause) => {
      const li = document.createElement("li");
      li.classList.add(
        "custom-haupt-neben-satze__clause",
        "custom-haupt-neben-satze__clause--draggable",
      );
      li.setAttribute("draggable", "true");
      li.id = subClause.get("id");
      li.textContent = subClause.get("text");
      this.elements.subClauses.appendChild(li);
    });
  }

  checkAnswer() {
    let isAllCorrect = true;
    this.correctAnswers = 0;
    this.state.userAnswers = [];

    const mainClausesArray = this.data.get("main_clauses");
    const correctPairs = this.data.get("correct_pairs");

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
        this.correctAnswers++;
        mainElement.classList.add("custom-haupt-neben-satze__clause--correct");
      } else {
        isAllCorrect = false;
        mainElement.classList.add(
          "custom-haupt-neben-satze__clause--incorrect",
        );
      }

      this.state.userAnswers.push({
        mainClause: mainClause.get("text"),
        userSubClause: subElement
          ? subElement.textContent
          : window.konnektoren.tr("No selection"),
        correctSubClause: this.data
          .get("sub_clauses")
          .find((sub) => sub.get("id") === correctSubId)
          .get("text"),
        isCorrect: isCorrect,
      });
    });

    if (isAllCorrect) {
      this.elements.result.textContent = window.konnektoren.tr("Correct!");
      this.elements.result.style.color = "green";
      this.finish();
    } else {
      this.elements.result.textContent = window.konnektoren.tr(
        "There are errors. Please check your answers.",
      );
      this.elements.result.style.color = "red";
      this.elements.resetButton.style.display = "inline-block";
    }

    this.elements.checkButton.disabled = true;
  }

  setupEventListeners() {
    if (this.elements.checkButton) {
      this.elements.checkButton.addEventListener("click", () =>
        this.checkAnswer(),
      );
    }
    if (this.elements.resetButton) {
      this.elements.resetButton.addEventListener("click", () =>
        this.generateClauses(),
      );
    }
    if (this.elements.finishButton) {
      this.elements.finishButton.addEventListener("click", () => this.finish());
    }
  }

  initialize() {
    super.initialize();
    this.translateStaticText();
    this.generateClauses();
    this.setupDragAndDrop();
    this.setupEventListeners();
  }

  displayResults(result) {
    if (!this.elements.resultsContainer) return;

    this.elements.resultsContainer.innerHTML = "";

    try {
      // Performance display
      const performanceElement = document.createElement("div");
      performanceElement.className = "custom-haupt-neben-satze__performance";
      const performance = result.performance * 100;
      performanceElement.innerHTML = `
              <h2 class="custom-haupt-neben-satze__performance-title">${window.konnektoren.tr("Your Performance")}</h2>
              <p class="custom-haupt-neben-satze__performance-score">${performance.toFixed(1)}%</p>
          `;
      this.elements.resultsContainer.appendChild(performanceElement);

      // Overview section
      const overviewElement = document.createElement("div");
      overviewElement.className = "custom-haupt-neben-satze__overview";

      console.log("result.data.get('answers')", result.data.get("answers"));

      const answers = Array.from(result.data.get("answers"));
      const correctCount = answers.filter((answer) =>
        answer.get("isCorrect"),
      ).length;
      const totalQuestions = answers.length;

      overviewElement.innerHTML = `
              <h3 class="custom-haupt-neben-satze__overview-title">${window.konnektoren.tr("Overview")}</h3>
              <div class="custom-haupt-neben-satze__overview-stats">
                  <p class="custom-haupt-neben-satze__stat custom-haupt-neben-satze__stat--correct">
                      ${window.konnektoren.tr("Correct Answers")}: ${correctCount}
                  </p>
                  <p class="custom-haupt-neben-satze__stat custom-haupt-neben-satze__stat--incorrect">
                      ${window.konnektoren.tr("Incorrect Answers")}: ${totalQuestions - correctCount}
                  </p>
                  <p class="custom-haupt-neben-satze__stat custom-haupt-neben-satze__stat--time">
                      ${window.konnektoren.tr("Time Spent")}: ${Math.round((result.data.get("timeSpent") || 0) / 1000)}s
                  </p>
              </div>
          `;
      this.elements.resultsContainer.appendChild(overviewElement);

      // Detailed results
      const detailedResults = document.createElement("div");
      detailedResults.className = "custom-haupt-neben-satze__detailed-results";
      detailedResults.innerHTML = `<h3 class="custom-haupt-neben-satze__detailed-title">${window.konnektoren.tr("Detailed Results")}</h3>`;

      const resultsList = document.createElement("ul");
      resultsList.className = "custom-haupt-neben-satze__results-list";

      answers.forEach((answer, index) => {
        const listItem = document.createElement("li");
        listItem.className = `custom-haupt-neben-satze__result-item ${
          answer.get("isCorrect") ? "correct" : "incorrect"
        }`;

        listItem.innerHTML = `
                  <div class="custom-haupt-neben-satze__result-header">
                      ${answer.get("isCorrect") ? "✅" : "❌"}
                      <span class="custom-haupt-neben-satze__result-number">
                          ${index + 1}/${totalQuestions}
                      </span>
                  </div>
                  <div class="custom-haupt-neben-satze__result-content">
                      <p class="custom-haupt-neben-satze__result-main">
                          <strong>${window.konnektoren.tr("Main Clause")}:</strong>
                          ${answer.get("mainClause")}
                      </p>
                      <p class="custom-haupt-neben-satze__result-user">
                          <strong>${window.konnektoren.tr("Your Answer")}:</strong>
                          ${answer.get("userSubClause")}
                      </p>
                      <p class="custom-haupt-neben-satze__result-correct">
                          <strong>${window.konnektoren.tr("Correct Answer")}:</strong>
                          ${answer.get("correctSubClause")}
                      </p>
                  </div>
              `;

        resultsList.appendChild(listItem);
      });

      detailedResults.appendChild(resultsList);
      this.elements.resultsContainer.appendChild(detailedResults);
    } catch (error) {
      console.error("Error displaying results:", error);
      this.elements.resultsContainer.innerHTML = `
              <div class="custom-haupt-neben-satze__error">
                  ${window.konnektoren.tr("An error occurred while displaying results.")}
              </div>
          `;
    }
  }

  calculatePerformance() {
    const answers = Array.from(this.state.userAnswers);
    const correctCount = answers.filter((answer) => answer.isCorrect).length;
    return correctCount / this.data.get("main_clauses").length;
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new HauptNebenSatzeChallenge({
    id: "custom_haupt_neben_satze",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".custom-haupt-neben-satze__results-container")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
