class VerbPrepositionsChallenge extends KonnektorenChallenge {
  constructor(config) {
    super({
      id: config.id,
      type: "verbs_prepositions",
      data: config.data,
    });

    this.elements = {
      title: document.querySelector(".verbs-prepositions__title"),
      description: document.querySelector(".verbs-prepositions__description"),
      questionsContainer: document.querySelector(
        ".verbs-prepositions__questions",
      ),
      checkButton: document.querySelector(".verbs-prepositions__button--check"),
      resetButton: document.querySelector(".verbs-prepositions__button--reset"),
      feedback: document.querySelector(".verbs-prepositions__feedback"),
    };
  }

  translateStaticText() {
    this.elements.title.textContent = window.konnektoren.tr(
      "Verbs with Prepositions",
    );
    this.elements.description.textContent = window.konnektoren.tr(
      "Select the correct preposition for each verb",
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
      questionElement.className = "verbs-prepositions__question";

      const sentence = question.get("sentence");
      const options = question.get("options");
      const verb = question.get("verb");

      questionElement.innerHTML = `
        <div class="verbs-prepositions__sentence">
          ${sentence.replace("__", this.createSelect(options, index))}
        </div>
        <div class="verbs-prepositions__verb-info">
          ${window.konnektoren.tr("Verb")}: <strong>${verb}</strong>
        </div>
        <div class="verbs-prepositions__feedback" id="feedback-${index}"></div>
      `;

      this.elements.questionsContainer.appendChild(questionElement);
    });
  }

  createSelect(options, questionIndex) {
    return `
      <select class="verbs-prepositions__select" id="question-${questionIndex}">
        <option value="">${window.konnektoren.tr("Select...")}</option>
        ${options
          .map(
            (option) => `
          <option value="${option}">${option}</option>
        `,
          )
          .join("")}
      </select>
    `;
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
      const explanation = question.get("explanation");
      const isCorrect = userAnswer === correctAnswer;

      if (!isCorrect) allCorrect = false;

      if (isCorrect) {
        this.state.correctAnswers++;
        select.className =
          "verbs-prepositions__select verbs-prepositions__select--correct";
        feedback.textContent = window.konnektoren.tr("Correct!");
        feedback.className =
          "verbs-prepositions__feedback verbs-prepositions__feedback--correct";
      } else {
        select.className =
          "verbs-prepositions__select verbs-prepositions__select--incorrect";
        feedback.innerHTML = `${window.konnektoren.tr("Incorrect!")}<br>${explanation}`;
        feedback.className =
          "verbs-prepositions__feedback verbs-prepositions__feedback--incorrect";
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
  const challenge = new VerbPrepositionsChallenge({
    id: "custom_verbs_prepositions",
    data: window.konnektoren.challenge.data,
  });

  if (document.querySelector(".verbs-prepositions__results")) {
    challenge.displayResults(window.konnektoren.result);
  } else {
    challenge.initialize();
  }
}

// Start the challenge
initializeChallenge();
