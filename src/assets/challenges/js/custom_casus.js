class CasusChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "casus",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".custom-casus-challenge__title"),
      form: document.querySelector(".custom-casus-challenge__form"),
      optionsList: document.querySelector(
        ".custom-casus-challenge__options-list",
      ),
      submitButton: document.querySelector(
        ".custom-casus-challenge__submit-button",
      ),
      feedback: document.querySelector(".custom-casus-challenge__feedback"),
      resultsContainer: document.querySelector(
        ".custom-casus-challenge__results-container",
      ),
    };
  }

  translateStaticText() {
    if (this.elements.title) {
      this.elements.title.textContent = window.konnektoren.tr(
        "German Casus Exercise",
      );
    }
  }

  loadQuestion() {
    const currentQuestion = this.data.get("questions")[this.state.currentIndex];
    if (!this.elements.optionsList) return;

    // Reset feedback
    this.elements.feedback.textContent = "";
    this.elements.feedback.style.display = "none";

    // Clear previous question content
    this.elements.optionsList.innerHTML = "";

    // Create question text
    const questionText = document.createElement("p");
    questionText.className = "custom-casus-challenge__question-text";
    questionText.textContent = currentQuestion.get("question");
    this.elements.optionsList.appendChild(questionText);

    // Create options
    currentQuestion.get("options").forEach((option) => {
      const li = document.createElement("li");
      li.className = "custom-casus-challenge__option";
      li.innerHTML = `
                  <label class="custom-casus-challenge__option-label">
                      <input type="radio"
                             name="q${this.state.currentIndex}"
                             value="${option.toLowerCase()}"
                             class="custom-casus-challenge__option-input">
                      ${option}
                  </label>
              `;
      this.elements.optionsList.appendChild(li);
    });

    // Update submit button text
    if (this.elements.submitButton) {
      this.elements.submitButton.textContent =
        window.konnektoren.tr("Check Answer");
    }
  }

  checkAnswer(event) {
    event?.preventDefault();

    const currentQuestion = this.data.get("questions")[this.state.currentIndex];
    const selectedOption = document.querySelector(
      `input[name="q${this.state.currentIndex}"]:checked`,
    );

    if (!selectedOption) return;

    const isCorrect =
      selectedOption.value === currentQuestion.get("correct").toLowerCase();

    if (isCorrect) {
      this.state.correctAnswers++;
      this.elements.feedback.textContent = window.konnektoren.tr("Correct!");
      this.elements.feedback.classList.add(
        "custom-casus-challenge__feedback--correct",
      );
      this.elements.feedback.classList.remove(
        "custom-casus-challenge__feedback--incorrect",
      );
    } else {
      this.elements.feedback.textContent = window.konnektoren.tr("Incorrect!");
      this.elements.feedback.classList.add(
        "custom-casus-challenge__feedback--incorrect",
      );
      this.elements.feedback.classList.remove(
        "custom-casus-challenge__feedback--correct",
      );
    }

    this.state.userAnswers.push({
      questionId: this.state.currentIndex,
      selectedOption: selectedOption.value,
      correctOption: currentQuestion.get("correct"),
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

  setupEventListeners() {
    if (this.elements.form) {
      this.elements.form.addEventListener("submit", (e) => {
        e.preventDefault();
        this.checkAnswer();
      });
    }
  }

  displayResults(result) {
    if (!this.elements.resultsContainer) return;

    const answers = result.data.get("answers");
    const questions = this.data.get("questions");
    let correctCount = 0;
    let incorrectCount = 0;

    // Clear container
    this.elements.resultsContainer.innerHTML = "";

    // Add performance
    const performanceWrapper = document.createElement("div");
    performanceWrapper.className =
      "custom-casus-challenge__performance-wrapper";
    performanceWrapper.innerHTML = `
            ${window.konnektoren.tr("Your performance")}:
            <span class="custom-casus-challenge__performance">
                ${(result.performance * 100).toFixed(2)}%
            </span>
        `;
    this.elements.resultsContainer.appendChild(performanceWrapper);

    // Create results list
    const resultsList = document.createElement("ul");
    resultsList.className = "custom-casus-challenge__results-list";

    answers.forEach((answer, index) => {
      const question = questions[index];
      if (answer.get("isCorrect")) correctCount++;
      else incorrectCount++;

      const listItem = document.createElement("li");
      listItem.className = "custom-casus-challenge__result-item";
      listItem.innerHTML = `
                <p class="custom-casus-challenge__result-question">
                    <strong>${window.konnektoren.tr("Question")}:</strong>
                    ${question.get("question")}
                </p>
                <p class="custom-casus-challenge__result-answer">
                    <strong>${window.konnektoren.tr("Your answer")}:</strong>
                    ${answer.get("selectedOption")}
                    ${answer.get("isCorrect") ? "✅" : "❌"}
                </p>
                <p class="custom-casus-challenge__result-correct">
                    <strong>${window.konnektoren.tr("Correct answer")}:</strong>
                    ${question.get("correct")}
                </p>
            `;
      resultsList.appendChild(listItem);
    });

    // Add overview
    const overview = document.createElement("div");
    overview.className = "custom-casus-challenge__overview";
    overview.innerHTML = `
            <h3>${window.konnektoren.tr("Overview")}</h3>
            <p>${window.konnektoren.tr("Correct Answers")}: ${correctCount}</p>
            <p>${window.konnektoren.tr("Incorrect Answers")}: ${incorrectCount}</p>
        `;

    this.elements.resultsContainer.appendChild(overview);
    this.elements.resultsContainer.appendChild(resultsList);
  }
}

function initializeChallenge() {
  const challenge = new CasusChallenge({
    id: "custom_casus",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".custom-casus-challenge__results-container")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
