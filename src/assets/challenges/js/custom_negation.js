class NegationChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "negation",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".negation-challenge__title"),
      questionsContainer: document.querySelector(
        ".negation-challenge__questions",
      ),
      checkButton: document.querySelector(".negation-challenge__button--check"),
      resetButton: document.querySelector(".negation-challenge__button--reset"),
      feedback: document.querySelector(".negation-challenge__feedback"),
      results: document.querySelector(".negation-challenge__results"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Negation in German: Kein vs. Nicht",
    );
    this.elements.checkButton.textContent = window.konnektoren.tr("Check");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "negation-challenge__question";

      const sentence = question.get("sentence");
      const translation = question.get("translation");
      const highlight = question.get("highlight");
      const example = question.get("example");

      questionElement.innerHTML = `
                <p class="negation-challenge__sentence">
                    ${sentence}
                    <span class="negation-challenge__translation">${translation}</span>
                </p>
                <input type="text"
                       class="negation-challenge__input"
                       id="question-${index}"
                       placeholder="${window.konnektoren.tr("Your Answer")}"
                />
                <div class="negation-challenge__example">
                    ${window.konnektoren.tr("Example")}:
                    <span class="negation-challenge__highlight">${highlight}</span>
                    (${example})
                </div>
                <div class="negation-challenge__feedback" id="feedback-${index}"></div>
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
      const correctAnswer = question.get("correct_answer").toLowerCase();
      const isCorrect = userAnswer === correctAnswer;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
      }

      this.state.userAnswers.push({
        questionId: question.get("id"),
        userAnswer: userAnswer,
        correctAnswer: correctAnswer,
        isCorrect: isCorrect,
      });

      feedback.textContent = isCorrect
        ? window.konnektoren.tr("Correct!")
        : `${window.konnektoren.tr("Incorrect!")} ${question.get("explanation")}`;
      feedback.className = `negation-challenge__feedback negation-challenge__feedback--${isCorrect ? "correct" : "incorrect"}`;
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
  const challenge = new NegationChallenge({
    id: "custom_negation",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".negation-challenge__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
