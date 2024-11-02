class GroupOfNounsChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "group_of_nouns",
      data: config.data,
    });

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
    const endings = this.data.get("questions");
    const currentEnding = endings[this.state.currentIndex];

    if (this.elements.endingDisplay) {
      this.elements.endingDisplay.textContent = `${window.konnektoren.tr("Noun ending:")} ${currentEnding.get("ending")}`;
      this.elements.feedback.textContent = "";
      this.elements.correctAnswer.textContent = "";
    }
  }

  checkAnswer(selectedArticle) {
    const endings = this.data.get("questions");
    const currentEnding = endings[this.state.currentIndex];
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

      if (this.state.currentIndex >= this.data.get("questions").length - 1) {
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
      const answers = result.data.get("answers");
      const correctCount = answers.filter((answer) =>
        answer.get("isCorrect"),
      ).length;
      const totalQuestions = this.data.get("questions").length;

      overviewElement.innerHTML = `
        <h3>${window.konnektoren.tr("Overview")}</h3>
        <p>${window.konnektoren.tr("Correct Answers")}: ${correctCount}</p>
        <p>${window.konnektoren.tr("Incorrect Answers")}: ${totalQuestions - correctCount}</p>
      `;
      this.elements.resultsContainer.appendChild(overviewElement);

      // Detailed results
      const detailedResultsElement = document.createElement("div");
      detailedResultsElement.innerHTML = `<h3>${window.konnektoren.tr("Detailed Results")}</h3>`;
      const resultsList = document.createElement("ul");

      answers.forEach((answer) => {
        const listItem = document.createElement("li");
        listItem.innerHTML = `
          <p>${window.konnektoren.tr("Ending:")} ${answer.get("ending")}</p>
          <p>${window.konnektoren.tr("Your Answer:")} ${answer.get("selectedArticle")}</p>
          <p>${window.konnektoren.tr("Correct Answer:")} ${answer.get("correctArticle")}</p>
          <p>${answer.get("isCorrect") ? "✅" : "❌"}</p>
        `;
        resultsList.appendChild(listItem);
      });

      detailedResultsElement.appendChild(resultsList);
      this.elements.resultsContainer.appendChild(detailedResultsElement);
    } catch (error) {
      console.error("Error displaying results:", error);
    }
  }
}

function initializeChallenge() {
  const challenge = new GroupOfNounsChallenge({
    id: "custom_group_of_nouns",
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
