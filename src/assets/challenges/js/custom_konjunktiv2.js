class Konjunktiv2Challenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "konjunktiv2",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".custom-konjunktiv2-challenge__title"),
      progressBar: document.querySelector(
        ".custom-konjunktiv2-challenge__progress-bar",
      ),
      scenarioText: document.querySelector(
        ".custom-konjunktiv2-challenge__scenario-text",
      ),
      verb1Select: document.querySelector(
        ".custom-konjunktiv2-challenge__verb1",
      ),
      verb2Select: document.querySelector(
        ".custom-konjunktiv2-challenge__verb2",
      ),
      checkButton: document.querySelector(
        ".custom-konjunktiv2-challenge__button--check",
      ),
      resetButton: document.querySelector(
        ".custom-konjunktiv2-challenge__button--reset",
      ),
      finishButton: document.querySelector(
        ".custom-konjunktiv2-challenge__button--finish",
      ),
      feedback: document.querySelector(
        ".custom-konjunktiv2-challenge__feedback",
      ),
      hint: document.querySelector(".custom-konjunktiv2-challenge__hint"),
      results: document.querySelector(".custom-konjunktiv2-challenge__results"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Practice Subjunctive II",
    );
    this.elements.checkButton.textContent = window.konnektoren.tr("Check");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
    this.elements.finishButton.textContent =
      window.konnektoren.tr("Finish Challenge");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions")); // Convert Map to Array

    if (!questions || !questions[this.state.currentIndex]) {
      return;
    }

    const currentQuestion = questions[this.state.currentIndex];

    // Update scenario text
    this.elements.scenarioText.textContent = currentQuestion.get("scenario");

    // Update hint
    this.elements.hint.textContent = currentQuestion.get("hint");

    // Populate verb selects
    this.populateSelect(
      this.elements.verb1Select,
      currentQuestion.get("verb1"),
    );
    this.populateSelect(
      this.elements.verb2Select,
      currentQuestion.get("verb2"),
    );

    // Reset feedback
    this.elements.feedback.textContent = "";
    this.elements.feedback.className = "custom-konjunktiv2-challenge__feedback";

    // Update progress
    this.updateProgress();
  }

  populateSelect(select, options) {
    select.innerHTML = '<option value="">Select</option>';
    options.forEach((option) => {
      const optElement = document.createElement("option");
      optElement.value = option;
      optElement.textContent = option;
      select.appendChild(optElement);
    });
  }

  updateProgress() {
    const questions = Array.from(this.data.get("questions"));
    const progress = ((this.state.currentIndex + 1) / questions.length) * 100;
    this.elements.progressBar.style.width = `${progress}%`;
  }

  checkAnswer() {
    const questions = Array.from(this.data.get("questions"));

    console.log(questions, this.state.currentIndex);

    const currentQuestion = questions[this.state.currentIndex];
    const isLastQuestion = this.state.currentIndex === questions.length - 1;

    if (!currentQuestion) {
      console.error("No question found at index", this.state.currentIndex);
      return;
    }

    const verb1 = this.elements.verb1Select.value;
    const verb2 = this.elements.verb2Select.value;

    const correctAnswers = currentQuestion.get("correctAnswers");
    const isCorrect =
      verb1 === correctAnswers.get("verb1") &&
      verb2 === correctAnswers.get("verb2");

    this.state.userAnswers.push({
      scenario: currentQuestion.get("scenario"),
      verb1Selected: verb1,
      verb2Selected: verb2,
      verb1Correct: correctAnswers.get("verb1"),
      verb2Correct: correctAnswers.get("verb2"),
      isCorrect: isCorrect,
    });

    if (isCorrect) {
      this.state.correctAnswers++;
      this.elements.feedback.textContent = window.konnektoren.tr("Correct!");
      this.elements.feedback.classList.add("correct");

      if (!isLastQuestion) {
        setTimeout(() => {
          this.state.currentIndex++;
          this.loadQuestion();
        }, 1000);
      } else {
        // Last question was answered correctly
        setTimeout(() => {
          this.finish();
        }, 1000);
      }
    } else {
      this.elements.feedback.textContent =
        window.konnektoren.tr("There are errors");
      this.elements.feedback.classList.add("incorrect");
    }
  }

  setupEventListeners() {
    this.elements.checkButton.addEventListener("click", () =>
      this.checkAnswer(),
    );
    this.elements.resetButton.addEventListener("click", () =>
      this.loadQuestion(),
    );
    this.elements.finishButton.addEventListener("click", () => this.finish());

    // Check if both verbs are selected
    const checkBothSelected = () => {
      const verb1 = this.elements.verb1Select.value;
      const verb2 = this.elements.verb2Select.value;
      if (verb1 && verb2) {
        this.checkAnswer();
      }
    };

    this.elements.verb1Select.addEventListener("change", checkBothSelected);
    this.elements.verb2Select.addEventListener("change", checkBothSelected);
  }

  calculatePerformance() {
    const questions = Array.from(this.data.get("questions"));
    return this.state.correctAnswers / questions.length;
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new Konjunktiv2Challenge({
    id: "custom_konjunktiv2",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".results-container")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
