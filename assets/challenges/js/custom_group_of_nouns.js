class GroupOfNounsChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "group_of_nouns",
      task_ids: config.task_ids,
      data: config.data,
    });

    console.log("Tash ids", this.task_ids);

    this.elements = {
      title: document.querySelector(".custom_group_of_nouns-container__title"),
      endingDisplay: document.getElementById("ending-display"),
      choices: document.querySelector(
        ".custom_group_of_nouns-container__choices",
      ),
      feedback: document.getElementById("feedback"),
      correctAnswer: document.getElementById("correct-answer"),
      finishButton: document.querySelector(
        ".custom_group_of_nouns-container__finish-button",
      ),
      resultsContainer: document.querySelector(
        ".custom_group_of_nouns-results-container",
      ),
    };

    this.questions = this.filterQuestionsByTaskIds();
  }

  filterQuestionsByTaskIds() {
    const allEndings = this.data.get("questions");
    const taskIds = this.task_ids;

    if (!taskIds || !Array.isArray(taskIds)) {
      return allEndings; // Return all questions if no task_ids specified
    }

    // Filter endings based on task_ids
    return taskIds.map((id) => allEndings[id]).filter(Boolean);
  }

  translateStaticText() {
    if (this.elements.title) {
      this.elements.title.textContent = window.konnektoren.tr(
        "Identify the German Noun Gender",
      );
    }
    if (this.elements.finishButton) {
      this.elements.finishButton.textContent =
        window.konnektoren.tr("Finish Exercise");
    }
  }

  loadQuestion() {
    const currentEnding = this.questions[this.state.currentIndex];

    if (!currentEnding) {
      console.warn("No question found at index", this.state.currentIndex);
      return;
    }

    if (this.elements.endingDisplay) {
      this.elements.endingDisplay.textContent = `${window.konnektoren.tr("Noun ending:")} ${currentEnding.get("ending")}`;
      this.elements.feedback.textContent = "";
      this.elements.correctAnswer.textContent = "";
    }
  }

  checkAnswer(selectedArticle) {
    const currentEnding = this.questions[this.state.currentIndex];
    const correctArticle = currentEnding.get("article");
    const isCorrect = selectedArticle === correctArticle;

    if (isCorrect) {
      this.state.correctAnswers++;
      this.elements.feedback.textContent = window.konnektoren.tr("Correct!");
      document.body.classList.add("custom_group_of_nouns-container--green-bg");
    } else {
      this.elements.feedback.textContent = window.konnektoren.tr("Incorrect!");
      document.body.classList.add("custom_group_of_nouns-container--red-bg");
    }

    this.elements.correctAnswer.textContent = `${window.konnektoren.tr("The correct answer is:")} ${correctArticle}`;

    this.state.userAnswers.push({
      ending: currentEnding.get("ending"),
      selectedArticle: selectedArticle,
      correctArticle: correctArticle,
      isCorrect: isCorrect,
    });

    setTimeout(() => {
      document.body.classList.remove(
        "custom_group_of_nouns-container--green-bg",
        "custom_group_of_nouns-container--red-bg",
      );

      if (this.state.currentIndex >= this.questions.length - 1) {
        this.finish();
      } else {
        this.state.currentIndex++;
        this.loadQuestion();
      }
    }, 2000);
  }

  setupEventListeners() {
    const choiceItems = document.querySelectorAll(
      ".custom_group_of_nouns-container__choice-item",
    );
    choiceItems.forEach((item) => {
      item.addEventListener("click", () =>
        this.checkAnswer(item.dataset.article),
      );
    });

    if (this.elements.finishButton) {
      this.elements.finishButton.addEventListener("click", () => this.finish());
    }
  }

  calculatePerformance() {
    return this.state.correctAnswers / this.questions.length;
  }

  displayResults(result) {
    if (!this.elements.resultsContainer) return;

    this.elements.resultsContainer.innerHTML = "";

    try {
      // Performance display
      const performanceElement = document.createElement("h2");
      performanceElement.textContent = `${window.konnektoren.tr("Your Performance:")} ${(result.performance * 100).toFixed(2)}%`;
      this.elements.resultsContainer.appendChild(performanceElement);

      // Overview section
      const overviewElement = document.createElement("div");
      const answers = Array.from(result.data.get("answers"));
      const correctCount = answers.filter((answer) =>
        answer.get("isCorrect"),
      ).length;
      const totalQuestions = this.questions.length;

      overviewElement.innerHTML = `
        <h3>${window.konnektoren.tr("Overview")}</h3>
        <p>${window.konnektoren.tr("Correct Answers")}: ${correctCount}</p>
        <p>${window.konnektoren.tr("Incorrect Answers")}: ${totalQuestions - correctCount}</p>
        <p>${window.konnektoren.tr("Time Spent")}: ${Math.round((result.data.get("timeSpent") || 0) / 1000)}s</p>
      `;
      this.elements.resultsContainer.appendChild(overviewElement);

      // Detailed results
      const detailedResultsElement = document.createElement("div");
      detailedResultsElement.className =
        "custom_group_of_nouns-container__detailed-results";
      detailedResultsElement.innerHTML = `<h3>${window.konnektoren.tr("Detailed Results")}</h3>`;

      const resultsList = document.createElement("ul");
      resultsList.className = "custom_group_of_nouns-container__results-list";

      answers.forEach((answer) => {
        const listItem = document.createElement("li");
        listItem.className = "custom_group_of_nouns-container__result-item";

        const resultStatus = answer.get("isCorrect")
          ? '<span class="custom_group_of_nouns-container__result-correct">✅</span>'
          : '<span class="custom_group_of_nouns-container__result-incorrect">❌</span>';

        listItem.innerHTML = `
          <div class="custom_group_of_nouns-container__result-header">
            ${resultStatus}
            <span class="custom_group_of_nouns-container__result-ending">
              ${window.konnektoren.tr("Ending:")} ${answer.get("ending")}
            </span>
          </div>
          <div class="custom_group_of_nouns-container__result-details">
            <p class="custom_group_of_nouns-container__result-answer">
              <strong>${window.konnektoren.tr("Your Answer:")} </strong>
              <span class="${answer.get("isCorrect") ? "correct" : "incorrect"}">
                ${answer.get("selectedArticle")}
              </span>
            </p>
            <p class="custom_group_of_nouns-container__result-correct-answer">
              <strong>${window.konnektoren.tr("Correct Answer:")} </strong>
              ${answer.get("correctArticle")}
            </p>
          </div>
        `;

        resultsList.appendChild(listItem);
      });

      detailedResultsElement.appendChild(resultsList);
      this.elements.resultsContainer.appendChild(detailedResultsElement);

      // Add final score summary
      const summaryElement = document.createElement("div");
      summaryElement.className = "custom_group_of_nouns-container__summary";
      const percentageScore = ((correctCount / totalQuestions) * 100).toFixed(
        1,
      );
      summaryElement.innerHTML = `
        <h3>${window.konnektoren.tr("Final Score")}</h3>
        <p class="custom_group_of_nouns-container__score">
          ${percentageScore}%
          <span class="custom_group_of_nouns-container__score-detail">
            (${correctCount} ${window.konnektoren.tr("out of")} ${totalQuestions})
          </span>
        </p>
      `;

      this.elements.resultsContainer.appendChild(summaryElement);
    } catch (error) {
      console.error("Error displaying results:", error);

      // Display user-friendly error message
      const errorElement = document.createElement("div");
      errorElement.className = "custom_group_of_nouns-container__error";
      errorElement.textContent = window.konnektoren.tr(
        "An error occurred while displaying results.",
      );
      this.elements.resultsContainer.appendChild(errorElement);
    }
  }
}

function initializeChallenge() {
  const challenge = new GroupOfNounsChallenge({
    id: "custom_group_of_nouns",
    task_ids: window.konnektoren.challenge.task_ids,
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".custom_group_of_nouns-results-container")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Initialize the challenge
initializeChallenge();
