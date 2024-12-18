class VerbsZuChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "verbs_zu",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".verbs-zu__title"),
      description: document.querySelector(".verbs-zu__description"),
      questionsContainer: document.querySelector(".verbs-zu__questions"),
      checkButton: document.querySelector(".verbs-zu__button--check"),
      resetButton: document.querySelector(".verbs-zu__button--reset"),
      feedback: document.querySelector(".verbs-zu__feedback"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Modal Verbs & Brauchen + zu",
    );
    this.elements.description.textContent = window.konnektoren.tr(
      "Fill in the blanks with the correct form of a modal verb or 'brauchen + zu'",
    );
    this.elements.checkButton.textContent =
      window.konnektoren.tr("Check Answers");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "verbs-zu__question";

      questionElement.innerHTML = `
        <p class="verbs-zu__sentence">${question.get("sentence")}</p>
        <input
          type="text"
          class="verbs-zu__input"
          id="question-${index}"
          placeholder="${window.konnektoren.tr("Enter the verb form")}"
        >
        <div class="verbs-zu__feedback" id="feedback-${index}"></div>
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
      const userAnswer = input.value.trim();
      const correctAnswer = question.get("correct_answer");
      const isCorrect = userAnswer === correctAnswer;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
        input.className = "verbs-zu__input verbs-zu__input--correct";
        feedback.textContent = question.get("explanation").get("correct");
        feedback.className = "verbs-zu__feedback verbs-zu__feedback--correct";
      } else {
        input.className = "verbs-zu__input verbs-zu__input--incorrect";
        feedback.textContent = question.get("explanation").get("wrong");
        feedback.className = "verbs-zu__feedback verbs-zu__feedback--incorrect";
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
    this.elements.resetButton.addEventListener("click", () =>
      this.loadQuestion(),
    );
  }
}

// Initialize the challenge
function initializeChallenge() {
  const challenge = new VerbsZuChallenge({
    id: "custom_verbs_zu",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".verbs-zu__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
