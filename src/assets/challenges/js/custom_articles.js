class ArticlesChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "articles",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".custom-articles-challenge__title"),
      questionText: document.querySelector(
        ".custom-articles-challenge__question-text",
      ),
      helpText: document.querySelector(".custom-articles-challenge__help-text"),
      questionImage: document.querySelector(
        ".custom-articles-challenge__question-image",
      ),
      optionsList: document.querySelector(
        ".custom-articles-challenge__options-list",
      ),
      feedback: document.querySelector(".custom-articles-challenge__feedback"),
      finishButton: document.querySelector(
        ".custom-articles-challenge__button--finish",
      ),
    };
  }

  translateStaticText() {
    if (this.elements.title) {
      this.elements.title.textContent = window.konnektoren.tr(
        "German Article Exercise",
      );
    }

    if (this.elements.finishButton) {
      this.elements.finishButton.textContent =
        window.konnektoren.tr("Finish Exercise");
    }
  }

  loadQuestion() {
    const questions = this.data.get("questions");
    if (!questions || !questions[this.state.currentIndex]) {
      console.warn("No question found at index", this.state.currentIndex);
      return;
    }

    const currentQuestion = questions[this.state.currentIndex];
    console.log("Loading question:", currentQuestion); // Debug log

    // Reset feedback
    this.elements.feedback.textContent = "";
    this.elements.feedback.style.display = "none";

    try {
      // Update question and help text
      this.elements.questionText.textContent = currentQuestion.get("question");
      this.elements.helpText.textContent = currentQuestion.get("help");

      // Update image
      if (currentQuestion.get("image")) {
        this.elements.questionImage.className =
          "custom-articles-challenge__question-image fas " +
          currentQuestion.get("image");
      } else {
        this.elements.questionImage.className =
          "custom-articles-challenge__question-image";
      }

      // Clear and create options
      this.elements.optionsList.innerHTML = "";

      const options = this.data.get("options");
      if (!options) {
        console.error("No options found in data");
        return;
      }

      options.forEach((option) => {
        const li = document.createElement("li");
        li.className = "custom-articles-challenge__option";
        li.textContent = option.get("name");
        li.dataset.id = option.get("id");
        li.addEventListener("click", () =>
          this.checkAnswer(parseInt(option.get("id"))),
        );
        this.elements.optionsList.appendChild(li);
      });
    } catch (error) {
      console.error("Error loading question:", error);
      console.log("Current question data:", currentQuestion);
      console.log("Current state:", this.state);
    }
  }

  checkAnswer(selectedOption) {
    const currentQuestion = this.data.get("questions")[this.state.currentIndex];
    const isCorrect = selectedOption === currentQuestion.get("option");

    if (isCorrect) {
      this.state.correctAnswers++;
      this.elements.feedback.textContent = window.konnektoren.tr("Correct!");
      this.elements.feedback.classList.add(
        "custom-articles-challenge__feedback--correct",
      );
      this.elements.feedback.classList.remove(
        "custom-articles-challenge__feedback--incorrect",
      );
    } else {
      this.elements.feedback.textContent = window.konnektoren.tr("Incorrect!");
      this.elements.feedback.classList.add(
        "custom-articles-challenge__feedback--incorrect",
      );
      this.elements.feedback.classList.remove(
        "custom-articles-challenge__feedback--correct",
      );
    }

    this.state.userAnswers.push({
      questionId: this.state.currentIndex,
      selectedOption: selectedOption,
      correctOption: currentQuestion.get("option"),
      isCorrect: isCorrect,
    });

    this.elements.feedback.style.display = "inline-block";

    if (this.state.currentIndex === this.data.get("questions").length - 1) {
      this.finish();
    } else {
      setTimeout(() => {
        this.elements.feedback.style.display = "none";
        this.state.currentIndex++;
        this.loadQuestion();
      }, 1000);
    }
  }

  displayResults(result) {
    if (!window.konnektoren.result) return;

    const performanceWrapper = document.querySelector(
      ".custom-articles-challenge__performance-wrapper",
    );
    const resultsContainer = document.querySelector(
      ".custom-articles-challenge__results-container",
    );

    if (!performanceWrapper || !resultsContainer) return;

    // Display performance
    const performance = result.performance * 100;
    performanceWrapper.innerHTML = `
            ${window.konnektoren.tr("Your performance")}:
            <span class="custom-articles-challenge__performance">${performance.toFixed(2)}%</span>
        `;

    // Clear previous results
    resultsContainer.innerHTML = "";

    let correctCount = 0;
    let incorrectCount = 0;

    // Create overview section
    const overviewSection = document.createElement("div");
    overviewSection.className = "custom-articles-challenge__overview";

    // Create detailed results list
    const resultsList = document.createElement("ul");
    resultsList.className = "custom-articles-challenge__results-list";

    // Extract answers, questions, and options
    const answers = Array.from(result.data.get("answers"));
    const questions = Array.from(this.data.get("questions"));
    const options = Array.from(this.data.get("options"));

    // Iterate through answers
    answers.forEach((answer, index) => {
      const question = questions[index];

      if (question) {
        const selectedOption = options.find(
          (o) => o.get("id") === answer.get("selectedOption"),
        );
        const correctOption = options.find(
          (o) => o.get("id") === question.get("option"),
        );

        if (answer.get("isCorrect")) {
          correctCount++;
        } else {
          incorrectCount++;
        }

        const listItem = document.createElement("li");
        listItem.className = "custom-articles-challenge__result-item";

        // Add question image if available
        let imageHtml = "";
        if (question.get("image")) {
          imageHtml = `<i class="custom-articles-challenge__result-image ${question.get("image")}"></i>`;
        }

        listItem.innerHTML = `
                    ${imageHtml}
                    <p class="custom-articles-challenge__result-question">
                        <strong>${window.konnektoren.tr("Question")}:</strong>
                        ${question.get("question")}
                    </p>
                    <p class="custom-articles-challenge__result-answer">
                        <strong>${window.konnektoren.tr("Your answer")}:</strong>
                        ${selectedOption ? selectedOption.get("name") : "Option not found"}
                        ${answer.get("isCorrect") ? "✅" : "❌"}
                    </p>
                    <p class="custom-articles-challenge__result-correct">
                        <strong>${window.konnektoren.tr("Correct answer")}:</strong>
                        ${correctOption ? correctOption.get("name") : "Option not found"}
                    </p>
                    <p class="custom-articles-challenge__result-help">
                        <strong>${window.konnektoren.tr("Help")}:</strong>
                        ${question.get("help")}
                    </p>
                `;

        resultsList.appendChild(listItem);
      }
    });

    // Add overview section
    overviewSection.innerHTML = `
            <h3 class="custom-articles-challenge__overview-title">
                ${window.konnektoren.tr("Overview")}
            </h3>
            <div class="custom-articles-challenge__overview-stats">
                <p class="custom-articles-challenge__overview-item custom-articles-challenge__overview-item--correct">
                    ${window.konnektoren.tr("Correct Answers")}: ${correctCount}
                </p>
                <p class="custom-articles-challenge__overview-item custom-articles-challenge__overview-item--incorrect">
                    ${window.konnektoren.tr("Incorrect Answers")}: ${incorrectCount}
                </p>
                <p class="custom-articles-challenge__overview-item custom-articles-challenge__overview-item--time">
                    ${window.konnektoren.tr("Time Spent")}: ${Math.round((result.data.get("timeSpent") || 0) / 1000)}s
                </p>
            </div>
        `;

    // Add everything to the results container
    resultsContainer.appendChild(overviewSection);
    resultsContainer.appendChild(resultsList);

    // Optional: Scroll to top of results
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

  setupEventListeners() {
    if (this.elements.finishButton) {
      this.elements.finishButton.addEventListener("click", () => {
        this.finish();
      });
    }
  }

  initialize() {
    // Call parent initialize
    super.initialize();

    // Do our specific initialization
    console.log("ArticlesChallenge initialization");
    this.translateStaticText();
    this.loadQuestion();
    window.konnektoren.setState(this.state);
  }
}

// Initialize the challenge
function initializeChallenge() {
  // Create ArticlesChallenge directly instead of using createChallenge
  const challenge = new ArticlesChallenge({
    id: "custom_articles",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".custom-articles-challenge__results-container")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
