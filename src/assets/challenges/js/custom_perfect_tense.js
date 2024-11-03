class PerfectTenseChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "perfect_tense",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".perfect-tense-challenge__title"),
      subtitle: document.querySelector(".perfect-tense-challenge__subtitle"),
      questionsContainer: document.querySelector(
        ".perfect-tense-challenge__questions",
      ),
      checkButton: document.querySelector(
        ".perfect-tense-challenge__button--check",
      ),
      resetButton: document.querySelector(
        ".perfect-tense-challenge__button--reset",
      ),
      feedback: document.querySelector(".perfect-tense-challenge__feedback"),
      results: document.querySelector(".perfect-tense-challenge__results"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent =
      window.konnektoren.tr("The Perfect Tense");
    this.elements.subtitle.textContent = window.konnektoren.tr(
      "Fill in the correct auxiliary verb for the perfect tense",
    );
    this.elements.checkButton.textContent = window.konnektoren.tr("Check");
    this.elements.resetButton.textContent = window.konnektoren.tr("Try Again");
  }

  loadQuestion() {
    const questions = Array.from(this.data.get("questions"));
    this.elements.questionsContainer.innerHTML = "";

    questions.forEach((question, index) => {
      const questionElement = document.createElement("div");
      questionElement.className = "perfect-tense-challenge__question";

      const sentence = question.get("sentence");
      const options = question.get("options");
      const pronoun = question.get("pronoun");

      questionElement.innerHTML = `
        <div class="perfect-tense-challenge__pronoun">${pronoun}</div>
        <label class="perfect-tense-challenge__label">
          ${sentence}
          <select class="perfect-tense-challenge__select" id="question-${index}">
            <option value="" disabled selected>${window.konnektoren.tr("Select")}</option>
            ${options
              .map((option) => `<option value="${option}">${option}</option>`)
              .join("")}
          </select>
        </label>
        <div class="perfect-tense-challenge__feedback" id="feedback-${index}"></div>
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
      const select = document.getElementById(`question-${index}`);
      const feedback = document.getElementById(`feedback-${index}`);
      const userAnswer = select.value;
      const correctAnswer = question.get("correct_answer");
      const isCorrect = userAnswer === correctAnswer;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
        feedback.textContent = window.konnektoren.tr("Correct!");
        feedback.className =
          "perfect-tense-challenge__feedback perfect-tense-challenge__feedback--correct";
        select.className =
          "perfect-tense-challenge__select perfect-tense-challenge__select--correct";
      } else {
        feedback.textContent = `${window.konnektoren.tr("Incorrect!")} ${window.konnektoren.tr("Correct answer")}: ${correctAnswer}`;
        feedback.className =
          "perfect-tense-challenge__feedback perfect-tense-challenge__feedback--incorrect";
        select.className =
          "perfect-tense-challenge__select perfect-tense-challenge__select--incorrect";
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
  const challenge = new PerfectTenseChallenge({
    id: "custom_perfect_tense",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".perfect-tense-challenge__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
