class VerbsChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "verbs",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".verbs-challenge__title"),
      verbName: document.querySelector(".verbs-challenge__verb-name"),
      verbTranslation: document.querySelector(
        ".verbs-challenge__verb-translation",
      ),
      questionsContainer: document.querySelector(".verbs-challenge__questions"),
      checkButton: document.querySelector(".verbs-challenge__button--check"),
      feedback: document.querySelector(".verbs-challenge__feedback"),
      results: document.querySelector(".verbs-challenge__results"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "German Verb Conjugation Exercise",
    );
    this.elements.checkButton.textContent =
      window.konnektoren.tr("Check Answers");
  }

  loadQuestion() {
    // Set verb information
    this.elements.verbName.textContent = this.data.get("verb");
    this.elements.verbTranslation.textContent =
      this.data.get("verb_translation");

    // Generate questions
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "verbs-challenge__question";

      const pronoun = question.get("pronoun");
      const verb = this.data.get("verb");

      questionElement.innerHTML = `
        <label class="verbs-challenge__label">
          ${pronoun}
          <input
            type="text"
            class="verbs-challenge__input"
            id="question-${index}"
            placeholder="${window.konnektoren.tr("Enter verb form")}"
          >
          (${verb})
        </label>
        <div class="verbs-challenge__feedback" id="feedback-${index}"></div>
      `;

      this.elements.questionsContainer.appendChild(questionElement);
    });
  }

  checkAnswer() {
    const questions = Array.from(this.data.get("questions"));
    let allCorrect = true;
    this.state.userAnswers = [];
    this.state.correctAnswers = 0;

    questions.forEach((question, index) => {
      const input = document.getElementById(`question-${index}`);
      const feedback = document.getElementById(`feedback-${index}`);
      const userAnswer = input.value.trim().toLowerCase();
      const correctAnswer = question.get("answer").toLowerCase();
      const isCorrect = userAnswer === correctAnswer;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
        input.className =
          "verbs-challenge__input verbs-challenge__input--correct";
        feedback.textContent = window.konnektoren.tr("Correct!");
        feedback.className =
          "verbs-challenge__feedback verbs-challenge__feedback--correct";
      } else {
        input.className =
          "verbs-challenge__input verbs-challenge__input--incorrect";
        feedback.textContent = `${window.konnektoren.tr("Incorrect!")} ${window.konnektoren.tr("Correct answer")}: ${correctAnswer}`;
        feedback.className =
          "verbs-challenge__feedback verbs-challenge__feedback--incorrect";
      }

      this.state.userAnswers.push({
        questionId: question.get("id"),
        userAnswer: userAnswer,
        correctAnswer: correctAnswer,
        isCorrect: isCorrect,
      });
    });

    if (allCorrect) {
      this.finish();
    }
  }

  setupEventListeners() {
    this.elements.checkButton.addEventListener("click", () =>
      this.checkAnswer(),
    );
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new VerbsChallenge({
    id: "custom_verbs",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".verbs-challenge__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
